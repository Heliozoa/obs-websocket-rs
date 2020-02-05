use super::responses;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn get_version(message_id: &str) -> Value {
    json!({
        "request-type": "GetVersion",
        "message-id": message_id,
    })
}

pub fn get_auth_required(message_id: &str) -> Value {
    json!({
        "request-type": "GetAuthRequired",
        "message-id": message_id,
    })
}

pub fn authenticate(message_id: &str, auth: &str) -> Value {
    json!({
        "request-type": "Authenticate",
        "message-id": message_id,
        "auth": auth,
    })
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

pub fn get_stats(message_id: &str) -> Value {
    json!({
        "request-type": "GetStats",
        "message-id": message_id,
    })
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

pub fn list_outputs(message_id: &str) -> Value {
    json!({
        "request-type": "ListOutputs",
        "message-id": message_id,
    })
}

pub fn get_output_info(message_id: &str, output_name: &str) -> Value {
    json!({
        "request-type": "GetOutputInfo",
        "message-id": message_id,
        "outputName": output_name,
    })
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

pub fn list_profiles(message_id: &str) -> Value {
    json!({
        "request-type": "ListProfiles",
        "message-id": message_id,
    })
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

pub fn get_recording_folder(message_id: &str) -> Value {
    json!({
        "request-type": "GetRecordingFolder",
        "message-id": message_id,
    })
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

pub fn list_scene_collections(message_id: &str) -> Value {
    json!({
        "request-type": "ListSceneCollections",
        "message-id": message_id,
    })
}

pub fn get_scene_item_properties(message_id: &str, scene_name: Option<&str>, item: &str) -> Value {
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

#[derive(Serialize, Debug, PartialEq)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alignment: Option<i32>,
}

impl Position {
    pub fn is_none(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.alignment.is_none()
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Scale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

impl Scale {
    pub fn is_none(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Crop {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
}

impl Crop {
    pub fn is_none(&self) -> bool {
        self.top.is_none() && self.right.is_none() && self.bottom.is_none() && self.left.is_none()
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BoundsType {
    #[serde(rename = "OBS_BOUNDS_NONE")]
    None,
    #[serde(rename = "OBS_BOUNDS_STRETCH")]
    Stretch,
    #[serde(rename = "OBS_BOUNDS_SCALE_INNER")]
    ScaleInner,
    #[serde(rename = "OBS_BOUNDS_SCALE_OUTER")]
    ScaleOuter,
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_WIDTH")]
    ScaleToWidth,
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_HEIGHT")]
    ScaleToHeight,
    #[serde(rename = "OBS_BOUNDS_MAX_ONLY")]
    MaxOnly,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Bounds {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds_type: Option<BoundsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

impl Bounds {
    pub fn is_none(&self) -> bool {
        self.bounds_type.is_none()
            && self.alignment.is_none()
            && self.x.is_none()
            && self.y.is_none()
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct SetSceneItemPropertiesRequest {
    request_type: String,
    message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene_name: Option<String>,
    item: String,
    #[serde(skip_serializing_if = "Position::is_none")]
    position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<f64>,
    #[serde(skip_serializing_if = "Scale::is_none")]
    scale: Scale,
    #[serde(skip_serializing_if = "Crop::is_none")]
    crop: Crop,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(skip_serializing_if = "Bounds::is_none")]
    bounds: Bounds,
}

pub fn set_scene_item_properties(
    message_id: &str,
    scene_name: Option<String>,
    item: String,
    position: Position,
    rotation: Option<f64>,
    scale: Scale,
    crop: Crop,
    visible: Option<bool>,
    locked: Option<bool>,
    bounds: Bounds,
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

pub fn reset_scene_item(scene_name: Option<String>, item: String) -> Value {
    json!({"scene-name": scene_name, "item": item})
}
