use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Request<'a> {
    request_type: &'a str,
    message_id: &'a str,
}

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
    x: Option<f64>,
    y: Option<f64>,
    alignment: Option<i32>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Scale {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Crop {
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
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
    pub bounds_type: Option<BoundsType>,
    pub alignment: Option<i32>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct SetSceneItemPropertiesRequest {
    request_type: String,
    message_id: String,
    scene_name: Option<String>,
    item: String,
    position: Position,
    rotation: Option<f64>,
    scale: Scale,
    crop: Crop,
    visible: Option<bool>,
    locked: Option<bool>,
    bounds: Bounds,
}

pub fn set_scene_item_properties(
    message_id: &str,
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
    bounds_type: Option<BoundsType>,
    bounds_alignment: Option<i32>,
    bounds_x: Option<f64>,
    bounds_y: Option<f64>,
) -> Value {
    let req = SetSceneItemPropertiesRequest {
        request_type: "SetSceneItemProperties".to_string(),
        message_id: message_id.to_string(),
        scene_name: scene_name.map(|s| s.to_string()),
        item: item.to_string(),
        position: Position {
            x: position_x,
            y: position_y,
            alignment: position_alignment,
        },
        rotation,
        scale: Scale {
            x: scale_x,
            y: scale_y,
        },
        crop: Crop {
            top: crop_top,
            right: crop_right,
            bottom: crop_bottom,
            left: crop_left,
        },
        visible,
        locked,
        bounds: Bounds {
            bounds_type,
            alignment: bounds_alignment,
            x: bounds_x,
            y: bounds_y,
        },
    };
    let v = serde_json::to_value(&req).unwrap();
    v
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct StreamSettings {
    server: String,
    key: String,
    use_auth: bool,
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct Stream {
    #[serde(rename = "type")]
    stream_type: Option<String>,
    metadata: Option<Value>,
    settings: StreamSettings,
}

#[derive(Deserialize, Debug)]
pub struct WithTransition {
    name: String,
    duration: Option<i32>,
}

pub fn reset_scene_item(message_id: &str, scene_name: Option<&str>, item: &str) -> Value {
    json!({
        "request-type": "ResetSceneItem",
        "message-id": message_id,
        "scene-name": scene_name,
        "item": item,
    })
}

pub fn delete_scene_item(
    message_id: &str,
    scene_name: Option<&str>,
    item_name: &str,
    item_id: i32,
) -> Value {
    json!({
        "request-type": "DeleteSceneItem",
        "message-id": message_id,
        "scene": scene_name,
        "item": {
            "name": item_name,
            "id": item_id,
        },
    })
}
