use super::typedefs;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GetVersion {
    version: f64,
    obs_websocket_version: String,
    obs_studio_version: String,
    available_requests: String,
}

pub fn get_auth_required(message_id: &str) -> Value {
    json!({
        "request-type": "GetAuthRequired",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct SetHeartbeat {}

pub fn set_filename_formatting(message_id: &str, filename_formatting: &str) -> Value {
    json!({
        "request-type": "SetFilenameFormatting",
        "message-id": message_id,
        "filename-formatting": filename_formatting,
    })
}

#[derive(Deserialize, Debug)]
pub struct SetFilenameFormatting {}

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

pub fn broadcast_custom_message(
    message_id: &str,
    realm: String,
    data: HashMap<String, String>,
) -> Value {
    json!({
        "request-type": "BroadcastCustomMessage",
        "message-id": message_id,
        "realm": realm,
        "data": data,
    })
}

#[derive(Deserialize, Debug)]
pub struct BroadcastCustomMessage {}

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
    scale_type: i32,
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

#[derive(Deserialize, Debug)]
pub struct StartOutput {}

pub fn stop_output(message_id: &str, output_name: &str) -> Value {
    json!({
        "request-type": "StopOutput",
        "message-id": message_id,
        "outputName": output_name,
    })
}

#[derive(Deserialize, Debug)]
pub struct StopOutput {}

pub fn set_current_profile(message_id: &str, profile_name: &str) -> Value {
    json!({
        "request-type": "SetCurrentProfile",
        "message-id": message_id,
        "profile-name": profile_name,
    })
}

#[derive(Deserialize, Debug)]
pub struct SetCurrentProfile {}

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

#[derive(Deserialize, Debug)]
pub struct StartStopRecording {}

pub fn start_recording(message_id: &str) -> Value {
    json!({
        "request-type": "StartRecording",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct StartRecording {}

pub fn stop_recording(message_id: &str) -> Value {
    json!({
        "request-type": "StopRecording",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct StopRecording {}

pub fn pause_recording(message_id: &str) -> Value {
    json!({
        "request-type": "PauseRecording",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct PauseRecording {}

pub fn resume_recording(message_id: &str) -> Value {
    json!({
        "request-type": "ResumeRecording",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct ResumeRecording {}

pub fn set_recording_folder(message_id: &str, rec_folder: &str) -> Value {
    json!({
        "request-type": "SetRecordingFolder",
        "message-id": message_id,
        "rec-folder": rec_folder,
    })
}

#[derive(Deserialize, Debug)]
pub struct SetRecordingFolder {}

pub fn get_recording_folder(message_id: &str, rec_folder: &str) -> Value {
    json!({
        "request-type": "GetRecordingFolder",
        "message-id": message_id,
        "rec-folder": rec_folder,
    })
}

#[derive(Deserialize, Debug)]
pub struct GetRecordingFolder {}

pub fn start_stop_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "StartStopReplayBuffer",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct StartStopReplayBuffer {}

pub fn start_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "StartReplayBuffer",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct StartReplayBuffer {}

pub fn stop_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "StopReplayBuffer",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct StopReplayBuffer {}

pub fn save_replay_buffer(message_id: &str) -> Value {
    json!({
        "request-type": "SaveReplayBuffer",
        "message-id": message_id,
    })
}

#[derive(Deserialize, Debug)]
pub struct SaveReplayBuffer {}

pub fn set_current_scene_collection(message_id: &str, sc_name: &str) -> Value {
    json!({
        "request-type": "SetCurrentSceneCollection",
        "message-id": message_id,
        "sc-name": sc_name,
    })
}

#[derive(Deserialize, Debug)]
pub struct SetCurrentSceneCollection {}

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
        let v = json!({
            "request-type": "GetSceneItemProperties",
            "message-id": message_id,
            "item": item,
        });
        v
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GetSceneItemProperties {
    scene_collections: Vec<String>,
}
