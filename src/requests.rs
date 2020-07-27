//! Request types. Sent to the server

use super::responses;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use typed_builder::TypedBuilder;

static RUNNING_MESSAGE_ID: AtomicU32 = AtomicU32::new(0);

// trait that all request types must implement
pub trait Request {
    // type of the response from the server
    type Response: DeserializeOwned;

    // returns the message_id
    fn message_id(&self) -> Option<&str>;

    fn message_id_or_running(&self) -> String {
        match self.message_id() {
            Some(id) => id.to_string(),
            None => format!("_{}", RUNNING_MESSAGE_ID.fetch_add(1, Ordering::Relaxed)),
        }
    }

    // converts the struct into a JSON value
    // use request's message id if one was given, else use running number
    // prepended with an underscore to reduce the chance of conflicting with user-given ids
    fn to_json(&self) -> Value;
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetVersion<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetVersion<'_> {
    type Response = responses::GetVersion;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetVersion",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetAuthRequired<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetAuthRequired<'_> {
    type Response = responses::GetAuthRequired;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetAuthRequired",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct Authenticate<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    auth: &'a str,
}

impl Request for Authenticate<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "Authenticate",
            "message-id": self.message_id_or_running(),
            "auth": self.auth,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetHeartbeat<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    enable: bool,
}

impl Request for SetHeartbeat<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetHeartbeat",
            "message-id": self.message_id_or_running(),
            "enable": self.enable,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetFilenameFormatting<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    filename_formatting: &'a str,
}

impl Request for SetFilenameFormatting<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetFilenameFormatting",
            "message-id": self.message_id_or_running(),
            "filename-formatting": self.filename_formatting,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetFilenameFormatting<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetFilenameFormatting<'_> {
    type Response = responses::GetFilenameFormatting;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetFilenameFormatting",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetStats<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetStats<'_> {
    type Response = responses::GetStats;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStats",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq)]
pub struct BroadcastCustomMessage<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    realm: &'a str,
    data: Value,
}

impl Request for BroadcastCustomMessage<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "BroadcastCustomMessage",
            "message-id": self.message_id_or_running(),
            "realm": self.realm,
            "data": self.data,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetVideoInfo<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetVideoInfo<'_> {
    type Response = responses::GetVideoInfo;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetVideoInfo",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ListOutputs<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ListOutputs<'_> {
    type Response = responses::ListOutputs;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ListOutputs",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetOutputInfo<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    output_name: &'a str,
}

impl Request for GetOutputInfo<'_> {
    type Response = responses::GetOutputInfo;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetOutputInfo",
            "message-id": self.message_id_or_running(),
            "outputName": self.output_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartOutput<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    output_name: &'a str,
}

impl Request for StartOutput<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartOutput",
            "message-id": self.message_id_or_running(),
            "outputName": self.output_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopOutput<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    output_name: &'a str,
    #[builder(default, setter(strip_option))]
    force: Option<bool>,
}

impl Request for StopOutput<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopOutput",
            "message-id": self.message_id_or_running(),
            "outputName": self.output_name,
            "force": self.force,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentProfile<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    profile_name: &'a str,
}

impl Request for SetCurrentProfile<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentProfile",
            "message-id": self.message_id_or_running(),
            "profile-name": self.profile_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetCurrentProfile<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetCurrentProfile<'_> {
    type Response = responses::GetCurrentProfile;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentProfile",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ListProfiles<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ListProfiles<'_> {
    type Response = responses::ListProfiles;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ListProfiles",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartStopRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartStopRecording<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStopRecording",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartRecording<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartRecording",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StopRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StopRecording<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopRecording",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct PauseRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for PauseRecording<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "PauseRecording",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ResumeRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ResumeRecording<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ResumeRecording",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetRecordingFolder<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    rec_folder: &'a str,
}

impl Request for SetRecordingFolder<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetRecordingFolder",
            "message-id": self.message_id_or_running(),
            "rec-folder": self.rec_folder,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetRecordingFolder<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetRecordingFolder<'_> {
    type Response = responses::GetRecordingFolder;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetRecordingFolder",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartStopReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartStopReplayBuffer<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStopReplayBuffer",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartReplayBuffer<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartReplayBuffer",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StopReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StopReplayBuffer<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopReplayBuffer",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct SaveReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for SaveReplayBuffer<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SaveReplayBuffer",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentSceneCollection<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    sc_name: &'a str,
}

impl Request for SetCurrentSceneCollection<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentSceneCollection",
            "message-id": self.message_id_or_running(),
            "sc-name": self.sc_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetCurrentSceneCollection<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetCurrentSceneCollection<'_> {
    type Response = responses::GetCurrentSceneCollection;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentSceneCollection",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ListSceneCollections<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ListSceneCollections<'_> {
    type Response = responses::ListSceneCollections;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ListSceneCollections",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneItemProperties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    scene_name: Option<&'a str>,
    item: &'a str,
}

impl Request for GetSceneItemProperties<'_> {
    type Response = responses::GetSceneItemProperties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSceneItemProperties",
            "message-id": self.message_id_or_running(),
            "scene-name": self.scene_name,
            "item": self.item,
        })
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

#[derive(Debug, TypedBuilder, PartialEq)]
pub struct SetSceneItemProperties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    scene_name: Option<&'a str>,
    item: &'a str,
    #[builder(default, setter(strip_option))]
    position_x: Option<f64>,
    #[builder(default, setter(strip_option))]
    position_y: Option<f64>,
    #[builder(default, setter(strip_option))]
    position_alignment: Option<i32>,
    #[builder(default, setter(strip_option))]
    rotation: Option<f64>,
    #[builder(default, setter(strip_option))]
    scale_x: Option<f64>,
    #[builder(default, setter(strip_option))]
    scale_y: Option<f64>,
    #[builder(default, setter(strip_option))]
    crop_top: Option<i32>,
    #[builder(default, setter(strip_option))]
    crop_bottom: Option<i32>,
    #[builder(default, setter(strip_option))]
    crop_left: Option<i32>,
    #[builder(default, setter(strip_option))]
    crop_right: Option<i32>,
    #[builder(default, setter(strip_option))]
    visible: Option<bool>,
    #[builder(default, setter(strip_option))]
    locked: Option<bool>,
    #[builder(default, setter(strip_option))]
    bounds_type: Option<BoundsType>,
    #[builder(default, setter(strip_option))]
    bounds_alignment: Option<i32>,
    #[builder(default, setter(strip_option))]
    bounds_x: Option<f64>,
    #[builder(default, setter(strip_option))]
    bounds_y: Option<f64>,
}

impl Request for SetSceneItemProperties<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "message-id": self.message_id_or_running(),
            "request-type": "SetSceneItemProperties",
            "scene-name": self.scene_name,
            "item": self.item,
            "position": {
                "x": self.position_x,
                "y": self.position_y,
                "alignment": self.position_alignment,
            },
            "rotation": self.rotation,
            "scale": {
                "x": self.scale_x,
                "y": self.scale_y,
            },
            "crop": {
                "top": self.crop_top,
                "bottom": self.crop_bottom,
                "left": self.crop_left,
                "right": self.crop_right,
            },
            "visible": self.visible,
            "locked": self.locked,
            "bounds": {
                "type": self.bounds_type,
                "alignment": self.bounds_alignment,
                "x": self.bounds_x,
                "y": self.bounds_y,
            },
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ResetSceneItem<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    scene_name: Option<&'a str>,
    item: &'a str,
}

impl Request for ResetSceneItem<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ResetSceneItem",
            "message-id": self.message_id_or_running(),
            "scene-name": self.scene_name,
            "item": self.item,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DeleteSceneItem<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    scene: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    item_id: Option<ItemId<'a>>,
}

impl Request for DeleteSceneItem<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        json!({
            "request-type": "DeleteSceneItem",
            "message-id": self.message_id_or_running(),
            "scene": self.scene,
            "item": {
                "id": item_id,
                "name": item_name,
            },
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DuplicateSceneItem<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    from_scene: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    to_scene: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    item_id: Option<ItemId<'a>>,
}

impl Request for DuplicateSceneItem<'_> {
    type Response = responses::DuplicateSceneItem;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        json!({
            "request-type": "DuplicateSceneItem",
            "message-id": self.message_id_or_running(),
            "fromScene": self.from_scene,
            "toScene": self.to_scene,
            "item": {
                "name": item_name,
                "id": item_id,
            },
        })
    }
}

// cont here

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentScene<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    scene_name: &'a str,
}

impl Request for SetCurrentScene<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentScene",
            "message-id": self.message_id_or_running(),
            "scene-name": self.scene_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetCurrentScene<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetCurrentScene<'_> {
    type Response = responses::GetCurrentScene;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentScene",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetSceneList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetSceneList<'_> {
    type Response = responses::GetSceneList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSceneList",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSceneItems<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    scene: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    items: Option<Vec<ItemId<'a>>>,
}

impl Request for ReorderSceneItems<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        let items = self
            .items
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|item| match item {
                ItemId::Name(name) => json!({
                    "name": name,
                }),
                ItemId::Id(id) => json!({
                    "id": id,
                }),
            })
            .collect::<Vec<_>>();
        json!({
            "request-type": "ReorderSceneItems",
            "message-id": self.message_id_or_running(),
            "scene": self.scene,
            "items": items,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetSourcesList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetSourcesList<'_> {
    type Response = responses::GetSourcesList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourcesList",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetSourceTypesList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetSourceTypesList<'_> {
    type Response = responses::GetSourceTypesList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceTypesList",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVolume<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for GetVolume<'_> {
    type Response = responses::GetVolume;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetVolume",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetVolume<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
    volume: f64,
}

impl Request for SetVolume<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetVolume",
            "message-id": self.message_id_or_running(),
            "source": self.source,
            "volume": self.volume,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetMute<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for GetMute<'_> {
    type Response = responses::GetMute;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetMute",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetMute<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
    mute: bool,
}

impl Request for SetMute<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetMute",
            "message-id": self.message_id_or_running(),
            "source": self.source,
            "mute": self.mute,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleMute<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for ToggleMute<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ToggleMute",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSyncOffset<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
    offset: i32,
}

impl Request for SetSyncOffset<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSyncOffset",
            "message-id": self.message_id_or_running(),
            "source": self.source,
            "offset": self.offset
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSyncOffset<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for GetSyncOffset<'_> {
    type Response = responses::GetSyncOffset;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSyncOffset",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    #[builder(default, setter(strip_option))]
    source_type: Option<&'a str>,
}

impl Request for GetSourceSettings<'_> {
    type Response = responses::GetSourceSettings;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceSettings",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "sourceType": self.source_type,
        })
    }
}

// TODO: source settings
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    #[builder(default, setter(strip_option))]
    source_type: Option<&'a str>,
    source_settings: Value,
}

impl Request for SetSourceSettings<'_> {
    type Response = responses::SetSourceSettings;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSourceSettings",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "sourceType": self.source_type,
            "sourceSettings": self.source_settings,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextGDIPlusProperties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for GetTextGDIPlusProperties<'_> {
    type Response = responses::GetTextGDIPlusProperties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTextGDIPlusProperties",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetTextGDIPlusProperties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
    #[builder(default, setter(strip_option))]
    align: Option<Alignment>,
    #[builder(default, setter(strip_option))]
    bk_color: Option<i32>,
    #[builder(default, setter(strip_option))]
    bk_opacity: Option<i32>,
    #[builder(default, setter(strip_option))]
    chatlog: Option<bool>,
    #[builder(default, setter(strip_option))]
    chatlog_lines: Option<i32>,
    #[builder(default, setter(strip_option))]
    color: Option<i32>,
    #[builder(default, setter(strip_option))]
    extents: Option<bool>,
    #[builder(default, setter(strip_option))]
    extents_cx: Option<i32>,
    #[builder(default, setter(strip_option))]
    extents_cy: Option<i32>,
    #[builder(default, setter(strip_option))]
    file: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    read_from_file: Option<bool>,
    #[builder(default, setter(strip_option))]
    font_face: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    font_flags: Option<i32>,
    #[builder(default, setter(strip_option))]
    font_size: Option<i32>,
    #[builder(default, setter(strip_option))]
    font_style: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    gradient: Option<bool>,
    #[builder(default, setter(strip_option))]
    gradient_color: Option<i32>,
    #[builder(default, setter(strip_option))]
    gradient_dir: Option<f64>,
    #[builder(default, setter(strip_option))]
    gradient_opacity: Option<i32>,
    #[builder(default, setter(strip_option))]
    outline: Option<bool>,
    #[builder(default, setter(strip_option))]
    outline_color: Option<i32>,
    #[builder(default, setter(strip_option))]
    outline_size: Option<i32>,
    #[builder(default, setter(strip_option))]
    outline_opacity: Option<i32>,
    #[builder(default, setter(strip_option))]
    text: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    valign: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    vertical: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    render: Option<bool>,
}

impl Request for SetTextGDIPlusProperties<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetTextGDIPlusProperties",
            "message-id": self.message_id_or_running(),
            "source": self.source,
            "align": self.align,
            "bk-color": self.bk_color,
            "bk-opacity": self.bk_opacity,
            "chatlog": self.chatlog,
            "chatlog_lines": self.chatlog_lines,
            "color": self.color,
            "extents": self.extents,
            "extents_cx": self.extents_cx,
            "extents_cy": self.extents_cy,
            "file": self.file,
            "read_from_file": self.read_from_file,
            "font": {
                "face": self.font_face,
                "flags": self.font_flags,
                "size": self.font_size,
                "style": self.font_style,
            },
            "gradient": self.gradient,
            "gradient_color": self.gradient_color,
            "gradient_dir": self.gradient_dir,
            "gradient_opacity": self.gradient_opacity,
            "outline": self.outline,
            "outline_color": self.outline_color,
            "outline_size": self.outline_size,
            "outline_opacity": self.outline_opacity,
            "text": self.text,
            "valign": self.valign,
            "vertical": self.vertical,
            "render": self.render,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextFreetype2Properties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for GetTextFreetype2Properties<'_> {
    type Response = responses::GetTextFreetype2Properties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTextFreetype2Properties",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTextFreetype2Properties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
    #[builder(default, setter(strip_option))]
    color_1: Option<i32>,
    #[builder(default, setter(strip_option))]
    color_2: Option<i32>,
    #[builder(default, setter(strip_option))]
    custom_width: Option<i32>,
    #[builder(default, setter(strip_option))]
    drop_shadow: Option<bool>,
    #[builder(default, setter(strip_option))]
    font_face: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    font_flags: Option<i32>,
    #[builder(default, setter(strip_option))]
    font_size: Option<i32>,
    #[builder(default, setter(strip_option))]
    font_style: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    from_file: Option<bool>,
    #[builder(default, setter(strip_option))]
    log_mode: Option<bool>,
    #[builder(default, setter(strip_option))]
    outline: Option<bool>,
    #[builder(default, setter(strip_option))]
    text: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    text_file: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    word_wrap: Option<bool>,
}

impl Request for SetTextFreetype2Properties<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetTextFreetype2Properties",
            "message-id": self.message_id_or_running(),
            "source": self.source,
            "color1": self.color_1,
            "color2": self.color_2,
            "custom_width": self.custom_width,
            "drop_shadow": self.drop_shadow,
            "font": {
                "face": self.font_face,
                "flags": self.font_flags,
                "size": self.font_size,
                "style": self.font_style,
            },
            "from_file": self.from_file,
            "log_mode": self.log_mode,
            "outline": self.outline,
            "text": self.text,
            "text_file": self.text_file,
            "word_wrap": self.word_wrap,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetBrowserSourceProperties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
}

impl Request for GetBrowserSourceProperties<'_> {
    type Response = responses::GetBrowserSourceProperties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetBrowserSourceProperties",
            "message-id": self.message_id_or_running(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetBrowserSourceProperties<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source: &'a str,
    #[builder(default, setter(strip_option))]
    is_local_file: Option<bool>,
    #[builder(default, setter(strip_option))]
    local_file: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    url: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    css: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    width: Option<i32>,
    #[builder(default, setter(strip_option))]
    height: Option<i32>,
    #[builder(default, setter(strip_option))]
    fps: Option<i32>,
    #[builder(default, setter(strip_option))]
    shutdown: Option<bool>,
    #[builder(default, setter(strip_option))]
    render: Option<bool>,
}

impl Request for SetBrowserSourceProperties<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetBrowserSourceProperties",
            "message-id": self.message_id_or_running(),
            "source": self.source,
            "is_local_file": self.is_local_file,
            "local_file": self.local_file,
            "url": self.url,
            "css": self.css,
            "width": self.width,
            "height": self.height,
            "fps": self.fps,
            "shutdown": self.shutdown,
            "render": self.render,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetSpecialSources<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetSpecialSources<'_> {
    type Response = responses::GetSpecialSources;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSpecialSources",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilters<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
}

impl Request for GetSourceFilters<'_> {
    type Response = responses::GetSourceFilters;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceFilters",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilterInfo<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
}

impl Request for GetSourceFilterInfo<'_> {
    type Response = responses::GetSourceFilterInfo;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceFilterInfo",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
        })
    }
}

// TODO: filter settings
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct AddFilterToSource<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
    filter_type: &'a str,
    filter_settings: Value,
}

impl Request for AddFilterToSource<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "AddFilterToSource",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterType": self.filter_type,
            "filterSettings": self.filter_settings,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct RemoveFilterFromSource<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
}

impl Request for RemoveFilterFromSource<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "RemoveFilterFromSource",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSourceFilter<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
    new_index: i32,
}

impl Request for ReorderSourceFilter<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ReorderSourceFilter",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "newIndex": self.new_index,
        })
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum MovementType {
    Up,
    Down,
    Top,
    Bottom,
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct MoveSourceFilter<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
    movement_type: MovementType,
}

impl Request for MoveSourceFilter<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "MoveSourceFilter",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "movementType": self.movement_type,
        })
    }
}

// TODO: filter settings
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceFilterSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
    filter_settings: Value,
}

impl Request for SetSourceFilterSettings<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSourceFilterSettings",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterSettings": self.filter_settings,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSourceFilterVisibility<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    filter_name: &'a str,
    filter_enabled: bool,
}

impl Request for SetSourceFilterVisibility<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSourceFilterVisibility",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterEnabled": self.filter_enabled,
        })
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmbedPictureFormat {
    Bmp,
    Gif,
    Jpg,
    Jpeg,
    Png,
    Pbm,
    Pgm,
    Ppm,
    Xbm,
    Xpm,
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TakeSourceScreenshot<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    source_name: &'a str,
    #[builder(default, setter(strip_option))]
    embed_picture_format: Option<EmbedPictureFormat>,
    #[builder(default, setter(strip_option))]
    save_to_file_path: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    width: Option<i32>,
    #[builder(default, setter(strip_option))]
    height: Option<i32>,
}

impl Request for TakeSourceScreenshot<'_> {
    type Response = responses::TakeSourceScreenshot;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "TakeSourceScreenshot",
            "message-id": self.message_id_or_running(),
            "sourceName": self.source_name,
            "embedPictureFormat": self.embed_picture_format,
            "saveToFilePath": self.save_to_file_path,
            "width": self.width,
            "height": self.height,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetStreamingStatus<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetStreamingStatus<'_> {
    type Response = responses::GetStreamingStatus;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStreamingStatus",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartStopStreaming<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartStopStreaming<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStopStreaming",
            "message-id": self.message_id_or_running(),
        })
    }
}

// TODO:
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct StartStreaming<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_type: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_metadata: Option<Value>,
    #[builder(default, setter(strip_option))]
    stream_server: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_key: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_use_auth: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_username: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_password: Option<&'a str>,
}

impl Request for StartStreaming<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStreaming",
            "message-id": self.message_id_or_running(),
            "stream": {
                "type": self.stream_type,
                "metadata": self.stream_metadata,
                "settings": {
                    "server": self.stream_server,
                    "key": self.stream_key,
                    "use-auth": self.stream_use_auth,
                    "username": self.stream_username,
                    "password": self.stream_password,
                },
            },
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StopStreaming<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StopStreaming<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopStreaming",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetStreamSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    stream_type: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    server: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    key: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    use_auth: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    username: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    password: Option<&'a str>,
    save: bool,
}

impl Request for SetStreamSettings<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetStreamSettings",
            "message-id": self.message_id_or_running(),
            "type": self.stream_type,
            "settings": {
                "server": self.server,
                "key": self.key,
                "use-auth": self.use_auth,
                "username": self.username,
                "password": self.password,
            },
            "save": self.save,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetStreamSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetStreamSettings<'_> {
    type Response = responses::GetStreamSettings;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStreamSettings",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct SaveStreamSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for SaveStreamSettings<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SaveStreamSettings",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SendCaptions<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    text: &'a str,
}

impl Request for SendCaptions<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SendCaptions",
            "message-id": self.message_id_or_running(),
            "text": self.text,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetStudioModeStatus<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetStudioModeStatus<'_> {
    type Response = responses::GetStudioModeStatus;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStudioModeStatus",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetPreviewScene<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetPreviewScene<'_> {
    type Response = responses::GetPreviewScene;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetPreviewScene",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetPreviewScene<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    scene_name: &'a str,
}

impl Request for SetPreviewScene<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetPreviewScene",
            "message-id": self.message_id_or_running(),
            "scene-name": self.scene_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TransitionToProgram<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    with_transition_name: Option<&'a str>,
    with_transition_duration: Option<&'a str>,
}

impl Request for TransitionToProgram<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "TransitionToProgram",
            "message-id": self.message_id_or_running(),
            "with-transition": {
                "name": self.with_transition_name,
                "duration": self.with_transition_duration,
            }
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct EnableStudioMode<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for EnableStudioMode<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "EnableStudioMode",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct DisableStudioMode<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for DisableStudioMode<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "DisableStudioMode",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ToggleStudioMode<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ToggleStudioMode<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ToggleStudioMode",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetTransitionList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetTransitionList<'_> {
    type Response = responses::GetTransitionList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTransitionList",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetCurrentTransition<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetCurrentTransition<'_> {
    type Response = responses::GetCurrentTransition;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentTransition",
            "message-id": self.message_id_or_running(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentTransition<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    transition_name: &'a str,
}

impl Request for SetCurrentTransition<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentTransition",
            "message-id": self.message_id_or_running(),
            "transition-name": self.transition_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTransitionDuration<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
    duration: i32,
}

impl Request for SetTransitionDuration<'_> {
    type Response = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetTransitionDuration",
            "message-id": self.message_id_or_running(),
            "duration": self.duration,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetTransitionDuration<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetTransitionDuration<'_> {
    type Response = responses::GetTransitionDuration;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTransitionDuration",
            "message-id": self.message_id_or_running(),
        })
    }
}

// #### other typedefs ####
#[derive(Debug, PartialEq, Eq)]
pub enum ItemId<'a> {
    Name(&'a str),
    Id(i32),
}

impl<'a> ItemId<'a> {
    fn to_name(&self) -> Option<&'a str> {
        match self {
            Self::Name(s) => Some(s),
            _ => None,
        }
    }

    fn to_id(&self) -> Option<&i32> {
        match self {
            Self::Id(i) => Some(i),
            _ => None,
        }
    }
}
