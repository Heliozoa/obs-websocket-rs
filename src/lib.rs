mod error;
mod events;
mod requests;
mod responses;

use base64;
use error::Error;
use futures::{
    channel::{
        mpsc::{channel, Receiver, Sender},
        oneshot::{channel as oneshot_channel, Sender as OneshotSender},
    },
    executor, future,
    stream::{select, StreamExt},
    task::{Context, Poll},
    Stream,
};
use log::info;
use serde::de::DeserializeOwned;
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

#[derive(Debug)]
enum Message {
    Outgoing(Value, OneshotSender<String>),
    Incoming(WebSocketMessage),
}

struct WebSocketStream(WebSocket<TcpStream>);

impl Stream for WebSocketStream {
    type Item = Message;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let message = Pin::into_inner(self).0.read_message();
        match message {
            Ok(message) => Poll::Ready(Some(Message::Incoming(message))),
            Err(error) => match error {
                WebSocketError::Io(error) => match error.kind() {
                    IoError::WouldBlock => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    _ => Poll::Ready(None), // todo
                },
                _ => Poll::Ready(None), // todo
            },
        }
    }
}

pub struct Obs {
    socket_handle: Option<WebSocket<TcpStream>>,
    thread_handle: Option<JoinHandle<()>>,
    thread_sender: Option<Sender<Message>>,
}

impl Obs {
    pub fn new() -> Self {
        Obs {
            socket_handle: None,
            thread_handle: None,
            thread_sender: None,
        }
    }

    fn init_sockets(port: u16) -> (WebSocketStream, WebSocket<TcpStream>, WebSocket<TcpStream>) {
        let addr = format!("localhost:{}", port);
        let ws_addr = format!("ws://{}", addr);
        let recv_stream = TcpStream::connect(addr).unwrap();
        let send_stream = recv_stream.try_clone().unwrap();
        let close_stream = recv_stream.try_clone().unwrap();
        let (recv_socket, _res) = client(ws_addr, recv_stream).unwrap();
        close_stream.set_nonblocking(true).unwrap();

        let recv_socket_iter = WebSocketStream(recv_socket);
        let send_socket = WebSocket::from_raw_socket(send_stream, Role::Client, None);
        let close_socket = WebSocket::from_raw_socket(close_stream, Role::Client, None);
        (recv_socket_iter, send_socket, close_socket)
    }

    fn start_handler(
        mut send_socket: WebSocket<TcpStream>,
        outgoing_receiver: Receiver<Message>,
        websocket_stream: WebSocketStream,
    ) -> JoinHandle<()> {
        let handle = thread::spawn(move || {
            let streams = select(outgoing_receiver, websocket_stream);
            let mut pending_sender = None;
            let fut = streams.for_each(|message| {
                match message {
                    Message::Outgoing(json, sender) => {
                        send_socket
                            .write_message(WebSocketMessage::text(json.to_string()))
                            .unwrap();
                        pending_sender = Some(sender);
                    }
                    Message::Incoming(message) => match message {
                        WebSocketMessage::Close(_) => {
                            info!("closed websocket");
                        }
                        WebSocketMessage::Text(text) => {
                            if let Some(sender) = pending_sender.take() {
                                sender.send(text).unwrap();
                            }
                        }
                        _ => {}
                    },
                }
                future::ready(())
            });
            executor::block_on(fut);
            info!("receivers closed");
        });
        handle
    }

    pub fn connect(&mut self, port: u16) {
        let (thread_sender, thread_receiver) = channel(2048);
        let (websocket_stream, send_socket, close_socket) = Obs::init_sockets(port);
        let handle = Obs::start_handler(send_socket, thread_receiver, websocket_stream);

        self.socket_handle = Some(close_socket);
        self.thread_handle = Some(handle);
        self.thread_sender = Some(thread_sender);
    }

    pub fn close(self) {
        self.thread_sender.unwrap().close_channel();
        self.socket_handle.unwrap().close(None).unwrap();
        self.thread_handle.unwrap().join().unwrap();
    }

    fn get<T: DeserializeOwned>(&mut self, json: Value) -> Result<T> {
        let (os1, or1) = oneshot_channel();
        let message = Message::Outgoing(json, os1);
        self.thread_sender
            .as_mut()
            .unwrap()
            .try_send(message)
            .unwrap();
        info!("blocking");
        let res = executor::block_on(or1).unwrap();
        info!("done");
        Ok(serde_json::from_str(&res)?)
    }

    pub fn get_version(&mut self) -> Result<responses::GetVersion> {
        self.get(requests::get_version("0"))
    }

    pub fn get_auth_required(&mut self) -> Result<responses::GetAuthRequired> {
        self.get(requests::get_auth_required("0"))
    }

    pub fn authenticate(&mut self, password: &str) -> Result<responses::Response> {
        let auth: responses::GetAuthRequired = self.get(requests::get_auth_required("0"))?;
        if auth.auth_required {
            info!("auth required");
            let challenge = auth.challenge.unwrap();
            let salt = auth.salt.unwrap();

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            info!("authing");
            Ok(self.get(requests::authenticate("0", &auth_response))?)
        } else {
            Err(Error::ObsError("no auth required".to_string()))
        }
    }

    pub fn set_heartbeat(&mut self, enable: bool) -> Result<responses::Response> {
        self.get(requests::set_heartbeat("0", enable))
    }

    pub fn set_filename_formatting(
        &mut self,
        filename_formatting: &str,
    ) -> Result<responses::Response> {
        self.get(requests::set_filename_formatting("0", filename_formatting))
    }

    pub fn get_filename_formatting(&mut self) -> Result<responses::GetFilenameFormatting> {
        self.get(requests::get_filename_formatting("0"))
    }

    pub fn get_stats(&mut self) -> Result<responses::GetStats> {
        self.get(requests::get_stats("0"))
    }

    pub fn broadcast_custom_message(
        &mut self,
        realm: &str,
        data: Value,
    ) -> Result<responses::Response> {
        self.get(requests::broadcast_custom_message("0", realm, data))
    }

    pub fn get_video_info(&mut self) -> Result<responses::GetVideoInfo> {
        self.get(requests::get_video_info("0"))
    }

    pub fn list_outputs(&mut self) -> Result<responses::ListOutputs> {
        self.get(requests::list_outputs("0"))
    }

    pub fn get_output_info(&mut self, output_name: &str) -> Result<responses::GetOutputInfo> {
        self.get(requests::get_output_info("0", output_name))
    }

    pub fn start_output(&mut self, output_name: &str) -> Result<responses::Response> {
        self.get(requests::start_output("0", output_name))
    }

    pub fn stop_output(&mut self, output_name: &str, force: bool) -> Result<responses::Response> {
        self.get(requests::stop_output("0", output_name, force))
    }

    pub fn set_current_profile(&mut self, profile_name: &str) -> Result<responses::Response> {
        self.get(requests::set_current_profile("0", profile_name))
    }

    pub fn get_current_profile(&mut self) -> Result<responses::Profile> {
        self.get(requests::get_current_profile("0"))
    }

    pub fn list_profiles(&mut self) -> Result<responses::ListProfiles> {
        self.get(requests::list_profiles("0"))
    }

    pub fn toggle_recording(&mut self) -> Result<responses::Response> {
        self.get(requests::start_stop_recording("0"))
    }

    pub fn start_recording(&mut self) -> Result<responses::Response> {
        self.get(requests::start_recording("0"))
    }

    pub fn stop_recording(&mut self) -> Result<responses::Response> {
        self.get(requests::stop_recording("0"))
    }

    pub fn pause_recording(&mut self) -> Result<responses::Response> {
        self.get(requests::pause_recording("0"))
    }

    pub fn resume_recording(&mut self) -> Result<responses::Response> {
        self.get(requests::resume_recording("0"))
    }

    pub fn set_recording_folder(&mut self, rec_folder: &str) -> Result<responses::Response> {
        self.get(requests::set_recording_folder("0", rec_folder))
    }

    pub fn get_recording_folder(&mut self) -> Result<responses::GetRecordingFolder> {
        self.get(requests::get_recording_folder("0"))
    }

    pub fn toggle_replay_buffer(&mut self) -> Result<responses::Response> {
        self.get(requests::start_stop_replay_buffer("0"))
    }

    pub fn start_replay_buffer(&mut self) -> Result<responses::Response> {
        self.get(requests::start_replay_buffer("0"))
    }

    pub fn stop_replay_buffer(&mut self) -> Result<responses::Response> {
        self.get(requests::stop_replay_buffer("0"))
    }

    pub fn save_replay_buffer(&mut self) -> Result<responses::Response> {
        self.get(requests::save_replay_buffer("0"))
    }

    pub fn set_current_scene_collection(&mut self, sc_name: &str) -> Result<responses::Response> {
        self.get(requests::set_current_scene_collection("0", sc_name))
    }

    pub fn get_current_scene_collection(&mut self) -> Result<responses::SceneCollection> {
        self.get(requests::get_current_scene_collection("0"))
    }

    pub fn list_scene_collections(&mut self) -> Result<responses::ListSceneCollections> {
        self.get(requests::list_scene_collections("0"))
    }

    pub fn get_scene_item_properties(
        &mut self,
        scene_name: Option<&str>,
        item: &str,
    ) -> Result<responses::GetSceneItemProperties> {
        self.get(requests::get_scene_item_properties("0", scene_name, item))
    }

    pub fn set_scene_item_properties(
        &mut self,
        scene_name: Option<&str>,
        item: &str,
        position_x: Option<f64>,
        position_y: Option<f64>,
        position_alignment: Option<i32>,
        rotation: Option<f64>,
        scale_x: Option<f64>,
        scale_y: Option<f64>,
        crop_top: Option<i32>,
        crop_right: Option<i32>,
        crop_bottom: Option<i32>,
        crop_left: Option<i32>,
        visible: Option<bool>,
        locked: Option<bool>,
        bounds_type: Option<requests::BoundsType>,
        bounds_alignment: Option<i32>,
        bounds_x: Option<f64>,
        bounds_y: Option<f64>,
    ) -> Result<responses::Response> {
        self.get(requests::set_scene_item_properties(
            "0",
            scene_name,
            item,
            position_x,
            position_y,
            position_alignment,
            rotation,
            scale_x,
            scale_y,
            crop_top,
            crop_right,
            crop_bottom,
            crop_left,
            visible,
            locked,
            bounds_type,
            bounds_alignment,
            bounds_x,
            bounds_y,
        ))
    }

    pub fn reset_scene_item(
        &mut self,
        scene_name: Option<String>,
        item: String,
    ) -> Result<responses::Response> {
        self.get(requests::reset_scene_item(scene_name, item))
    }

    pub fn delete_scene_item(
        &mut self,
        scene: Option<String>,
        item: responses::Item,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn duplicate_scene_item(
        &mut self,
        from_scene: Option<String>,
        to_scene: Option<String>,
        item: responses::Item,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_current_scene(&mut self, scene_name: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_current_scene(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_scene_list(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn reorder_scene_items(
        &mut self,
        scene: Option<String>,
        items: Vec<responses::Item>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_sources_list(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_source_types_list(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_volume(&mut self, source: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_volume(&mut self, source: String, volume: f64) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_mute(&mut self, source: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_mute(&mut self, source: String, mute: bool) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn toggle_mute(&mut self, source: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_sync_offset(&mut self, source: String, offset: i32) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_sync_offset(&mut self, source: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_source_settings(
        &mut self,
        source_name: String,
        source_type: Option<String>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_source_settings(
        &mut self,
        source_name: String,
        source_type: Option<String>,
        source_settings: Value,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_text_gdi_plus_properties(&mut self, source: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_text_gdi_plus_properties(
        &mut self,
        source: String,
        align: responses::Align,
        bk_color: Option<i32>,
        bk_opacity: Option<i32>,
        chatlog: Option<bool>,
        chatlog_lines: Option<i32>,
        color: Option<i32>,
        extents: Option<bool>,
        extents_cx: Option<bool>,
        extents_cy: Option<bool>,
        file: Option<String>,
        read_from_file: Option<bool>,
        font: responses::Font,
        gradient: Option<bool>,
        gradient_color: Option<i32>,
        gradient_dir: Option<f64>,
        gradient_opacity: Option<i32>,
        outline: Option<bool>,
        outline_color: Option<i32>,
        outline_size: Option<i32>,
        outline_opacity: Option<i32>,
        text: Option<String>,
        valign: Option<String>,
        vertical: Option<bool>,
        render: Option<bool>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_text_freetype_2_properties(
        &mut self,
        source: String,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_text_freetype_2_properties(
        &mut self,
        source: String,
        color_1: Option<i32>,
        color_2: Option<i32>,
        custom_width: Option<i32>,
        drop_shadow: Option<i32>,
        font: responses::Font,
        from_file: Option<bool>,
        log_mode: Option<bool>,
        outline: Option<bool>,
        text: Option<String>,
        text_file: Option<String>,
        word_wrap: Option<bool>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_browser_source_properties(&mut self, source: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_browser_source_properties(
        &mut self,
        source: String,
        is_local_file: Option<bool>,
        local_file: Option<String>,
        url: Option<String>,
        css: Option<String>,
        width: Option<i32>,
        height: Option<i32>,
        fps: Option<i32>,
        shutdown: Option<bool>,
        render: Option<bool>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_special_sources(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_source_filters(&mut self, source_name: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_source_filter_info(
        &mut self,
        source_name: String,
        filter_name: String,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn add_filter_to_source(
        &mut self,
        source_name: String,
        filter_name: String,
        filter_type: String,
        filter_settings: Value,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn remove_filter_from_source(
        &mut self,
        source_name: String,
        filter_name: String,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn reorder_source_filter(
        &mut self,
        source_name: String,
        filter_name: String,
        new_index: i32,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn move_source_filter(
        &mut self,
        source_name: String,
        filter_name: String,
        movement_type: responses::MovementType,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_source_filter_settings(
        &mut self,
        source_name: String,
        filter_name: String,
        filter_settings: Value,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_source_filter_visibility(
        &mut self,
        source_name: String,
        filter_name: String,
        filter_enabled: bool,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn take_source_screenshot(
        &mut self,
        source_name: String,
        embed_picture_format: Option<String>,
        save_to_file_path: Option<String>,
        width: Option<i32>,
        height: Option<i32>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_streaming_status(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn toggle_streaming(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn start_streaming(&mut self, stream: requests::Stream) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn stop_streaming(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_stream_settings(
        &mut self,
        stream_type: String,
        settings: requests::StreamSettings,
        save: bool,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_stream_settings(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn save_stream_settings(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn send_captions(&mut self, text: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_studio_mode_status(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_preview_scene(
        &mut self,
        name: String,
        sources: Vec<responses::SceneItem>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_preview_scene(&mut self, scene_name: String) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn transition_to_program(
        &mut self,
        with_transition: Option<requests::WithTransition>,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn enable_studio_mode(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn disable_studio_mode(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn toggle_studio_mode(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_transition_list(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_current_transition(&mut self) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_current_transition(
        &mut self,
        transition_name: String,
    ) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn set_transition_duration(&mut self, duration: i32) -> Result<responses::Response> {
        unimplemented!()
    }

    pub fn get_transition_duration(&mut self) -> Result<responses::Response> {
        unimplemented!()
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

    fn init_without_server(port: u16) -> Obs {
        let mut obs = Obs::new();
        obs.connect(port);
        obs
    }

    fn init(responses: Vec<Value>) -> (Obs, JoinHandle<Vec<Value>>) {
        let server = TcpListener::bind("localhost:0").unwrap();
        let port = server.local_addr().unwrap().port();
        info!("mock server started at {}", port);
        let handle = spawn(move || {
            let mut actual_requests = vec![];
            let (stream, _) = server.accept().unwrap();
            info!("incoming connection");
            let mut websocket = accept(stream).expect("failed to accept");
            for response in responses {
                let message = websocket.read_message().expect("failed to read message");
                info!("read message");
                let parsed = serde_json::from_str::<Value>(&message.to_string())
                    .expect("failed to deserialize");
                actual_requests.push(parsed);
                websocket
                    .write_message(WebSocketMessage::Text(response.to_string()))
                    .expect("failed to write");
            }
            actual_requests
        });
        let obs = init_without_server(port);
        (obs, handle)
    }

    fn request_test<T, U>(requests: Vec<Value>, responses: Vec<Value>, expected: T, method: U)
    where
        T: PartialEq + std::fmt::Debug,
        U: Fn(&mut Obs) -> Result<T>,
    {
        let _ = env_logger::builder().is_test(true).try_init();
        let (mut obs, handle) = init(responses);
        let res = method(&mut obs).unwrap();
        let actual_requests = handle.join().unwrap();
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
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "version": 1.1,
            "obs-websocket-version": "4.7.0",
            "obs-studio-version": "24.0.3",
            "available-requests": "Request1,Request2"
        });
        let expected = responses::GetVersion {
            version: 1.1,
            obs_websocket_version: "4.7.0".to_string(),
            obs_studio_version: "24.0.3".to_string(),
            available_requests: vec!["Request1".to_string(), "Request2".to_string()],
        };
        let method = |obs: &mut Obs| obs.get_version();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_auth_required_true() {
        let request = json!({
            "request-type": "GetAuthRequired",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "authRequired": true,
            "challenge": "ch",
            "salt": "sa",
        });
        let expected = responses::GetAuthRequired {
            auth_required: true,
            challenge: Some("ch".to_string()),
            salt: Some("sa".to_string()),
        };
        let method = |obs: &mut Obs| obs.get_auth_required();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_auth_required_false() {
        let request = json!({
            "request-type": "GetAuthRequired",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "authRequired": false,
        });
        let expected = responses::GetAuthRequired {
            auth_required: false,
            challenge: None,
            salt: None,
        };
        let method = |obs: &mut Obs| obs.get_auth_required();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn authenticate() {
        let requests = vec![
            json!({
                "request-type": "GetAuthRequired",
                "message-id": "0",
            }),
            json!({
                "request-type": "Authenticate",
                "message-id": "0",
                "auth": "Z69J+b7C5Zj7jIXlqVp/xjp36sFSmpJpxZ41GN/UTu4=",
            }),
        ];
        let responses = vec![
            json!({
                "status": "ok",
                "message-id": "0",
                "authRequired": true,
                "challenge": "123",
                "salt": "456",
            }),
            json!({
                "status": "ok",
                "message-id": "0",
            }),
        ];
        let expected = responses::Response {
            message_id: "0".to_string(),
            status: responses::Status::Ok,
            error: None,
        };
        let method = |obs: &mut Obs| obs.authenticate("todo");
        request_test(requests, responses, expected, method);
    }

    #[test]
    fn set_heartbeat() {
        let request = json!({
            "request-type": "SetHeartbeat",
            "message-id": "0",
            "enable": true,
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            message_id: "0".to_string(),
            status: responses::Status::Ok,
            error: None,
        };
        let method = |obs: &mut Obs| obs.set_heartbeat(true);
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn set_filename_formatting() {
        let request = json!({
            "request-type": "SetFilenameFormatting",
            "message-id": "0",
            "filename-formatting": "test",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            message_id: "0".to_string(),
            status: responses::Status::Ok,
            error: None,
        };
        let method = |obs: &mut Obs| obs.set_filename_formatting("test");
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_filename_formatting() {
        let request = json!({
            "request-type": "GetFilenameFormatting",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "filename-formatting": "test",
        });
        let expected = responses::GetFilenameFormatting {
            filename_formatting: "test".to_string(),
        };
        let method = |obs: &mut Obs| obs.get_filename_formatting();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_stats() {
        let request = json!({
            "request-type": "GetStats",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
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
        let expected = responses::GetStats {
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
        let method = |obs: &mut Obs| obs.get_stats();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn broadcast_custom_message() {
        let request = json!({
            "request-type": "BroadcastCustomMessage",
            "message-id": "0",
            "realm": "test",
            "data": {
                "custom": "fields",
            },
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            message_id: "0".to_string(),
            status: responses::Status::Ok,
            error: None,
        };
        let method = |obs: &mut Obs| {
            let data = json!({
                "custom": "fields",
            });
            obs.broadcast_custom_message("test", data)
        };
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_video_info() {
        let request = json!({
            "request-type": "GetVideoInfo",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
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
        let method = |obs: &mut Obs| obs.get_video_info();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn list_outputs() {
        let request = json!({
            "request-type": "ListOutputs",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
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
                settings: std::collections::HashMap::new(),
                active: false,
                reconnecting: false,
                congestion: 2.0,
                total_frames: 3,
                dropped_frames: 4,
                total_bytes: 5,
            }],
        };
        let method = |obs: &mut Obs| obs.list_outputs();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_output_info() {
        let request = json!({
            "request-type": "GetOutputInfo",
            "message-id": "0",
            "outputName": "output1",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
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
                settings: std::collections::HashMap::new(),
                active: false,
                reconnecting: false,
                congestion: 2.0,
                total_frames: 3,
                dropped_frames: 4,
                total_bytes: 5,
            },
        };
        let method = |obs: &mut Obs| obs.get_output_info("output1");
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn start_output() {
        let request = json!({
            "request-type": "StartOutput",
            "message-id": "0",
            "outputName": "output1",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.start_output("output1");
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn stop_output() {
        let request = json!({
            "request-type": "StopOutput",
            "message-id": "0",
            "outputName": "output1",
            "force": false,
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.stop_output("output1", false);
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn set_current_profile() {
        let request = json!({
            "request-type": "SetCurrentProfile",
            "message-id": "0",
            "profile-name": "p",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.set_current_profile("p");
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_current_profile() {
        let request = json!({
            "request-type": "GetCurrentProfile",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "profile-name": "p",
        });
        let expected = responses::Profile {
            profile_name: "p".to_string(),
        };
        let method = |obs: &mut Obs| obs.get_current_profile();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn list_profiles() {
        let request = json!({
            "request-type": "ListProfiles",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "profiles": [
                {
                    "profile-name": "p1",
                },
                {
                    "profile-name": "p2",
                }
            ],
        });
        let expected = responses::ListProfiles {
            profiles: vec![
                responses::Profile {
                    profile_name: "p1".to_string(),
                },
                responses::Profile {
                    profile_name: "p2".to_string(),
                },
            ],
        };
        let method = |obs: &mut Obs| obs.list_profiles();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn toggle_recording() {
        let request = json!({
            "request-type": "StartStopRecording",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.toggle_recording();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn start_recording() {
        let request = json!({
            "request-type": "StartRecording",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.start_recording();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn stop_recording() {
        let request = json!({
            "request-type": "StopRecording",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.stop_recording();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn pause_recording() {
        let request = json!({
            "request-type": "PauseRecording",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.pause_recording();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn resume_recording() {
        let request = json!({
            "request-type": "ResumeRecording",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.resume_recording();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn set_recording_folder() {
        let request = json!({
            "request-type": "SetRecordingFolder",
            "message-id": "0",
            "rec-folder": "path",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.set_recording_folder("path");
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_recording_folder() {
        let request = json!({
            "request-type": "GetRecordingFolder",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "rec-folder": "path",
        });
        let expected = responses::GetRecordingFolder {
            rec_folder: "path".to_string(),
        };
        let method = |obs: &mut Obs| obs.get_recording_folder();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn toggle_replay_buffer() {
        let request = json!({
            "request-type": "StartStopReplayBuffer",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.toggle_replay_buffer();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn start_replay_buffer() {
        let request = json!({
            "request-type": "StartReplayBuffer",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.start_replay_buffer();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn stop_replay_buffer() {
        let request = json!({
            "request-type": "StopReplayBuffer",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.stop_replay_buffer();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn save_replay_buffer() {
        let request = json!({
            "request-type": "SaveReplayBuffer",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.save_replay_buffer();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn set_current_scene_collection() {
        let request = json!({
            "request-type": "SetCurrentSceneCollection",
            "message-id": "0",
            "sc-name": "scene",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        let expected = responses::Response {
            status: responses::Status::Ok,
            message_id: "0".to_string(),
            error: None,
        };
        let method = |obs: &mut Obs| obs.set_current_scene_collection("scene");
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_current_scene_collection() {
        let request = json!({
            "request-type": "GetCurrentSceneCollection",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "sc-name": "scene",
        });
        let expected = responses::SceneCollection {
            sc_name: "scene".to_string(),
        };
        let method = |obs: &mut Obs| obs.get_current_scene_collection();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn list_scene_collections() {
        let request = json!({
            "request-type": "ListSceneCollections",
            "message-id": "0",
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
            "scene-collections": [
                {
                    "sc-name": "scene1",
                },
                {
                    "sc-name": "scene2",
                }
            ],
        });
        let expected = responses::ListSceneCollections {
            scene_collections: vec![
                responses::SceneCollection {
                    sc_name: "scene1".to_string(),
                },
                responses::SceneCollection {
                    sc_name: "scene2".to_string(),
                },
            ],
        };
        let method = |obs: &mut Obs| obs.list_scene_collections();
        request_test(vec![request], vec![response], expected, method);
    }

    #[test]
    fn get_scene_item_properties() {
        let request = json!({
            "request-type": "GetSceneItemProperties",
            "message-id": "0",
            "scene-name": "scene",
            "item": "source"
        });
        let response = json!({
            "status": "ok",
            "message-id": "0",
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
        let expected = responses::GetSceneItemProperties {
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
        let method = |obs: &mut Obs| obs.get_scene_item_properties(Some("scene"), "source");
        request_test(vec![request], vec![response], expected, method);
    }
}
