use super::typedefs;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "message-id")]
    message_id: String,
    pub status: Status,
    pub error: Option<String>,
}

pub fn get_version(message_id: &str) -> Value {
    json!({
        "request-type": "GetVersion",
        "message-id": message_id,
    })
}

fn deserialize_comma_separated_string<'de, D>(d: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct V {}

    impl<'de> de::Visitor<'de> for V {
        type Value = Vec<String>;

        fn expecting(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            unreachable!()
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(s.split(',').map(|s| s.to_owned()).collect::<Vec<_>>())
        }
    }

    d.deserialize_str(V {})
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetVersion {
    version: f64,
    obs_websocket_version: String,
    obs_studio_version: String,
    #[serde(deserialize_with = "deserialize_comma_separated_string")]
    available_requests: Vec<String>,
}

pub fn get_auth_required(message_id: &str) -> Value {
    json!({
        "request-type": "GetAuthRequired",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthRequired {
    pub auth_required: bool,
    pub challenge: Option<String>,
    pub salt: Option<String>,
}

pub fn authenticate(message_id: &str, auth: &str) -> Value {
    json!({
        "request-type": "Authenticate",
        "message-id": message_id,
        "auth": auth,
    })
}

#[derive(Deserialize, Debug)]
pub struct Authenticate {
    auth: String,
}

pub fn set_heartbeat(message_id: &str, enable: bool) -> Value {
    json!({
        "request-type": "SetHeartbeat",
        "message-id": message_id,
        "enable": enable,
    })
}

pub fn set_filename_formatting(message_id: &str, filename_formatting: &str) -> Value {
    json!({
        "request-type": "SetFilenameFormatting",
        "message-id": message_id,
        "filename-formatting": filename_formatting,
    })
}

pub fn get_filename_formatting(message_id: &str) -> Value {
    json!({
        "request-type": "GetFilenameFormatting",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GetFilenameFormatting {
    filename_formatting: String,
}

pub fn get_stats(message_id: &str) -> Value {
    json!({
        "request-type": "GetStats",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct GetStats {
    stats: typedefs::ObsStats,
}

pub fn broadcast_custom_message(message_id: &str, realm: &str, data: Value) -> Value {
    json!({
        "request-type": "BroadcastCustomMessage",
        "message-id": message_id,
        "realm": realm,
        "data": data,
    })
}

pub fn get_video_info(message_id: &str) -> Value {
    json!({
        "request-type": "GetVideoInfo",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVideoInfo {
    base_width: i32,
    base_height: i32,
    output_width: i32,
    output_height: i32,
    scale_type: String,
    fps: f64,
    video_format: String,
    color_space: String,
    color_range: String,
}

pub fn list_outputs(message_id: &str) -> Value {
    json!({
        "request-type": "ListOutputs",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct ListOutputs {
    outputs: Vec<typedefs::Output>,
}

pub fn get_output_info(message_id: &str, output_name: &str) -> Value {
    json!({
        "request-type": "GetOutputInfo",
        "message-id": message_id,
        "outputName": output_name,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOutputInfo {
    output_info: typedefs::Output,
}

pub fn start_output(message_id: &str, output_name: &str) -> Value {
    json!({
        "request-type": "StartOutput",
        "message-id": message_id,
        "outputName": output_name,
    })
}

pub fn stop_output(message_id: &str, output_name: &str, force: bool) -> Value {
    json!({
        "request-type": "StopOutput",
        "message-id": message_id,
        "outputName": output_name,
        "force": force,
    })
}

pub fn set_current_profile(message_id: &str, profile_name: &str) -> Value {
    json!({
        "request-type": "SetCurrentProfile",
        "message-id": message_id,
        "profile-name": profile_name,
    })
}

pub fn get_current_profile(message_id: &str) -> Value {
    json!({
        "request-type": "GetCurrentProfile",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GetCurrentProfile {
    profile_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Profile {
    profile_name: String,
}

pub fn list_profiles(message_id: &str) -> Value {
    json!({
        "request-type": "ListProfiles",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ListProfiles {
    profiles: Vec<Profile>,
}

pub fn start_stop_recording(message_id: &str) -> Value {
    json!({
        "request-type": "StartStopRecording",
        "message-id": message_id,
    })
}

pub fn start_recording(message_id: &str) -> Value {
    json!({
        "request-type": "StartRecording",
        "message-id": message_id,
    })
}

pub fn stop_recording(message_id: &str) -> Value {
    json!({
        "request-type": "StopRecording",
        "message-id": message_id,
    })
}

pub fn pause_recording(message_id: &str) -> Value {
    json!({
        "request-type": "PauseRecording",
        "message-id": message_id,
    })
}

pub fn resume_recording(message_id: &str) -> Value {
    json!({
        "request-type": "ResumeRecording",
        "message-id": message_id,
    })
}

pub fn set_recording_folder(message_id: &str, rec_folder: &str) -> Value {
    json!({
        "request-type": "SetRecordingFolder",
        "message-id": message_id,
        "rec-folder": rec_folder,
    })
}

pub fn get_recording_folder(message_id: &str, rec_folder: &str) -> Value {
    json!({
        "request-type": "GetRecordingFolder",
        "message-id": message_id,
        "rec-folder": rec_folder,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GetRecordingFolder {
    rec_folder: String,
}

pub fn start_stop_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "StartStopReplayBuffer",
        "message-id": message_id,
    })
}

pub fn start_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "StartReplayBuffer",
        "message-id": message_id,
    })
}

pub fn stop_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "StopReplayBuffer",
        "message-id": message_id,
    })
}

pub fn save_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "SaveReplayBuffer",
        "message-id": message_id,
    })
}

pub fn set_current_scene_collection(message_id: &str, sc_name: &str) -> Value {
    json!({
        "request-type": "SetCurrentSceneCollection",
        "message-id": message_id,
        "sc-name": sc_name,
    })
}

pub fn get_current_scene_collection(message_id: &str) -> Value {
    json!({
        "request-type": "GetCurrentSceneCollection",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GetCurrentSceneCollection {
    sc_name: String,
}

pub fn list_scene_collections(message_id: &str) -> Value {
    json!({
        "request-type": "ListSceneCollections",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ListSceneCollections {
    scene_collections: Vec<String>,
}

pub fn get_scene_item_properties(
    message_id: &str,
    scene_name: Option<String>,
    item: String,
) -> Value {
    if scene_name.is_some() {
        json!({
            "request-type": "GetSceneItemProperties",
            "message-id": message_id,
            "scene-name": scene_name,
            "item": item,
        })
    } else {
        json!({
            "request-type": "GetSceneItemProperties",
            "message-id": message_id,
            "item": item,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSceneItemProperties {
    name: String,
    position: typedefs::Position,
    rotation: f64,
    scale: typedefs::Scale,
    crop: typedefs::Crop,
    visible: bool,
    locked: bool,
    bounds: typedefs::Bounds,
    source_width: i32,
    source_height: i32,
    width: f64,
    height: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct SetSceneItemPropertiesRequest {
    request_type: String,
    message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene_name: Option<String>,
    item: String,
    #[serde(skip_serializing_if = "typedefs::Position::is_none")]
    position: typedefs::Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<f64>,
    #[serde(skip_serializing_if = "typedefs::Scale::is_none")]
    scale: typedefs::Scale,
    #[serde(skip_serializing_if = "typedefs::Crop::is_none")]
    crop: typedefs::Crop,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(skip_serializing_if = "typedefs::Bounds::is_none")]
    bounds: typedefs::Bounds,
}

pub fn set_scene_item_properties(
    message_id: &str,
    scene_name: Option<String>,
    item: String,
    position: typedefs::Position,
    rotation: Option<f64>,
    scale: typedefs::Scale,
    crop: typedefs::Crop,
    visible: Option<bool>,
    locked: Option<bool>,
    bounds: typedefs::Bounds,
) -> Value {
    let req = SetSceneItemPropertiesRequest {
        request_type: "SetSceneItemProperties".to_string(),
        message_id: message_id.to_string(),
        scene_name,
        item,
        position,
        rotation,
        scale,
        crop,
        visible,
        locked,
        bounds,
    };
    let v = serde_json::to_value(&req).unwrap();
    v
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemProperties {
    name: String,
    position: typedefs::Position,
    rotation: f64,
    scale: typedefs::Scale,
    crop: typedefs::Crop,
    visible: bool,
    locked: bool,
    bounds: typedefs::Bounds,
    source_width: i32,
    source_height: i32,
    width: f64,
    height: f64,
}

pub fn reset_scene_item(scene_name: Option<String>, item: String) -> Value {
    json!({"scene-name": scene_name, "item": item})
}
