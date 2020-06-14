//! Request types. Sent to the server

use super::responses;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::num::Wrapping;
use typed_builder::TypedBuilder;

// trait that all request types must implement
pub trait Request {
    // type of the response from the server
    type Output: DeserializeOwned;

    // returns the message_id
    fn message_id(&self) -> Option<&str>;

    // converts the struct into a JSON value
    // use request's message id if one was given, else use running number
    // prepended with an underscore to reduce the chance of conflicting with user-given ids
    fn to_json(&self, message_id: String) -> Value;

    // gets the message id or makes a new one using the running id
    fn message_id_or_running(&self, running_id: &mut Wrapping<u32>) -> String {
        self.message_id().map(|s| s.to_string()).unwrap_or_else(|| {
            let id = running_id.0;
            *running_id += Wrapping(1);
            format!("_{}", id)
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetVersion<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetVersion<'_> {
    type Output = responses::GetVersion;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetVersion",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetAuthRequired<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetAuthRequired<'_> {
    type Output = responses::GetAuthRequired;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetAuthRequired",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "Authenticate",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetHeartbeat",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetFilenameFormatting",
            "message-id": message_id,
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
    type Output = responses::GetFilenameFormatting;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetFilenameFormatting",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetStats<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetStats<'_> {
    type Output = responses::GetStats;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetStats",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "BroadcastCustomMessage",
            "message-id": message_id,
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
    type Output = responses::GetVideoInfo;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetVideoInfo",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ListOutputs<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ListOutputs<'_> {
    type Output = responses::ListOutputs;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ListOutputs",
            "message-id": message_id,
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
    type Output = responses::GetOutputInfo;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetOutputInfo",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartOutput",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StopOutput",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetCurrentProfile",
            "message-id": message_id,
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
    type Output = responses::GetCurrentProfile;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetCurrentProfile",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ListProfiles<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ListProfiles<'_> {
    type Output = responses::ListProfiles;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ListProfiles",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartStopRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartStopRecording<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartStopRecording",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartRecording<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartRecording",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StopRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StopRecording<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StopRecording",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct PauseRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for PauseRecording<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "PauseRecording",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ResumeRecording<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ResumeRecording<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ResumeRecording",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetRecordingFolder",
            "message-id": message_id,
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
    type Output = responses::GetRecordingFolder;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetRecordingFolder",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartStopReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartStopReplayBuffer<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartStopReplayBuffer",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartReplayBuffer<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartReplayBuffer",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StopReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StopReplayBuffer<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StopReplayBuffer",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct SaveReplayBuffer<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for SaveReplayBuffer<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SaveReplayBuffer",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetCurrentSceneCollection",
            "message-id": message_id,
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
    type Output = responses::GetCurrentSceneCollection;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetCurrentSceneCollection",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ListSceneCollections<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ListSceneCollections<'_> {
    type Output = responses::ListSceneCollections;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ListSceneCollections",
            "message-id": message_id,
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
    type Output = responses::GetSceneItemProperties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSceneItemProperties",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ResetSceneItem",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        json!({
            "request-type": "DeleteSceneItem",
            "message-id": message_id,
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
    type Output = responses::DuplicateSceneItem;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        json!({
            "request-type": "DuplicateSceneItem",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetCurrentScene",
            "message-id": message_id,
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
    type Output = responses::GetCurrentScene;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetCurrentScene",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetSceneList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetSceneList<'_> {
    type Output = responses::GetSceneList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSceneList",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
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
            "message-id": message_id,
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
    type Output = responses::GetSourcesList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSourcesList",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetSourceTypesList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetSourceTypesList<'_> {
    type Output = responses::GetSourceTypesList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSourceTypesList",
            "message-id": message_id,
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
    type Output = responses::GetVolume;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetVolume",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetVolume",
            "message-id": message_id,
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
    type Output = responses::GetMute;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetMute",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetMute",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ToggleMute",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetSyncOffset",
            "message-id": message_id,
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
    type Output = responses::GetSyncOffset;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSyncOffset",
            "message-id": message_id,
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
    type Output = responses::GetSourceSettings;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSourceSettings",
            "message-id": message_id,
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
    type Output = responses::SetSourceSettings;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetSourceSettings",
            "message-id": message_id,
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
    type Output = responses::GetTextGDIPlusProperties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetTextGDIPlusProperties",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetTextGDIPlusProperties",
            "message-id": message_id,
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
    type Output = responses::GetTextFreetype2Properties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetTextFreetype2Properties",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetTextFreetype2Properties",
            "message-id": message_id,
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
    type Output = responses::GetBrowserSourceProperties;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetBrowserSourceProperties",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetBrowserSourceProperties",
            "message-id": message_id,
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
    type Output = responses::GetSpecialSources;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSpecialSources",
            "message-id": message_id,
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
    type Output = responses::GetSourceFilters;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSourceFilters",
            "message-id": message_id,
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
    type Output = responses::GetSourceFilterInfo;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetSourceFilterInfo",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "AddFilterToSource",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "RemoveFilterFromSource",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ReorderSourceFilter",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "MoveSourceFilter",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetSourceFilterSettings",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetSourceFilterVisibility",
            "message-id": message_id,
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
    type Output = responses::TakeSourceScreenshot;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "TakeSourceScreenshot",
            "message-id": message_id,
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
    type Output = responses::GetStreamingStatus;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetStreamingStatus",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct StartStopStreaming<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for StartStopStreaming<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartStopStreaming",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StartStreaming",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "StopStreaming",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetStreamSettings",
            "message-id": message_id,
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
    type Output = responses::GetStreamSettings;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetStreamSettings",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct SaveStreamSettings<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for SaveStreamSettings<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SaveStreamSettings",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SendCaptions",
            "message-id": message_id,
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
    type Output = responses::GetStudioModeStatus;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetStudioModeStatus",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetPreviewScene<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetPreviewScene<'_> {
    type Output = responses::GetPreviewScene;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetPreviewScene",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetPreviewScene",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "TransitionToProgram",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "EnableStudioMode",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct DisableStudioMode<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for DisableStudioMode<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "DisableStudioMode",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct ToggleStudioMode<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for ToggleStudioMode<'_> {
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "ToggleStudioMode",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetTransitionList<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetTransitionList<'_> {
    type Output = responses::GetTransitionList;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetTransitionList",
            "message-id": message_id,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Default)]
pub struct GetCurrentTransition<'a> {
    #[builder(default, setter(strip_option))]
    message_id: Option<&'a str>,
}

impl Request for GetCurrentTransition<'_> {
    type Output = responses::GetCurrentTransition;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetCurrentTransition",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetCurrentTransition",
            "message-id": message_id,
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
    type Output = responses::Empty;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "SetTransitionDuration",
            "message-id": message_id,
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
    type Output = responses::GetTransitionDuration;

    fn message_id(&self) -> Option<&str> {
        self.message_id
    }

    fn to_json(&self, message_id: String) -> Value {
        json!({
            "request-type": "GetTransitionDuration",
            "message-id": message_id,
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
