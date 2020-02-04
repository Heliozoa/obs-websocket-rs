mod error;
mod events;
mod requests;
mod typedefs;

use base64;
use error::Error;
use log::{debug, info};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tungstenite::{client::AutoStream, connect, protocol::WebSocket, Message};
use url::Url;

type Result<T> = std::result::Result<T, error::Error>;

pub struct Obs {
    socket: Option<WebSocket<AutoStream>>,
}

impl Obs {
    pub fn new() -> Self {
        Obs { socket: None }
    }

    pub fn connect(&mut self, port: u16) -> Result<()> {
        let address = format!("ws://localhost:{}", port.to_string());
        let (socket, _response) = connect(Url::parse(&address)?)?;
        self.socket = Some(socket);
        Ok(())
    }

    fn get<T: DeserializeOwned>(&mut self, json: Value) -> Result<T> {
        if let None = self.socket {
            return Err(Error::Custom("not connected".to_string()));
        }
        let socket = self.socket.as_mut().unwrap();
        let json = json.to_string();
        println!("SENT: {}", json);
        socket.write_message(Message::Text(json))?;
        let response = socket.read_message()?.to_string();
        println!("RECV: {}", response);
        let parsed: requests::Response = serde_json::from_str(&response)?;
        if let requests::Status::Ok = parsed.status {
            Ok(serde_json::from_str(&response)?)
        } else {
            let error_msg = parsed.error.unwrap();
            Err(Error::ObsError(error_msg))
        }
    }

    pub fn get_version(&mut self) -> Result<requests::GetVersion> {
        self.get(requests::get_version("0"))
    }

    pub fn get_auth_required(&mut self) -> Result<requests::GetAuthRequired> {
        self.get(requests::get_auth_required("0"))
    }

    pub fn authenticate(&mut self, password: &str) -> Result<requests::Response> {
        let auth: requests::GetAuthRequired = self.get(requests::get_auth_required("0"))?;
        if auth.auth_required {
            let challenge = auth.challenge.unwrap();
            let salt = auth.salt.unwrap();

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            Ok(self.get(requests::authenticate("0", &auth_response))?)
        } else {
            Err(Error::ObsError("no auth required".to_string()))
        }
    }

    pub fn set_heartbeat(&mut self, enable: bool) -> Result<requests::Response> {
        self.get(requests::set_heartbeat("0", enable))
    }

    pub fn set_filename_formatting(
        &mut self,
        filename_formatting: &str,
    ) -> Result<requests::Response> {
        self.get(requests::set_filename_formatting("0", filename_formatting))
    }

    pub fn get_filename_formatting(&mut self) -> Result<requests::GetFilenameFormatting> {
        self.get(requests::get_filename_formatting("0"))
    }

    pub fn get_stats(&mut self) -> Result<requests::GetStats> {
        self.get(requests::get_stats("0"))
    }

    pub fn broadcast_custom_message(
        &mut self,
        realm: &str,
        data: Value,
    ) -> Result<requests::Response> {
        self.get(requests::broadcast_custom_message("0", realm, data))
    }

    pub fn get_video_info(&mut self) -> Result<requests::GetVideoInfo> {
        self.get(requests::get_video_info("0"))
    }

    pub fn list_outputs(&mut self) -> Result<requests::ListOutputs> {
        self.get(requests::list_outputs("0"))
    }

    pub fn get_output_info(&mut self, output_name: &str) -> Result<requests::GetOutputInfo> {
        self.get(requests::get_output_info("0", output_name))
    }

    pub fn start_output(&mut self, output_name: &str) -> Result<requests::Response> {
        self.get(requests::start_output("0", output_name))
    }

    pub fn stop_output(&mut self, output_name: &str, force: bool) -> Result<requests::Response> {
        self.get(requests::stop_output("0", output_name, force))
    }

    pub fn set_current_profile(&mut self, profile_name: &str) -> Result<requests::Response> {
        self.get(requests::set_current_profile("0", profile_name))
    }

    pub fn get_current_profile(&mut self) -> Result<requests::GetCurrentProfile> {
        self.get(requests::get_current_profile("0"))
    }

    pub fn list_profiles(&mut self) -> Result<requests::ListProfiles> {
        self.get(requests::list_profiles("0"))
    }

    pub fn toggle_recording(&mut self) -> Result<requests::Response> {
        self.get(requests::start_stop_recording("0"))
    }

    pub fn start_recording(&mut self) -> Result<requests::Response> {
        self.get(requests::start_recording("0"))
    }

    pub fn stop_recording(&mut self) -> Result<requests::Response> {
        self.get(requests::stop_recording("0"))
    }

    pub fn pause_recording(&mut self) -> Result<requests::Response> {
        self.get(requests::pause_recording("0"))
    }

    pub fn resume_recording(&mut self) -> Result<requests::Response> {
        self.get(requests::resume_recording("0"))
    }

    pub fn set_recording_folder(&mut self, rec_folder: &str) -> Result<requests::Response> {
        self.get(requests::set_recording_folder("0", rec_folder))
    }

    pub fn get_recording_folder(&mut self, rec_folder: &str) -> Result<requests::Response> {
        self.get(requests::get_recording_folder("0", rec_folder))
    }

    pub fn toggle_replay_buffer(&mut self) -> Result<requests::Response> {
        self.get(requests::start_stop_replay_buffer("0"))
    }

    pub fn start_replay_buffer(&mut self) -> Result<requests::Response> {
        self.get(requests::start_replay_buffer("0"))
    }

    pub fn stop_replay_buffer(&mut self) -> Result<requests::Response> {
        self.get(requests::stop_replay_buffer("0"))
    }

    pub fn save_replay_buffer(&mut self) -> Result<requests::Response> {
        self.get(requests::save_replay_buffer("0"))
    }

    pub fn set_current_scene_collection(&mut self, sc_name: &str) -> Result<requests::Response> {
        self.get(requests::set_current_scene_collection("0", sc_name))
    }

    pub fn get_current_scene_collection(&mut self) -> Result<requests::GetCurrentSceneCollection> {
        self.get(requests::get_current_scene_collection("0"))
    }

    pub fn list_current_scene_collection(&mut self) -> Result<requests::ListSceneCollections> {
        self.get(requests::list_scene_collections("0"))
    }

    pub fn get_scene_item_properties(
        &mut self,
        scene_name: Option<String>,
        item: String,
    ) -> Result<requests::GetSceneItemProperties> {
        self.get(requests::get_scene_item_properties("0", scene_name, item))
    }

    pub fn set_scene_item_properties(
        &mut self,
        scene_name: Option<String>,
        item: String,
        position: typedefs::Position,
        rotation: Option<f64>,
        scale: typedefs::Scale,
        crop: typedefs::Crop,
        visible: Option<bool>,
        locked: Option<bool>,
        bounds: typedefs::Bounds,
    ) -> Result<requests::SetSceneItemProperties> {
        self.get(requests::set_scene_item_properties(
            "0", scene_name, item, position, rotation, scale, crop, visible, locked, bounds,
        ))
    }

    pub fn reset_scene_item(
        &mut self,
        scene_name: Option<String>,
        item: String,
    ) -> Result<requests::Response> {
        self.get(requests::reset_scene_item(scene_name, item))
    }

    pub fn delete_scene_item(
        &mut self,
        scene: Option<String>,
        item: typedefs::Item,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn duplicate_scene_item(
        &mut self,
        from_scene: Option<String>,
        to_scene: Option<String>,
        item: typedefs::Item,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_current_scene(&mut self, scene_name: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_current_scene(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_scene_list(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn reorder_scene_items(
        &mut self,
        scene: Option<String>,
        items: Vec<typedefs::Item>,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_sources_list(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_source_types_list(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_volume(&mut self, source: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_volume(&mut self, source: String, volume: f64) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_mute(&mut self, source: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_mute(&mut self, source: String, mute: bool) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn toggle_mute(&mut self, source: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_sync_offset(&mut self, source: String, offset: i32) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_sync_offset(&mut self, source: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_source_settings(
        &mut self,
        source_name: String,
        source_type: Option<String>,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_source_settings(
        &mut self,
        source_name: String,
        source_type: Option<String>,
        source_settings: Value,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_text_gdi_plus_properties(&mut self, source: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_text_gdi_plus_properties(
        &mut self,
        source: String,
        align: typedefs::Align,
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
        font: typedefs::Font,
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
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_text_freetype_2_properties(&mut self, source: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_text_freetype_2_properties(
        &mut self,
        source: String,
        color_1: Option<i32>,
        color_2: Option<i32>,
        custom_width: Option<i32>,
        drop_shadow: Option<i32>,
        font: typedefs::Font,
        from_file: Option<bool>,
        log_mode: Option<bool>,
        outline: Option<bool>,
        text: Option<String>,
        text_file: Option<String>,
        word_wrap: Option<bool>,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_browser_source_properties(&mut self, source: String) -> Result<requests::Response> {
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
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_special_sources(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_source_filters(&mut self, source_name: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_source_filter_info(
        &mut self,
        source_name: String,
        filter_name: String,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn add_filter_to_source(
        &mut self,
        source_name: String,
        filter_name: String,
        filter_type: String,
        filter_settings: Value,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn remove_filter_from_source(
        &mut self,
        source_name: String,
        filter_name: String,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn reorder_source_filter(
        &mut self,
        source_name: String,
        filter_name: String,
        new_index: i32,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn move_source_filter(
        &mut self,
        source_name: String,
        filter_name: String,
        movement_type: typedefs::MovementType,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_source_filter_settings(
        &mut self,
        source_name: String,
        filter_name: String,
        filter_settings: Value,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_source_filter_visibility(
        &mut self,
        source_name: String,
        filter_name: String,
        filter_enabled: bool,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn take_source_screenshot(
        &mut self,
        source_name: String,
        embed_picture_format: Option<String>,
        save_to_file_path: Option<String>,
        width: Option<i32>,
        height: Option<i32>,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_streaming_status(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn toggle_streaming(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn start_streaming(&mut self, stream: typedefs::Stream) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn stop_streaming(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_stream_settings(
        &mut self,
        stream_type: String,
        settings: typedefs::StreamSettings,
        save: bool,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_stream_settings(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn save_stream_settings(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn send_captions(&mut self, text: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_studio_mode_status(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_preview_scene(
        &mut self,
        name: String,
        sources: Vec<typedefs::SceneItem>,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_preview_scene(&mut self, scene_name: String) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn transition_to_program(
        &mut self,
        with_transition: Option<typedefs::WithTransition>,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn enable_studio_mode(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn disable_studio_mode(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn toggle_studio_mode(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_transition_list(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_current_transition(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_current_transition(
        &mut self,
        transition_name: String,
    ) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn set_transition_duration(&mut self, duration: i32) -> Result<requests::Response> {
        unimplemented!()
    }

    pub fn get_transition_duration(&mut self) -> Result<requests::Response> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::net::TcpListener;
    use std::thread::{spawn, JoinHandle};
    use tungstenite::server::accept;

    fn init_obs(port: u16) -> Obs {
        env_logger::init();
        let mut obs = Obs::new();
        obs.connect(port).unwrap();
        obs
    }

    fn start_mock_server(requests: Vec<Value>, responses: Vec<Value>) -> JoinHandle<()> {
        let handle = spawn(move || {
            let mut requests = requests.iter().cycle();
            let mut responses = responses.iter().cycle();
            let server = TcpListener::bind("localhost:4445").unwrap();
            for stream in server.incoming() {
                let mut websocket = accept(stream.unwrap()).unwrap();
                let msg = websocket.read_message().unwrap().to_string();
                let parsed = serde_json::from_str::<Value>(&msg).unwrap();
                assert_eq!(&parsed, requests.next().unwrap());
                websocket
                    .write_message(Message::Text(responses.next().unwrap().to_string()))
                    .unwrap();
            }
        });
        handle
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
        start_mock_server(vec![request], vec![response]);
        let mut obs = init_obs(4445);
        let res = obs.get_version().unwrap();
        assert_eq!(
            res,
            requests::GetVersion {
                version: 1.1,
                obs_websocket_version: "4.7.0".to_string(),
                obs_studio_version: "24.0.3".to_string(),
                available_requests: vec!["Request1".to_string(), "Request2".to_string()],
            }
        );
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
        start_mock_server(vec![request], vec![response]);
        let mut obs = init_obs(4445);
        let res = obs.get_auth_required().unwrap();
        assert_eq!(
            res,
            requests::GetAuthRequired {
                auth_required: true,
                challenge: Some("ch".to_string()),
                salt: Some("sa".to_string())
            }
        );
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
        start_mock_server(vec![request], vec![response]);
        let mut obs = init_obs(4445);
        let res = obs.get_auth_required().unwrap();
        assert_eq!(
            res,
            requests::GetAuthRequired {
                auth_required: false,
                challenge: None,
                salt: None,
            }
        );
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
        let handle = start_mock_server(requests, responses);
        let mut obs = init_obs(4445);
        let res = obs.authenticate("todo");
        let res = res.unwrap();
        handle.join().unwrap();
        assert_eq!(
            res,
            requests::Response {
                message_id: "0".to_string(),
                status: requests::Status::Ok,
                error: None,
            }
        );
    }

    #[test]
    fn set_heartbeat() {
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        start_mock_server(vec![], vec![response]);
        let mut obs = init_obs(4445);
        let res = obs.set_heartbeat(true).unwrap();
        assert_eq!(
            res,
            requests::Response {
                message_id: "0".to_string(),
                status: requests::Status::Ok,
                error: None,
            }
        );
    }

    #[test]
    fn set_filename_formatting() {
        let response = json!({
            "status": "ok",
            "message-id": "0",
        });
        start_mock_server(vec![], vec![response]);
        let mut obs = init_obs(4445);
        let res = obs.set_filename_formatting("test").unwrap();
        assert_eq!(
            res,
            requests::Response {
                message_id: "0".to_string(),
                status: requests::Status::Ok,
                error: None,
            }
        );
    }
}
