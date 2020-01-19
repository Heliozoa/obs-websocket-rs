use base64;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tungstenite::{client::AutoStream, connect, protocol::WebSocket, Message};
use url::Url;

mod events;
mod requests;
mod typedefs;

type Result<T> = std::result::Result<T, String>;

pub struct Obs {
    socket: Option<WebSocket<AutoStream>>,
}

impl Obs {
    pub fn new() -> Self {
        Obs { socket: None }
    }

    pub fn connect(&mut self) {
        let (socket, _response) =
            connect(Url::parse("ws://localhost:4444").unwrap()).expect("Can't connect");
        self.socket = Some(socket);
    }

    fn get<T: DeserializeOwned>(&mut self, json: Value) -> Result<T> {
        let socket = self.socket.as_mut().unwrap();
        let json = json.to_string();
        println!("SENT: {}", json);
        socket.write_message(Message::Text(json)).unwrap();
        let response = socket
            .read_message()
            .expect("Error reading message")
            .to_string();
        println!("RECV: {}", response);
        let parsed: requests::Response = serde_json::from_str(&response).unwrap();
        if let requests::Status::Ok = parsed.status {
            Ok(serde_json::from_str(&response).unwrap())
        } else {
            Err(parsed.error.unwrap())
        }
    }

    pub fn get_version(&mut self) -> Result<requests::GetVersion> {
        self.get(requests::get_version("0"))
    }

    pub fn get_auth_required(&mut self) -> Result<requests::GetAuthRequired> {
        self.get(requests::get_auth_required("0"))
    }

    pub fn authenticate(&mut self) -> Result<requests::Response> {
        let auth: requests::GetAuthRequired = self.get(requests::get_auth_required("0")).unwrap();
        if auth.auth_required {
            let password = "todo";
            let challenge = auth.challenge.unwrap();
            let salt = auth.salt.unwrap();

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            Ok(self
                .get(requests::authenticate("0", &auth_response))
                .unwrap())
        } else {
            Err("no auth required".to_string())
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
        data: HashMap<String, String>,
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

    pub fn stop_output(&mut self, output_name: &str) -> Result<requests::Response> {
        self.get(requests::stop_output("0", output_name))
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

    pub fn set_current_scene_collection(&mut self, rec_folder: &str) -> Result<requests::Response> {
        self.get(requests::set_current_scene_collection("0", rec_folder))
    }

    pub fn get_current_scene_collection(&mut self) -> Result<requests::Response> {
        self.get(requests::get_current_scene_collection("0"))
    }

    pub fn list_current_scene_collection(&mut self) -> Result<requests::Response> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_version() {
        let mut o = Obs::new();
        o.connect();
        o.get_version().unwrap();
    }

    #[test]
    fn get_auth_required() {
        let mut o = Obs::new();
        o.connect();
        o.get_auth_required().unwrap();
    }

    #[test]
    fn authenticate() {
        let mut o = Obs::new();
        o.connect();
        o.authenticate().unwrap();
    }

    #[test]
    fn list_profiles() {
        let mut o = Obs::new();
        o.connect();
        let res = o.list_profiles().unwrap();
        println!("{:?}", res);
        unimplemented!()
    }
}
