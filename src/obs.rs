//! Contains Obs, the primary struct for interacting with the OBS WebSocket server.

use crate::{
    error::{HandlerError, ObsError},
    events::{self, Event},
    requests::*,
    responses,
};

use async_tungstenite::{
    tungstenite::{protocol::Role, Message as WebSocketMessage},
    WebSocketStream,
};
use futures::{
    channel::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot::{self, Sender as OneshotSender},
    },
    future::{self, Either},
    sink::SinkExt,
    stream::StreamExt,
};
use piper::Arc;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use smol::{Async, Timer};
use std::{
    collections::HashMap,
    net::{TcpStream, ToSocketAddrs},
    thread::{self, JoinHandle},
    time::Duration,
};

type WebSocketHandle = WebSocketStream<Arc<Async<TcpStream>>>;
type HandlerHandle = JoinHandle<Result<(), HandlerError>>;

/// The primary struct for interacting with the OBS WebSocket server.
pub struct Obs {
    connection_data: ConnectionData,
}

impl Obs {
    /// Attempts to connect to OBS. Starts a thread that handles
    /// Returns an error if already connected.
    pub async fn connect(
        address: &str,
        port: u16,
    ) -> Result<(Self, UnboundedReceiver<events::Event>), ObsError> {
        log::debug!("Connecting to: {}:{}", address, port);

        let (thread_sender, thread_receiver) = mpsc::unbounded::<Message>();
        let (event_sender, event_receiver) = mpsc::unbounded::<Event>();
        let (websocket_stream, send_socket, close_handle) =
            Obs::init_sockets(address, port).await?;
        let thread_handle =
            Obs::start_handler(send_socket, thread_receiver, websocket_stream, event_sender)
                .map_err(ObsError::Thread)?;

        let connection_data = ConnectionData {
            socket_handle: close_handle,
            thread_handle,
            thread_sender,
        };
        Ok((Obs { connection_data }, event_receiver))
    }

    /// Disconnects from OBS.
    /// Returns an error if not connected, or if there was an issue closing the WebSocket socket
    /// or closing the thread.
    pub async fn disconnect(self) -> Result<(), ObsError> {
        let ConnectionData {
            thread_sender,
            mut socket_handle,
            thread_handle,
        } = self.connection_data;

        log::info!("Closing connection");
        // closing thread sender should close the thread
        thread_sender.close_channel();
        let socket_res = socket_handle.close(None).await;
        let thread_res = thread_handle.join();

        if socket_res.is_err() || thread_res.is_err() {
            return Err(ObsError::DisconnectError {
                socket_error: socket_res.err().map(|e| Box::new(e.into())),
                thread_error: thread_res
                    .err()
                    .map(|e| Box::new(ObsError::HandlerThreadError(e))),
            });
        }
        Ok(())
    }

    /// Sends the given request to OBS.
    pub async fn request<T>(&self, req: &T) -> Result<T::Response, ObsError>
    where
        T: Request + std::fmt::Debug,
    {
        let ConnectionData { thread_sender, .. } = &self.connection_data;
        log::debug!("Requesting: {:#?}", req);
        let (message_id, value) = req.to_json();
        log::trace!("Converted request to JSON: {:#}", value);

        // channel for receiving the response
        let (oneshot_sender, oneshot_receiver) = oneshot::channel::<Result<Value, String>>();

        // send to handler thread
        let message = Message {
            message_id,
            value,
            sender: oneshot_sender,
        };
        log::trace!("Sending");
        thread_sender
            .unbounded_send(message)
            .map_err(|_| ObsError::ConnectionInterrupted)?;
        log::trace!("Sent");

        // wait for response from handler thread
        match oneshot_receiver.await {
            Ok(res) => match res {
                Ok(res) => {
                    log::debug!("Received response: {}", res);
                    Ok(serde_json::from_value(res)?)
                }
                Err(res) => {
                    log::error!("Received error: {:#?}", res);
                    Err(ObsError::ObsError(res))
                }
            },
            Err(canceled) => {
                log::info!("Channel to handler canceled: {}", canceled);
                Err(ObsError::OneshotCanceled(canceled))
            }
        }
    }

    /// Tries to authenticate with OBS. Returns an error if no authentication is required.
    pub async fn authenticate(&mut self, password: &str) -> Result<responses::Empty, ObsError> {
        let auth = self.request(&GetAuthRequired::builder().build()).await?;
        if auth.auth_required {
            log::debug!("Authentication required");
            let challenge = auth.challenge.ok_or(ObsError::MissingChallenge)?;
            let salt = auth.salt.ok_or(ObsError::MissingSalt)?;

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            log::info!("Authenticating");
            let req = Authenticate::builder().auth(auth_response).build();
            Ok(self.request(&req).await?)
        } else {
            Err(ObsError::NoAuthRequired)
        }
    }

    // initializes the connection to OBS WebSocket
    async fn init_sockets(
        address: &str,
        port: u16,
    ) -> Result<(WebSocketHandle, WebSocketHandle, WebSocketHandle), ObsError> {
        let addr = format!("{}:{}", address, port);
        let ws_addr = format!("ws://{}", addr);

        // parse addr
        let addr = addr
            .to_socket_addrs()
            .ok()
            .and_then(|mut sa| sa.next())
            .ok_or_else(|| ObsError::InvalidAddress(addr.clone()))?;
        log::debug!("Connecting TCP stream to: {}", addr);

        // connect to OBS
        let tcp_stream = Async::<TcpStream>::connect(addr).await?;
        let tcp_stream = Arc::new(tcp_stream);
        let send_stream = tcp_stream.clone();
        let close_stream = tcp_stream.clone();

        // establish WS connection to OBS with timeout
        let tungstenite_future = async_tungstenite::client_async(ws_addr, tcp_stream);
        futures::pin_mut!(tungstenite_future);
        let timer = Timer::after(Duration::from_millis(100));
        let (recv_socket, _res) = match future::select(tungstenite_future, timer).await {
            Either::Left((tungstenite_client, _)) => tungstenite_client?,
            Either::Right(_) => return Err(ObsError::TungsteniteTimeout),
        };

        let send_socket = WebSocketStream::from_raw_socket(send_stream, Role::Client, None).await;
        let close_socket = WebSocketStream::from_raw_socket(close_stream, Role::Client, None).await;
        Ok((recv_socket, send_socket, close_socket))
    }

    // handles an incoming WebSocket message from OBS
    async fn handle_incoming(
        pending_senders: &mut HashMap<String, OneshotSender<Result<Value, String>>>,
        event_sender: &mut UnboundedSender<events::Event>,
        message: String,
    ) -> Result<(), HandlerError> {
        log::trace!("Received text: {}", message);
        match serde_json::from_str::<ResponseOrEvent>(&message) {
            Ok(ResponseOrEvent::Response(response)) => {
                // see if we have a sender with a matching message-id
                if let Some(response_sender) = pending_senders.remove(&response.message_id) {
                    log::debug!("Received response: {:#?}", response);
                    let response = match response.response_data {
                        responses::ResponseData::Ok(value) => Ok(value),
                        responses::ResponseData::Error { error } => Err(error),
                    };
                    response_sender
                        .send(response)
                        .map_err(|_response| HandlerError::SendResponse)?;
                } else {
                    log::warn!("Unexpected response: {:?}", response);
                }
            }
            Ok(ResponseOrEvent::Event(event)) => {
                log::debug!("Received event: {:#?}", event);
                let _ = event_sender.send(*event).await; // ignore errors, user may have dropped event receiver
            }
            Err(e) => log::error!(
                "Received invalid text \"{}\" which failed to deserialize: {:#?}",
                message,
                e
            ),
        }
        Ok(())
    }

    // handles an outgoing Message to OBS
    async fn handle_outgoing(
        send_socket: &mut WebSocketHandle,
        pending_senders: &mut HashMap<String, OneshotSender<Result<Value, String>>>,
        message: Message,
    ) -> Result<(), HandlerError> {
        log::trace!("Received outgoing message: {:?}", message);
        send_socket
            .send(WebSocketMessage::text(message.value.to_string()))
            .await
            .map_err(HandlerError::Tungstenite)?;
        log::debug!("Sent text: {:#}", message.value);
        pending_senders.insert(message.message_id, message.sender);
        Ok(())
    }

    // starts the handler thread
    fn start_handler(
        mut send_socket: WebSocketHandle,
        mut outgoing_receiver: UnboundedReceiver<Message>,
        mut websocket_stream: WebSocketHandle,
        mut event_sender: UnboundedSender<events::Event>,
    ) -> Result<HandlerHandle, std::io::Error> {
        log::debug!("Starting handler");
        thread::Builder::new()
            .name("message_handler".to_string())
            .spawn(move || {
                smol::block_on(async move {
                    // { request's message-id -> oneshot sender for sending the response }
                    let mut pending_senders = HashMap::new();
                    // combine streams for outgoing (JSON from user) and incoming (WS from OBS) messages to thread
                    loop {
                        match future::select(outgoing_receiver.next(), websocket_stream.next())
                            .await
                        {
                            Either::Left((outgoing, _)) => match outgoing {
                                Some(outgoing) => {
                                    Obs::handle_outgoing(
                                        &mut send_socket,
                                        &mut pending_senders,
                                        outgoing,
                                    )
                                    .await?
                                }
                                None => {
                                    log::info!("Outgoing sender closed, closing thread");
                                    return Ok(());
                                }
                            },
                            Either::Right((incoming, _)) => match incoming {
                                Some(Ok(incoming)) => match incoming {
                                    WebSocketMessage::Text(incoming) => {
                                        // incoming text from OBS
                                        Obs::handle_incoming(
                                            &mut pending_senders,
                                            &mut event_sender,
                                            incoming,
                                        )
                                        .await?
                                    }
                                    WebSocketMessage::Close(close_frame) => {
                                        let reason = close_frame
                                            .map(|c| c.reason.into_owned())
                                            .unwrap_or_else(|| "no reason given".to_string());
                                        log::info!(
                                            "OBS closed WebSocket connection, closing thread: {}",
                                            reason
                                        );
                                        return Ok(());
                                    }
                                    unexpected => {
                                        log::warn!("Unexpected websocket message: {}", unexpected);
                                        continue;
                                    }
                                },
                                Some(Err(e)) => {
                                    log::error!("Tungstenite error, closing thread: {}", e);
                                    return Err(HandlerError::Tungstenite(e));
                                }
                                None => {
                                    log::info!("OBS socket closed, closing thread");
                                    return Ok(());
                                }
                            },
                        };
                    }
                })
            })
    }
}

// message from the WebSocket server
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum ResponseOrEvent {
    Response(responses::ResponseWrapper),
    Event(Box<events::Event>),
}

// message used to communicate with the handler channel that owns the WebSocket connection
#[derive(Debug)]
struct Message {
    // message id
    message_id: String,
    // JSON to be sent
    value: Value,
    // oneshot sender to send the result back with
    // ok contains the entire message which has been checked to not be an error
    // err contains the error message
    sender: OneshotSender<Result<Value, String>>,
}

// container for data related to the WebSocket connection
struct ConnectionData {
    socket_handle: WebSocketHandle,
    thread_handle: HandlerHandle,
    thread_sender: UnboundedSender<Message>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common_types::{self, *};
    use async_tungstenite::tungstenite::server::accept;
    use serde_json::{json, Value};
    use std::{
        net::TcpListener,
        thread::{spawn, JoinHandle},
    };

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn init_without_server(port: u16) -> Obs {
        log::debug!("initiating without server at {}", port);
        smol::block_on(Obs::connect("localhost", port))
            .expect("failed to connect")
            .0
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
            for mut response in responses {
                let message = websocket.read_message().expect("failed to read message");
                log::info!("read message {:#?}", message);
                let parsed = serde_json::from_str::<Value>(&message.to_string())
                    .expect("failed to deserialize");
                let message_id = parsed
                    .as_object()
                    .as_ref()
                    .unwrap()
                    .get("message-id")
                    .unwrap()
                    .clone();
                actual_requests.push(parsed);
                response
                    .as_object_mut()
                    .unwrap()
                    .insert("message-id".to_string(), message_id);
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

    fn request_test<T>(
        expected_requests: Vec<Value>,
        expected_responses: Vec<Value>,
        request: T,
        expected: T::Response,
    ) where
        T: Request + PartialEq + std::fmt::Debug,
        T::Response: PartialEq + std::fmt::Debug,
    {
        let (obs, handle) = init(expected_responses);
        let res = smol::block_on(obs.request(&request)).expect("request returned err");
        let actual_requests = handle.join().expect("failed to join");
        smol::block_on(obs.disconnect()).unwrap();
        for (request, mut actual_request) in expected_requests.into_iter().zip(actual_requests) {
            // ignore message-id in the comparison
            actual_request
                .as_object_mut()
                .unwrap()
                .remove("message-id")
                .unwrap();
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
        init_logger();

        let request = json!({
        "request-type": "GetVersion",
        });
        let response = json!({
            "status": "ok",
            "version": 1.1,
            "obs-websocket-version": "4.7.0",
            "obs-studio-version": "24.0.3",
            "available-requests": "Request1,Request2"
        });
        let req = GetVersion::builder().build();
        let expected = responses::GetVersion {
            version: 1.1,
            obs_websocket_version: "4.7.0".to_string(),
            obs_studio_version: "24.0.3".to_string(),
            available_requests: vec!["Request1".to_string(), "Request2".to_string()],
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_auth_required_true() {
        init_logger();

        let request = json!({
            "request-type": "GetAuthRequired",
        });
        let response = json!({
            "status": "ok",
            "authRequired": true,
            "challenge": "ch",
            "salt": "sa",
        });
        let req = GetAuthRequired::builder().build();
        let expected = responses::GetAuthRequired {
            auth_required: true,
            challenge: Some("ch".to_string()),
            salt: Some("sa".to_string()),
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn get_auth_required_false() {
        init_logger();

        let request = json!({
            "request-type": "GetAuthRequired",
        });
        let response = json!({
            "status": "ok",
            "authRequired": false,
        });
        let req = GetAuthRequired::builder().build();
        let expected = responses::GetAuthRequired {
            auth_required: false,
            challenge: None,
            salt: None,
        };
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn authenticate() {
        init_logger();

        let requests = vec![
            json!({
                "request-type": "GetAuthRequired",
            }),
            json!({
                "request-type": "Authenticate",
                "auth": "Z69J+b7C5Zj7jIXlqVp/xjp36sFSmpJpxZ41GN/UTu4=",
            }),
        ];
        let responses = vec![
            json!({
                "status": "ok",
                "authRequired": true,
                "challenge": "123",
                "salt": "456",
            }),
            json!({
                "status": "ok",
            }),
        ];
        let expected = responses::Empty {};
        let (mut obs, handle) = init(responses);
        let res = smol::block_on(obs.authenticate("todo")).expect("authenticate");
        let actual_requests = handle.join().expect("join");
        smol::block_on(obs.disconnect()).unwrap();
        for (request, mut actual_request) in requests.into_iter().zip(actual_requests) {
            // remove message-id
            actual_request
                .as_object_mut()
                .unwrap()
                .remove("message-id")
                .unwrap();
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
        init_logger();

        let request = json!({
            "request-type": "GetStats",
        });
        let response = json!({
            "status": "ok",
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
        let req = GetStats::builder().build();
        let expected = responses::GetStats {
            stats: ObsStats {
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
        init_logger();

        let request = json!({
            "request-type": "GetVideoInfo",
        });
        let response = json!({
            "status": "ok",
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
        let req = GetVideoInfo::builder().build();
        let expected = responses::GetVideoInfo {
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
        init_logger();

        let request = json!({
            "request-type": "ListOutputs",
        });
        let response = json!({
            "status": "ok",
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
        let req = ListOutputs::builder().build();
        let expected = responses::ListOutputs {
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
                settings: Value::Object(serde_json::Map::new()),
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
        init_logger();

        let request = json!({
            "request-type": "GetOutputInfo",
            "outputName": "output1",
        });
        let response = json!({
            "status": "ok",
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
                settings: Value::Object(serde_json::Map::new()),
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
        init_logger();

        let request = json!({
            "request-type": "GetSceneItemProperties",
            "scene-name": "scene",
            "item": "source"
        });
        let response = json!({
            "status": "ok",
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
            name: "source".to_string(),
            position: common_types::Position {
                x: 0.0,
                y: 1.0,
                alignment: 2,
            },
            rotation: 3.0,
            scale: common_types::Scale { x: 4.0, y: 5.0 },
            crop: common_types::Crop {
                top: 6,
                right: 7,
                bottom: 8,
                left: 9,
            },
            visible: true,
            locked: true,
            bounds: common_types::Bounds {
                bounds_type: common_types::BoundsType::Stretch,
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
        init_logger();

        let request = json!({
            "request-type": "SetSceneItemProperties",
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
        let expected = responses::Empty {};
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn reorder_scene_items() {
        init_logger();

        let request = json!({
            "request-type": "ReorderSceneItems",
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
            "status": "ok",
        });
        let req = ReorderSceneItems::builder()
            .scene("s")
            .items(vec![ItemId::Name("n".to_string()), ItemId::Id(1)])
            .build();
        let expected = responses::Empty {};
        request_test(vec![request], vec![response], req, expected);
    }

    #[test]
    fn obs_closed() {
        init_logger();

        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        thread::spawn(move || {
            let (stream, _) = server.accept().expect("accept");
            let mut websocket = accept(stream).expect("failed to accept");
            log::info!("mock obs closing");
            websocket.close(None).expect("close");
        });
        let obs = smol::block_on(Obs::connect("localhost", port))
            .expect("connect")
            .0;
        assert!(smol::block_on(obs.request(&GetVersion::builder().build())).is_err());
    }

    #[test]
    fn obs_crash_after_accept() {
        init_logger();

        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        thread::spawn(move || {
            use std::panic;

            let (stream, _) = server.accept().expect("accept");
            accept(stream).expect("failed to accept");
            log::info!("crashing mock obs");
            panic::set_hook(Box::new(|_| {}));
            panic!();
        });
        let obs = smol::block_on(Obs::connect("localhost", port))
            .expect("connect")
            .0;
        assert!(smol::block_on(obs.request(&GetVersion::builder().build())).is_err());
    }

    #[test]
    fn obs_crash_before_accept() {
        init_logger();

        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();
        thread::spawn(move || {
            use std::panic;

            server.accept().expect("accept");
            log::info!("crashing mock obs");
            panic::set_hook(Box::new(|_| {}));
            panic!();
        });
        let res = smol::block_on(Obs::connect("localhost", port));
        assert!(res.is_err());
    }

    #[test]
    fn server_not_accepting_websocket() {
        init_logger();

        let server = TcpListener::bind("localhost:0").expect("bind");
        let port = server.local_addr().expect("local addr").port();

        let res = smol::block_on(Obs::connect("localhost", port));
        assert!(res.is_err());
    }

    #[test]
    fn obs_offline() {
        init_logger();

        let res = smol::block_on(Obs::connect("localhost", 1234));
        assert!(res.is_err());
    }
}
