pub mod error;
mod events;
pub mod requests;
pub mod responses;

#[macro_use]
extern crate typed_builder;

use base64;
use error::Error;
use futures::{
    channel::{
        mpsc::{channel, Receiver, Sender},
        oneshot::{channel as oneshot_channel, Sender as OneshotSender},
    },
    executor, future,
    stream::{select, StreamExt, TryStreamExt},
    task::{Context, Poll},
    Stream,
};
use log::{debug, error, info, trace, warn};
use requests::*;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    io::ErrorKind as IoError,
    net::TcpStream,
    pin::Pin,
    thread::{self, JoinHandle},
};
use tungstenite::{
    client, protocol::Role, Error as WebSocketError, Message as WebSocketMessage, WebSocket,
};

type Result<T> = std::result::Result<T, Error>;
type StdResult<T, E> = std::result::Result<T, E>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
enum ResponseOrEvent {
    Response(responses::Response),
    Event(events::Event),
}

#[derive(Debug)]
enum Message {
    Outgoing(Value, OneshotSender<StdResult<String, responses::Response>>),
    Incoming(WebSocketMessage),
}

struct WebSocketStream(WebSocket<TcpStream>);

impl Stream for WebSocketStream {
    type Item = StdResult<Message, ()>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let message = Pin::into_inner(self).0.read_message();
        match message {
            Ok(message) => Poll::Ready(Some(Ok(Message::Incoming(message)))),
            Err(error) => match error {
                WebSocketError::Io(error) => match error.kind() {
                    IoError::WouldBlock => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    other_error => {
                        warn!("other IO error: {:#?}", other_error);
                        Poll::Ready(None) // TODO: handle properly
                    }
                },
                WebSocketError::Protocol(protocol_violation) => {
                    error!("protocol violation: {:#?}", protocol_violation);
                    Poll::Ready(Some(Err(())))
                }
                other_error => {
                    warn!("other error: {:#?}", other_error);
                    Poll::Ready(None) // TODO: handle properly
                }
            },
        }
    }
}

#[derive(Default)]
pub struct Obs {
    socket_handle: Option<WebSocket<TcpStream>>,
    thread_handle: Option<JoinHandle<()>>,
    thread_sender: Option<Sender<Message>>,
}

impl Obs {
    pub fn new() -> Self {
        Obs::default()
    }

    fn init_sockets(
        address: &str,
        port: u16,
    ) -> Result<(WebSocketStream, WebSocket<TcpStream>, WebSocket<TcpStream>)> {
        let addr = format!("{}:{}", address, port);
        let ws_addr = format!("ws://{}", addr);
        debug!("connecting to {}", addr);
        let recv_stream = TcpStream::connect(addr)?;
        let send_stream = recv_stream.try_clone()?;
        let close_stream = recv_stream.try_clone()?;
        let (recv_socket, _res) = client(ws_addr, recv_stream)?;
        close_stream.set_nonblocking(true)?;

        let recv_socket_iter = WebSocketStream(recv_socket);
        let send_socket = WebSocket::from_raw_socket(send_stream, Role::Client, None);
        let close_socket = WebSocket::from_raw_socket(close_stream, Role::Client, None);
        Ok((recv_socket_iter, send_socket, close_socket))
    }

    fn start_handler(
        mut send_socket: WebSocket<TcpStream>,
        outgoing_receiver: Receiver<Message>,
        websocket_stream: WebSocketStream,
    ) -> JoinHandle<()> {
        debug!("starting handler");
        thread::Builder::new().name("handler".to_string()).spawn(move || {
            let outgoing_receiver_adapted = outgoing_receiver.map(|msg| Ok(msg));
            let websocket_stream_adapted = websocket_stream.map(|msg| msg.map_err(|err| std::result::Result::<(), ()>::Err(err)));
            let streams = select(outgoing_receiver_adapted, websocket_stream_adapted);
            let mut pending_sender = None;
            let fut = streams.try_for_each(|message| {
                trace!("received message");
                match message {
                    Message::Outgoing(json, sender) => {
                        trace!("received outgoing message");
                        send_socket
                            .write_message(WebSocketMessage::text(json.to_string()))
                            .expect("failed to write message");
                        debug!("sent text {}", json);
                        pending_sender = Some(sender);
                    }
                    Message::Incoming(message) => match message {
                        WebSocketMessage::Close(_) => {
                            info!("websocket connection closed");
                            return future::err(Err(()));
                        }
                        WebSocketMessage::Text(text) => {
                            debug!("received text {}", text);
                            let parsed = serde_json::from_str::<ResponseOrEvent>(&text);
                            match parsed {
                                Ok(ResponseOrEvent::Response(response)) => {
                                    trace!("received response {:#?}", response);
                                    if let Some(error) = &response.error {
                                        error!("error: {}", error);
                                        if let Some(sender) = pending_sender.take() {
                                            sender.send(Err(response)).expect("failed to send");
                                        } else {
                                            warn!("unexpected response");
                                        }
                                    } else {
                                        if let Some(sender) = pending_sender.take() {
                                            sender.send(Ok(text)).expect("failed to send");
                                        } else {
                                            warn!("unexpected response");
                                        }
                                    }
                                }
                                Ok(ResponseOrEvent::Event(event)) => {
                                    info!("received event {:#?}", event);
                                }
                                Err(e) => error!("received invalid text: {} which failed to deserialize with {:#?}", text, e),
                            }
                        }
                        _ => {
                            warn!("unexpected websocket message");
                        }
                    },
                }
                future::ready(Ok(()))
            });
            let res = executor::block_on(fut);
            match res {
                Ok(ok) => debug!("{:#?}", ok),
                Err(err) => debug!("{:#?}", err),
            }
            info!("receivers closed");
        }).expect("failed to create thread")
    }

    pub fn connect(&mut self, address: &str, port: u16) -> Result<()> {
        debug!("connecting to {}:{}", address, port);
        let (thread_sender, thread_receiver) = channel(2048);
        let (websocket_stream, send_socket, close_socket) = Obs::init_sockets(address, port)?;
        let handle = Obs::start_handler(send_socket, thread_receiver, websocket_stream);

        self.socket_handle = Some(close_socket);
        self.thread_handle = Some(handle);
        self.thread_sender = Some(thread_sender);
        Ok(())
    }

    pub fn close(self) {
        debug!("closing");
        self.thread_sender
            .expect("no thread sender")
            .close_channel();
        self.socket_handle
            .expect("no socket handle")
            .close(None)
            .expect("failed to close socket handle");
        self.thread_handle
            .expect("no thread handle")
            .join()
            .expect("failed to join thread handle");
    }

    fn request<T>(&mut self, req: T) -> Result<T::Output>
    where
        T: ToRequest + std::fmt::Debug,
    {
        debug!("requesting {:#?}", req);
        let val = req.to_request();
        trace!("converted request to json {}", val);
        let (os1, or1) = oneshot_channel();
        let message = Message::Outgoing(val, os1);
        trace!("sending");
        if self
            .thread_sender
            .as_mut()
            .expect("no thread sender")
            .try_send(message)
            .is_err()
        {
            self.thread_sender
                .as_mut()
                .expect("should have thread sender")
                .close_channel();
            self.socket_handle
                .as_mut()
                .expect("should ahve socket handle")
                .close(None)
                .expect("failed to close socket");
            self.thread_sender = None;
            self.socket_handle = None;
            self.thread_handle = None;
            return Err(Error::Custom("connection interrupted".to_string()));
        }
        trace!("sent");
        let res = executor::block_on(or1);
        match res {
            Ok(res) => match res {
                Ok(res) => {
                    debug!("received response {}", res);
                    Ok(serde_json::from_str(&res)?)
                }
                Err(res) => {
                    error!("received error {:#?}", res);
                    Err(Error::ObsError(
                        res.error
                            .expect("error from sender should have error message"),
                    ))
                }
            },
            Err(e) => {
                info!("channel to handler closed: {}", e);
                Err(Error::Custom("channel to handler closed".to_string()))
            }
        }
    }

    pub fn authenticate(&mut self, password: &str) -> Result<responses::Empty> {
        let auth = self.request(GetAuthRequired::builder().build())?;
        if auth.auth_required {
            info!("auth required");
            let challenge = auth.challenge.expect("should have challenge");
            let salt = auth.salt.expect("should have salt");

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            info!("authing");
            let req = Authenticate::builder().auth(&auth_response).build();
            Ok(self.request(req)?)
        } else {
            Err(Error::ObsError("no auth required".to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
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
        debug!("initiating without server at {}", port);
        let mut obs = Obs::new();
        obs.connect("localhost", port).expect("failed to connect");
        obs
    }

    fn init(responses: Vec<Value>) -> (Obs, JoinHandle<Vec<Value>>) {
        let server = TcpListener::bind("localhost:0").expect("failed to bind");
        let port = server.local_addr().expect("local addr").port();
        info!("mock server started at {}", port);
        let handle = spawn(move || {
            let mut actual_requests = vec![];
            let (stream, _) = server.accept().expect("accept");
            info!("incoming connection");
            let mut websocket = accept(stream).expect("failed to accept");
            for response in responses {
                let message = websocket.read_message().expect("failed to read message");
                info!("read message {:#?}", message);
                let parsed = serde_json::from_str::<Value>(&message.to_string())
                    .expect("failed to deserialize");
                actual_requests.push(parsed);
                info!("responding with {:#?}", response);
                websocket
                    .write_message(WebSocketMessage::Text(response.to_string()))
                    .expect("failed to write");
            }
            info!("closing mock server");
            websocket.close(None).expect("failed to close");
            actual_requests
        });
        let obs = init_without_server(port);
        (obs, handle)
    }

    fn request_test<T>(requests: Vec<Value>, responses: Vec<Value>, request: T, expected: T::Output)
    where
        T: ToRequest + PartialEq + std::fmt::Debug,
        T::Output: PartialEq + std::fmt::Debug,
    {
        let _ = env_logger::builder().is_test(true).try_init();
        let (mut obs, handle) = init(responses);
        let res = obs.request(request).expect("request returned err");
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
            info!("mock obs closing");
            websocket.close(None).expect("close");
        });
        obs.connect("localhost", port).expect("connect");
        assert!(obs.request(GetVersion::default()).is_err());
    }

    #[test]
    fn obs_crash() {
        let _ = env_logger::builder().is_test(true).try_init();
        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        let mut obs = Obs::new();
        thread::spawn(move || {
            use std::panic;

            let (stream, _) = server.accept().expect("accept");
            accept(stream).expect("failed to accept");
            info!("crashing mock obs");
            panic::set_hook(Box::new(|_| {}));
            panic!();
        });
        obs.connect("localhost", port).expect("connect");
        assert!(obs.request(GetVersion::default()).is_err());
    }
}
