//! Contains OBS, the primary struct for interacting with the OBS WebSocket server.

mod websocket_stream;
use websocket_stream::WebSocketStream;

use crate::{error::ObsError, events, requests::*, responses};

use base64;
use futures::{
    channel::{
        mpsc::{self, Receiver, Sender},
        oneshot::{channel as oneshot_channel, Sender as OneshotSender},
    },
    executor, future,
    stream::{self, StreamExt, TryStreamExt},
};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    net::{TcpStream, ToSocketAddrs},
    thread::{self, JoinHandle},
    time::Duration,
};
use tungstenite::{self, protocol::Role, Message as WebSocketMessage, WebSocket};

#[derive(Default)]
pub struct Obs {
    connection_data: Option<ConnectionData>,
}

impl Obs {
    pub fn new() -> Self {
        Obs::default()
    }

    /// Attempts to connect to OBS.
    pub fn connect(&mut self, address: &str, port: u16) -> Result<(), ObsError> {
        log::debug!("connecting to {}:{}", address, port);
        let (thread_sender, thread_receiver) = mpsc::channel(2048);
        let (event_sender, event_receiver) = mpsc::channel(2048);
        let (websocket_stream, send_socket, close_socket) = Obs::init_sockets(address, port)?;
        let handle =
            Obs::start_handler(send_socket, thread_receiver, websocket_stream, event_sender);

        self.connection_data = Some(ConnectionData {
            socket_handle: close_socket,
            thread_handle: handle,
            thread_sender,
            event_receiver,
        });
        Ok(())
    }

    /// Closes the connection to OBS (if any).
    pub fn close(&mut self) {
        if let Some(ConnectionData {
            mut thread_sender,
            mut socket_handle,
            thread_handle,
            mut event_receiver,
        }) = self.connection_data.take()
        {
            log::info!("closing connection");
            thread_sender.close_channel();
            socket_handle
                .close(None)
                .expect("failed to close socket handle");
            thread_handle.join().expect("failed to join thread handle");
            event_receiver.close();
        } else {
            log::info!("not connected");
        }
    }

    /// Sends the given request to OBS and blocks until a response has been received.
    // TODO: async?
    pub fn request<T>(&mut self, req: &T) -> Result<T::Output, ObsError>
    where
        T: Request + std::fmt::Debug,
    {
        if let Some(ConnectionData { thread_sender, .. }) = self.connection_data.as_mut() {
            log::debug!("requesting {:#?}", req);
            let value = req.to_json();
            log::trace!("converted request to json {}", value);
            let (oneshot_sender, oneshot_receiver) = oneshot_channel();
            let message = Message::Outgoing {
                message_id: req.message_id().to_string(),
                value,
                sender: oneshot_sender,
            };
            log::trace!("sending");
            if thread_sender.try_send(message).is_err() {
                // failed to connect to thread, close connection
                self.close();
                return Err(ObsError::ConnectionInterrupted);
            }
            log::trace!("sent");
            let res = executor::block_on(oneshot_receiver);
            match res {
                // received something from channel
                Ok(res) => match res {
                    Ok(res) => {
                        log::debug!("received response {}", res);
                        Ok(serde_json::from_str(&res)?)
                    }
                    Err(res) => {
                        log::error!("received error {:#?}", res);
                        Err(ObsError::ObsError(
                            res.error
                                .expect("error from sender should have error message"),
                        ))
                    }
                },
                Err(canceled) => {
                    log::info!("channel to handler closed: {}", canceled);
                    Err(ObsError::OneshotCanceled(canceled))
                }
            }
        } else {
            Err(ObsError::NotConnected)
        }
    }

    /// Tries to authenticate with OBS. Returns an error if no authentication is required.
    pub fn authenticate(&mut self, password: &str) -> Result<responses::Empty, ObsError> {
        let auth = self.request(&GetAuthRequired::builder().build())?;
        if auth.auth_required {
            log::info!("auth required");
            let challenge = auth.challenge.expect("should have challenge");
            let salt = auth.salt.expect("should have salt");

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            log::info!("authing");
            let req = Authenticate::builder().auth(&auth_response).build();
            Ok(self.request(&req)?)
        } else {
            Err(ObsError::NoAuthRequired)
        }
    }

    pub fn check_event(&mut self) -> Option<events::Event> {
        if let Some(data) = self.connection_data.as_mut() {
            data.event_receiver.next();
            todo!()
        } else {
            None
        }
    }

    // initializes connection data
    fn init_sockets(
        address: &str,
        port: u16,
    ) -> Result<(WebSocketStream, WebSocket<TcpStream>, WebSocket<TcpStream>), ObsError> {
        let addr = format!("{}:{}", address, port);
        let ws_addr = format!("ws://{}", addr);
        let recv_stream = TcpStream::connect_timeout(
            &addr.to_socket_addrs()?.next().expect("no addresses parsed"),
            Duration::from_millis(100),
        )?;
        recv_stream
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        let send_stream = recv_stream.try_clone()?;
        let close_stream = recv_stream.try_clone()?;
        let (recv_socket, _res) = tungstenite::client(ws_addr, recv_stream)?;
        close_stream.set_nonblocking(true)?;

        let recv_socket_iter = WebSocketStream(recv_socket);
        let send_socket = WebSocket::from_raw_socket(send_stream, Role::Client, None);
        let close_socket = WebSocket::from_raw_socket(close_stream, Role::Client, None);
        Ok((recv_socket_iter, send_socket, close_socket))
    }

    // starts the handler thread
    fn start_handler(
        mut send_socket: WebSocket<TcpStream>,
        outgoing_receiver: Receiver<Message>,
        websocket_stream: WebSocketStream,
        mut event_sender: Sender<events::Event>,
    ) -> JoinHandle<()> {
        log::debug!("starting handler");
        thread::Builder::new().name("handler".to_string()).spawn(move || {
            // map to result to make compatible with ws stream
            let outgoing_receiver_adapted = outgoing_receiver.map(|m| Ok(m));

            // combine streams for outgoing (JSON from user) and incoming (WS from OBS) messages to thread
            let streams = stream::select(websocket_stream, outgoing_receiver_adapted);
            let mut pending_senders = HashMap::new();

            let fut = streams.try_for_each(|message| {
                log::trace!("received message");
                match message {
                    Message::Close => {}
                    Message::Outgoing { message_id, value, sender } => {
                        log::trace!("received outgoing message");
                        send_socket
                            .write_message(WebSocketMessage::text(value.to_string()))
                            .expect("failed to write message");
                        log::debug!("sent text {}", value);
                        pending_senders.insert(message_id, sender);
                    }
                    Message::Incoming(message) => match message {
                        WebSocketMessage::Text(text) => {
                            log::debug!("received text {}", text);
                            let parsed = serde_json::from_str::<ResponseOrEvent>(&text);
                            match parsed {
                                Ok(ResponseOrEvent::Response(response)) => {
                                    if let Some(sender) = pending_senders.remove(&response.message_id) {
                                        log::trace!("received response {:#?}", response);
                                        if let Some(error) = &response.error {
                                            log::error!("error: {}", error);
                                            sender.send(Err(response)).expect("failed to send");
                                        } else {
                                            sender.send(Ok(text)).expect("failed to send");
                                        }
                                    } else {
                                        log::warn!("unexpected response");
                                    }
                                }
                                Ok(ResponseOrEvent::Event(event)) => {
                                    log::info!("received event {:#?}", event);
                                    if event_sender.try_send(event).is_err() {
                                        log::error!("failed to send event");
                                    };
                                }
                                Err(e) => log::error!("received invalid text: {} which failed to deserialize with {:#?}", text, e),
                            }
                        }
                        unexpected => {
                            log::warn!("unexpected websocket message: {:?}", unexpected);
                        }
                    },
                }
                future::ok(())
            });
            let res = executor::block_on(fut);
            log::info!("res {:?}", res);
            log::info!("receivers closed");
        }).expect("failed to create thread")
    }
}

// message from the WebSocket server
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum ResponseOrEvent {
    Response(responses::Response),
    Event(events::Event),
}

// message used to communicate with the handler channel that owns the WebSocket connection
#[derive(Debug)]
enum Message {
    // message id, JSON to be sent, and oneshot sender to send the result back with
    Outgoing {
        message_id: String,
        value: Value,
        sender: OneshotSender<Result<String, responses::Response>>,
    },
    Incoming(WebSocketMessage),
    Close,
}

// container for data related to the WebSocket connection
struct ConnectionData {
    socket_handle: WebSocket<TcpStream>,
    thread_handle: JoinHandle<()>,
    thread_sender: Sender<Message>,
    event_receiver: Receiver<events::Event>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{json, Value};
    use std::{
        net::TcpListener,
        thread::{spawn, JoinHandle},
    };
    use tungstenite::server::accept;

    fn response_data() -> responses::Response {
        responses::Response {
            message_id: "".to_string(),
            status: responses::Status::Ok,
            error: None,
        }
    }

    fn init_without_server(port: u16) -> Obs {
        log::debug!("initiating without server at {}", port);
        let mut obs = Obs::new();
        obs.connect("localhost", port).expect("failed to connect");
        obs
    }

    fn init(responses: Vec<Value>) -> (Obs, JoinHandle<Vec<Value>>) {
        let server = TcpListener::bind("localhost:0").expect("failed to bind");
        let port = server.local_addr().expect("local addr").port();
        log::info!("mock server started at {}", port);
        let handle = spawn(move || {
            let mut actual_requests = vec![];
            let (stream, _) = server.accept().expect("accept");
            log::info!("incoming connection");
            let mut websocket = accept(stream).expect("failed to accept");
            for response in responses {
                let message = websocket.read_message().expect("failed to read message");
                log::info!("read message {:#?}", message);
                let parsed = serde_json::from_str::<Value>(&message.to_string())
                    .expect("failed to deserialize");
                actual_requests.push(parsed);
                log::info!("responding with {:#?}", response);
                websocket
                    .write_message(WebSocketMessage::Text(response.to_string()))
                    .expect("failed to write");
            }
            log::info!("closing mock server");
            websocket.close(None).expect("failed to close");
            actual_requests
        });
        let obs = init_without_server(port);
        (obs, handle)
    }

    fn request_test<T>(requests: Vec<Value>, responses: Vec<Value>, request: T, expected: T::Output)
    where
        T: Request + PartialEq + std::fmt::Debug,
        T::Output: PartialEq + std::fmt::Debug,
    {
        let _ = env_logger::builder().is_test(true).try_init();
        let (mut obs, handle) = init(responses);
        let res = obs.request(&request).expect("request returned err");
        let actual_requests = handle.join().expect("failed to join");
        obs.close();
        for (request, actual_request) in requests.into_iter().zip(actual_requests) {
            assert_eq!(
                request, actual_request,
                "request (left) did not match expected (right)"
            );
        }
        assert_eq!(
            res, expected,
            "result (left) did not match expected (right)"
        );
    }

    #[test]
    fn get_version() {
        let request = json!({
            "request-type": "GetVersion",
            "message-id": "",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "version": 1.1,
            "obs-websocket-version": "4.7.0",
            "obs-studio-version": "24.0.3",
            "available-requests": "Request1,Request2"
        });
        let req = GetVersion::default();
        let expected = responses::GetVersion {
            response_data: response_data(),
            version: 1.1,
            obs_websocket_version: "4.7.0".to_string(),
            obs_studio_version: "24.0.3".to_string(),
            available_requests: vec!["Request1".to_string(), "Request2".to_string()],
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_auth_required_true() {
        let request = json!({
            "request-type": "GetAuthRequired",
            "message-id": "",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "authRequired": true,
            "challenge": "ch",
            "salt": "sa",
        });
        let req = GetAuthRequired::default();
        let expected = responses::GetAuthRequired {
            response_data: response_data(),
            auth_required: true,
            challenge: Some("ch".to_string()),
            salt: Some("sa".to_string()),
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_auth_required_false() {
        let request = json!({
            "request-type": "GetAuthRequired",
            "message-id": "",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "authRequired": false,
        });
        let req = GetAuthRequired::default();
        let expected = responses::GetAuthRequired {
            response_data: response_data(),
            auth_required: false,
            challenge: None,
            salt: None,
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn authenticate() {
        let _ = env_logger::builder().is_test(true).try_init();
        let requests = vec![
            json!({
                "request-type": "GetAuthRequired",
                "message-id": "",
            }),
            json!({
                "request-type": "Authenticate",
                "message-id": "",
                "auth": "Z69J+b7C5Zj7jIXlqVp/xjp36sFSmpJpxZ41GN/UTu4=",
            }),
        ];
        let responses = vec![
            json!({
                "status": "ok",
                "message-id": "",
                "authRequired": true,
                "challenge": "123",
                "salt": "456",
            }),
            json!({
                "status": "ok",
                "message-id": "",
            }),
        ];
        let expected = responses::Empty {
            response_data: response_data(),
        };
        let (mut obs, handle) = init(responses);
        let res = obs.authenticate("todo").expect("authenticate");
        let actual_requests = handle.join().expect("join");
        obs.close();
        for (request, actual_request) in requests.into_iter().zip(actual_requests) {
            assert_eq!(
                request, actual_request,
                "request (left) did not match expected (right)"
            );
        }
        assert_eq!(
            res, expected,
            "result (left) did not match expected (right)"
        );
    }

    #[test]
    fn get_stats() {
        let request = json!({
            "request-type": "GetStats",
            "message-id": "",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "stats": {
                "fps": 0.0,
                "render-total-frames": 1,
                "render-missed-frames": 2,
                "output-total-frames": 3,
                "output-skipped-frames": 4,
                "average-frame-time": 5.0,
                "cpu-usage": 6.0,
                "memory-usage": 7.0,
                "free-disk-space": 8.0,
            }
        });
        let req = GetStats::default();
        let expected = responses::GetStats {
            response_data: response_data(),
            stats: responses::ObsStats {
                fps: 0.0,
                render_total_frames: 1,
                render_missed_frames: 2,
                output_total_frames: 3,
                output_skipped_frames: 4,
                average_frame_time: 5.0,
                cpu_usage: 6.0,
                memory_usage: 7.0,
                free_disk_space: 8.0,
            },
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_video_info() {
        let request = json!({
            "request-type": "GetVideoInfo",
            "message-id": "",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "baseWidth": 0,
            "baseHeight": 1,
            "outputWidth": 2,
            "outputHeight": 3,
            "scaleType": "VIDEO_SCALE_BICUBIC",
            "fps": 4.0,
            "videoFormat": "VIDEO_FORMAT_NV12",
            "colorSpace": "VIDEO_CS_601",
            "colorRange": "VIDEO_RANGE_PARTIAL",
        });
        let req = GetVideoInfo::default();
        let expected = responses::GetVideoInfo {
            response_data: response_data(),
            base_width: 0,
            base_height: 1,
            output_width: 2,
            output_height: 3,
            scale_type: responses::ScaleType::Bicubic,
            fps: 4.0,
            video_format: responses::VideoFormat::NV12,
            color_space: responses::ColorSpace::CS601,
            color_range: responses::ColorRange::Partial,
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn list_outputs() {
        let request = json!({
            "request-type": "ListOutputs",
            "message-id": "",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "outputs": [
                {
                    "name": "simple_file_output",
                    "type": "ffmpeg_muxer",
                    "width": 0,
                    "height": 1,
                    "flags": {
                        "rawValue": 6,
                        "audio": true,
                        "video": true,
                        "encoded": true,
                        "multiTrack": true,
                        "service": true,
                    },
                    "settings": {},
                    "active": false,
                    "reconnecting": false,
                    "congestion": 2.0,
                    "totalFrames": 3,
                    "droppedFrames": 4,
                    "totalBytes": 5,
                }
            ],
        });
        let req = ListOutputs::default();
        let expected = responses::ListOutputs {
            response_data: response_data(),
            outputs: vec![responses::Output {
                name: "simple_file_output".to_string(),
                output_type: "ffmpeg_muxer".to_string(),
                width: 0,
                height: 1,
                flags: responses::Flags {
                    raw_value: 6,
                    audio: true,
                    video: true,
                    encoded: true,
                    multi_track: true,
                    service: true,
                },
                settings: std::collections::HashMap::new(),
                active: false,
                reconnecting: false,
                congestion: 2.0,
                total_frames: 3,
                dropped_frames: 4,
                total_bytes: 5,
            }],
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_output_info() {
        let request = json!({
            "request-type": "GetOutputInfo",
            "message-id": "",
            "outputName": "output1",
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "outputInfo": {
                "name": "simple_file_output",
                "type": "ffmpeg_muxer",
                "width": 0,
                "height": 1,
                "flags": {
                    "rawValue": 6,
                    "audio": true,
                    "video": true,
                    "encoded": true,
                    "multiTrack": true,
                    "service": true,
                },
                "settings": {},
                "active": false,
                "reconnecting": false,
                "congestion": 2.0,
                "totalFrames": 3,
                "droppedFrames": 4,
                "totalBytes": 5,
            },
        });
        let req = GetOutputInfo::builder().output_name("output1").build();
        let expected = responses::GetOutputInfo {
            response_data: response_data(),
            output_info: responses::Output {
                name: "simple_file_output".to_string(),
                output_type: "ffmpeg_muxer".to_string(),
                width: 0,
                height: 1,
                flags: responses::Flags {
                    raw_value: 6,
                    audio: true,
                    video: true,
                    encoded: true,
                    multi_track: true,
                    service: true,
                },
                settings: std::collections::HashMap::new(),
                active: false,
                reconnecting: false,
                congestion: 2.0,
                total_frames: 3,
                dropped_frames: 4,
                total_bytes: 5,
            },
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_scene_item_properties() {
        let request = json!({
            "request-type": "GetSceneItemProperties",
            "message-id": "",
            "scene-name": "scene",
            "item": "source"
        });
        let response = json!({
            "status": "ok",
            "message-id": "",
            "name": "source",
            "position": {
                "x": 0,
                "y": 1,
                "alignment": 2,
            },
            "rotation": 3.0,
            "scale": {
                "x": 4.0,
                "y": 5.0,
            },
            "crop": {
                "top": 6,
                "right": 7,
                "bottom": 8,
                "left": 9,
            },
            "visible": true,
            "locked": true,
            "bounds": {
                "type": "OBS_BOUNDS_STRETCH",
                "alignment": 10,
                "x": 11.0,
                "y": 12.0,
            },
            "sourceWidth": 13,
            "sourceHeight": 14,
            "width": 15.0,
            "height": 16.0,
        });
        let req = GetSceneItemProperties::builder()
            .scene_name("scene")
            .item("source")
            .build();
        let expected = responses::GetSceneItemProperties {
            response_data: response_data(),
            name: "source".to_string(),
            position: responses::Position {
                x: 0.0,
                y: 1.0,
                alignment: 2,
            },
            rotation: 3.0,
            scale: responses::Scale { x: 4.0, y: 5.0 },
            crop: responses::Crop {
                top: 6,
                right: 7,
                bottom: 8,
                left: 9,
            },
            visible: true,
            locked: true,
            bounds: responses::Bounds {
                bounds_type: responses::BoundsType::Stretch,
                alignment: 10,
                x: 11.0,
                y: 12.0,
            },
            source_width: 13,
            source_height: 14,
            width: 15.0,
            height: 16.0,
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn set_scene_item_properties() {
        let request = json!({
            "request-type": "SetSceneItemProperties",
            "message-id": "",
            "scene-name": "scene",
            "item": "test",
            "position": {
                "x": 1.0,
                "y": 2.0,
                "alignment": 3,
            },
            "rotation": 4.0,
            "scale": {
                "x": 5.0,
                "y": 6.0,
            },
            "crop": {
                "top": 7,
                "right": 8,
                "bottom": 9,
                "left": 10,
            },
            "visible": true,
            "locked": true,
            "bounds": {
                "type": "OBS_BOUNDS_STRETCH",
                "alignment": 11,
                "x": 12.0,
                "y": 13.0,
            },
        });
        let response = json!({
            "message-id": "",
            "status": "ok",
        });
        let req = SetSceneItemProperties::builder()
            .scene_name("scene")
            .item("test")
            .position_x(1.0)
            .position_y(2.0)
            .position_alignment(3)
            .rotation(4.0)
            .scale_x(5.0)
            .scale_y(6.0)
            .crop_top(7)
            .crop_right(8)
            .crop_bottom(9)
            .crop_left(10)
            .visible(true)
            .locked(true)
            .bounds_type(BoundsType::Stretch)
            .bounds_alignment(11)
            .bounds_x(12.0)
            .bounds_y(13.0)
            .build();
        let expected = responses::Empty {
            response_data: response_data(),
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn reorder_scene_items() {
        let request = json!({
            "request-type": "ReorderSceneItems",
            "message-id": "",
            "scene": "s",
            "items": [
                {
                    "name": "n",
                },
                {
                    "id": 1,
                },
            ],
        });
        let response = json!({
            "message-id": "",
            "status": "ok",
        });
        let req = ReorderSceneItems::builder()
            .scene("s")
            .items(vec![ItemId::Name("n"), ItemId::Id(1)])
            .build();
        let expected = responses::Empty {
            response_data: response_data(),
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn obs_closed() {
        let _ = env_logger::builder().is_test(true).try_init();
        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        let mut obs = Obs::new();
        thread::spawn(move || {
            let (stream, _) = server.accept().expect("accept");
            let mut websocket = accept(stream).expect("failed to accept");
            log::info!("mock obs closing");
            websocket.close(None).expect("close");
        });
        obs.connect("localhost", port).expect("connect");
        assert!(obs.request(&GetVersion::default()).is_err());
    }

    #[test]
    fn obs_crash_after_accept() {
        let _ = env_logger::builder().is_test(true).try_init();
        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        let mut obs = Obs::new();
        thread::spawn(move || {
            use std::panic;

            let (stream, _) = server.accept().expect("accept");
            accept(stream).expect("failed to accept");
            log::info!("crashing mock obs");
            panic::set_hook(Box::new(|_| {}));
            panic!();
        });
        obs.connect("localhost", port).expect("connect");
        assert!(obs.request(&GetVersion::default()).is_err());
    }

    #[test]
    fn obs_crash_before_accept() {
        let _ = env_logger::builder().is_test(true).try_init();
        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        let mut obs = Obs::new();
        thread::spawn(move || {
            use std::panic;

            server.accept().expect("accept");
            log::info!("crashing mock obs");
            panic::set_hook(Box::new(|_| {}));
            panic!();
        });
        let res = obs.connect("localhost", port);
        assert!(res.is_err());
    }

    #[test]
    fn obs_offline() {
        let _ = env_logger::builder().is_test(true).try_init();
        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        let mut obs = Obs::new();
        let res = obs.connect("localhost", port);
        assert!(res.is_err());
    }
}
