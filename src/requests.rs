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
    fn get_message_id(&self) -> &str;

    // converts the struct into a JSON value
    fn to_json(&self) -> Value;
}

// creates a default value for message-id, using a running id
fn make_message_id() -> String {
    format!("_{}", RUNNING_MESSAGE_ID.fetch_add(1, Ordering::Relaxed))
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVersion {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetVersion {
    type Response = responses::GetVersion;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetVersion",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetAuthRequired {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetAuthRequired {
    type Response = responses::GetAuthRequired;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetAuthRequired",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct Authenticate {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    auth: String,
}

impl Request for Authenticate {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "Authenticate",
            "message-id": self.get_message_id(),
            "auth": self.auth,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetHeartbeat {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    enable: bool,
}

impl Request for SetHeartbeat {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetHeartbeat",
            "message-id": self.get_message_id(),
            "enable": self.enable,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetFilenameFormatting {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    filename_formatting: String,
}

impl Request for SetFilenameFormatting {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetFilenameFormatting",
            "message-id": self.get_message_id(),
            "filename-formatting": self.filename_formatting,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetFilenameFormatting {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetFilenameFormatting {
    type Response = responses::GetFilenameFormatting;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetFilenameFormatting",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStats {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStats {
    type Response = responses::GetStats;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStats",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq)]
pub struct BroadcastCustomMessage {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    realm: String,
    data: Value,
}

impl Request for BroadcastCustomMessage {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "BroadcastCustomMessage",
            "message-id": self.get_message_id(),
            "realm": self.realm,
            "data": self.data,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVideoInfo {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetVideoInfo {
    type Response = responses::GetVideoInfo;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetVideoInfo",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListOutputs {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ListOutputs {
    type Response = responses::ListOutputs;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ListOutputs",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetOutputInfo {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    output_name: String,
}

impl Request for GetOutputInfo {
    type Response = responses::GetOutputInfo;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetOutputInfo",
            "message-id": self.get_message_id(),
            "outputName": self.output_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartOutput {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    output_name: String,
}

impl Request for StartOutput {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartOutput",
            "message-id": self.get_message_id(),
            "outputName": self.output_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopOutput {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    output_name: String,
    #[builder(default, setter(strip_option))]
    force: Option<bool>,
}

impl Request for StopOutput {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopOutput",
            "message-id": self.get_message_id(),
            "outputName": self.output_name,
            "force": self.force,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentProfile {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    profile_name: String,
}

impl Request for SetCurrentProfile {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentProfile",
            "message-id": self.get_message_id(),
            "profile-name": self.profile_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentProfile {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentProfile {
    type Response = responses::GetCurrentProfile;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentProfile",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListProfiles {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ListProfiles {
    type Response = responses::ListProfiles;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ListProfiles",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartStopRecording {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStopRecording",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartRecording {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartRecording",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StopRecording {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopRecording",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct PauseRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for PauseRecording {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "PauseRecording",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ResumeRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ResumeRecording {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ResumeRecording",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetRecordingFolder {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    rec_folder: String,
}

impl Request for SetRecordingFolder {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetRecordingFolder",
            "message-id": self.get_message_id(),
            "rec-folder": self.rec_folder,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetRecordingFolder {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetRecordingFolder {
    type Response = responses::GetRecordingFolder;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetRecordingFolder",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartStopReplayBuffer {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStopReplayBuffer",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartReplayBuffer {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartReplayBuffer",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StopReplayBuffer {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopReplayBuffer",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SaveReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for SaveReplayBuffer {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SaveReplayBuffer",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentSceneCollection {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    sc_name: String,
}

impl Request for SetCurrentSceneCollection {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentSceneCollection",
            "message-id": self.get_message_id(),
            "sc-name": self.sc_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentSceneCollection {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentSceneCollection {
    type Response = responses::GetCurrentSceneCollection;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentSceneCollection",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListSceneCollections {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ListSceneCollections {
    type Response = responses::ListSceneCollections;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ListSceneCollections",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneItemProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    scene_name: Option<String>,
    #[builder(setter(into))]
    item: String,
}

impl Request for GetSceneItemProperties {
    type Response = responses::GetSceneItemProperties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSceneItemProperties",
            "message-id": self.get_message_id(),
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
pub struct SetSceneItemProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    scene_name: Option<String>,
    #[builder(setter(into))]
    item: String,
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

impl Request for SetSceneItemProperties {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "message-id": self.get_message_id(),
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
pub struct ResetSceneItem {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    scene_name: Option<String>,
    #[builder(setter(into))]
    item: String,
}

impl Request for ResetSceneItem {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ResetSceneItem",
            "message-id": self.get_message_id(),
            "scene-name": self.scene_name,
            "item": self.item,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DeleteSceneItem {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    scene: Option<String>,
    #[builder(default, setter(strip_option))]
    item_id: Option<ItemId>,
}

impl Request for DeleteSceneItem {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        json!({
            "request-type": "DeleteSceneItem",
            "message-id": self.get_message_id(),
            "scene": self.scene,
            "item": {
                "id": item_id,
                "name": item_name,
            },
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DuplicateSceneItem {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    from_scene: Option<String>,
    #[builder(default, setter(strip_option, into))]
    to_scene: Option<String>,
    #[builder(default, setter(strip_option))]
    item_id: Option<ItemId>,
}

impl Request for DuplicateSceneItem {
    type Response = responses::DuplicateSceneItem;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        json!({
            "request-type": "DuplicateSceneItem",
            "message-id": self.get_message_id(),
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
pub struct SetCurrentScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    scene_name: String,
}

impl Request for SetCurrentScene {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentScene",
            "message-id": self.get_message_id(),
            "scene-name": self.scene_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentScene {
    type Response = responses::GetCurrentScene;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentScene",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSceneList {
    type Response = responses::GetSceneList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSceneList",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSceneItems {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    scene: Option<String>,
    #[builder(default, setter(strip_option))]
    items: Option<Vec<ItemId>>,
}

impl Request for ReorderSceneItems {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
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
            "message-id": self.get_message_id(),
            "scene": self.scene,
            "items": items,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourcesList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSourcesList {
    type Response = responses::GetSourcesList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourcesList",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceTypesList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSourceTypesList {
    type Response = responses::GetSourceTypesList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceTypesList",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVolume {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for GetVolume {
    type Response = responses::GetVolume;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetVolume",
            "message-id": self.get_message_id(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetVolume {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
    volume: f64,
}

impl Request for SetVolume {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetVolume",
            "message-id": self.get_message_id(),
            "source": self.source,
            "volume": self.volume,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetMute {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for GetMute {
    type Response = responses::GetMute;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetMute",
            "message-id": self.get_message_id(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetMute {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
    mute: bool,
}

impl Request for SetMute {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetMute",
            "message-id": self.get_message_id(),
            "source": self.source,
            "mute": self.mute,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleMute {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for ToggleMute {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ToggleMute",
            "message-id": self.get_message_id(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSyncOffset {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
    offset: i32,
}

impl Request for SetSyncOffset {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSyncOffset",
            "message-id": self.get_message_id(),
            "source": self.source,
            "offset": self.offset
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSyncOffset {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for GetSyncOffset {
    type Response = responses::GetSyncOffset;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSyncOffset",
            "message-id": self.get_message_id(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(default, setter(strip_option, into))]
    source_type: Option<String>,
}

impl Request for GetSourceSettings {
    type Response = responses::GetSourceSettings;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceSettings",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "sourceType": self.source_type,
        })
    }
}

// TODO: source settings
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(default, setter(strip_option, into))]
    source_type: Option<String>,
    source_settings: Value,
}

impl Request for SetSourceSettings {
    type Response = responses::SetSourceSettings;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSourceSettings",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "sourceType": self.source_type,
            "sourceSettings": self.source_settings,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextGDIPlusProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for GetTextGDIPlusProperties {
    type Response = responses::GetTextGDIPlusProperties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTextGDIPlusProperties",
            "message-id": self.get_message_id(),
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
pub struct SetTextGDIPlusProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
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
    #[builder(default, setter(strip_option, into))]
    file: Option<String>,
    #[builder(default, setter(strip_option))]
    read_from_file: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    font_face: Option<String>,
    #[builder(default, setter(strip_option))]
    font_flags: Option<i32>,
    #[builder(default, setter(strip_option))]
    font_size: Option<i32>,
    #[builder(default, setter(strip_option, into))]
    font_style: Option<String>,
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
    #[builder(default, setter(strip_option, into))]
    text: Option<String>,
    #[builder(default, setter(strip_option, into))]
    valign: Option<String>,
    #[builder(default, setter(strip_option, into))]
    vertical: Option<String>,
    #[builder(default, setter(strip_option))]
    render: Option<bool>,
}

impl Request for SetTextGDIPlusProperties {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetTextGDIPlusProperties",
            "message-id": self.get_message_id(),
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
pub struct GetTextFreetype2Properties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for GetTextFreetype2Properties {
    type Response = responses::GetTextFreetype2Properties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTextFreetype2Properties",
            "message-id": self.get_message_id(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTextFreetype2Properties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
    #[builder(default, setter(strip_option))]
    color_1: Option<i32>,
    #[builder(default, setter(strip_option))]
    color_2: Option<i32>,
    #[builder(default, setter(strip_option))]
    custom_width: Option<i32>,
    #[builder(default, setter(strip_option))]
    drop_shadow: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    font_face: Option<String>,
    #[builder(default, setter(strip_option))]
    font_flags: Option<i32>,
    #[builder(default, setter(strip_option))]
    font_size: Option<i32>,
    #[builder(default, setter(strip_option, into))]
    font_style: Option<String>,
    #[builder(default, setter(strip_option))]
    from_file: Option<bool>,
    #[builder(default, setter(strip_option))]
    log_mode: Option<bool>,
    #[builder(default, setter(strip_option))]
    outline: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    text: Option<String>,
    #[builder(default, setter(strip_option, into))]
    text_file: Option<String>,
    #[builder(default, setter(strip_option))]
    word_wrap: Option<bool>,
}

impl Request for SetTextFreetype2Properties {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetTextFreetype2Properties",
            "message-id": self.get_message_id(),
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
pub struct GetBrowserSourceProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
}

impl Request for GetBrowserSourceProperties {
    type Response = responses::GetBrowserSourceProperties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetBrowserSourceProperties",
            "message-id": self.get_message_id(),
            "source": self.source,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetBrowserSourceProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source: String,
    #[builder(default, setter(strip_option))]
    is_local_file: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    local_file: Option<String>,
    #[builder(default, setter(strip_option, into))]
    url: Option<String>,
    #[builder(default, setter(strip_option, into))]
    css: Option<String>,
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

impl Request for SetBrowserSourceProperties {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetBrowserSourceProperties",
            "message-id": self.get_message_id(),
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

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSpecialSources {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSpecialSources {
    type Response = responses::GetSpecialSources;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSpecialSources",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilters {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
}

impl Request for GetSourceFilters {
    type Response = responses::GetSourceFilters;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceFilters",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilterInfo {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
}

impl Request for GetSourceFilterInfo {
    type Response = responses::GetSourceFilterInfo;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetSourceFilterInfo",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
        })
    }
}

// TODO: filter settings
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct AddFilterToSource {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
    #[builder(setter(into))]
    filter_type: String,
    filter_settings: Value,
}

impl Request for AddFilterToSource {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "AddFilterToSource",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterType": self.filter_type,
            "filterSettings": self.filter_settings,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct RemoveFilterFromSource {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
}

impl Request for RemoveFilterFromSource {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "RemoveFilterFromSource",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSourceFilter {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
    new_index: i32,
}

impl Request for ReorderSourceFilter {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ReorderSourceFilter",
            "message-id": self.get_message_id(),
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
pub struct MoveSourceFilter {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
    movement_type: MovementType,
}

impl Request for MoveSourceFilter {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "MoveSourceFilter",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "movementType": self.movement_type,
        })
    }
}

// TODO: filter settings
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceFilterSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
    filter_settings: Value,
}

impl Request for SetSourceFilterSettings {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSourceFilterSettings",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterSettings": self.filter_settings,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSourceFilterVisibility {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(setter(into))]
    filter_name: String,
    filter_enabled: bool,
}

impl Request for SetSourceFilterVisibility {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetSourceFilterVisibility",
            "message-id": self.get_message_id(),
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
pub struct TakeSourceScreenshot {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    source_name: String,
    #[builder(default, setter(strip_option))]
    embed_picture_format: Option<EmbedPictureFormat>,
    #[builder(default, setter(strip_option, into))]
    save_to_file_path: Option<String>,
    #[builder(default, setter(strip_option))]
    width: Option<i32>,
    #[builder(default, setter(strip_option))]
    height: Option<i32>,
}

impl Request for TakeSourceScreenshot {
    type Response = responses::TakeSourceScreenshot;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "TakeSourceScreenshot",
            "message-id": self.get_message_id(),
            "sourceName": self.source_name,
            "embedPictureFormat": self.embed_picture_format,
            "saveToFilePath": self.save_to_file_path,
            "width": self.width,
            "height": self.height,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStreamingStatus {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStreamingStatus {
    type Response = responses::GetStreamingStatus;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStreamingStatus",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopStreaming {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartStopStreaming {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStopStreaming",
            "message-id": self.get_message_id(),
        })
    }
}

// TODO:
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct StartStreaming {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    stream_type: Option<String>,
    #[builder(default, setter(strip_option))]
    stream_metadata: Option<Value>,
    #[builder(default, setter(strip_option, into))]
    stream_server: Option<String>,
    #[builder(default, setter(strip_option, into))]
    stream_key: Option<String>,
    #[builder(default, setter(strip_option, into))]
    stream_use_auth: Option<String>,
    #[builder(default, setter(strip_option, into))]
    stream_username: Option<String>,
    #[builder(default, setter(strip_option, into))]
    stream_password: Option<String>,
}

impl Request for StartStreaming {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StartStreaming",
            "message-id": self.get_message_id(),
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

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopStreaming {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StopStreaming {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "StopStreaming",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetStreamSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    stream_type: Option<String>,
    #[builder(default, setter(strip_option, into))]
    server: Option<String>,
    #[builder(default, setter(strip_option, into))]
    key: Option<String>,
    #[builder(default, setter(strip_option, into))]
    use_auth: Option<String>,
    #[builder(default, setter(strip_option, into))]
    username: Option<String>,
    #[builder(default, setter(strip_option, into))]
    password: Option<String>,
    save: bool,
}

impl Request for SetStreamSettings {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetStreamSettings",
            "message-id": self.get_message_id(),
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

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStreamSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStreamSettings {
    type Response = responses::GetStreamSettings;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStreamSettings",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SaveStreamSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for SaveStreamSettings {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SaveStreamSettings",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SendCaptions {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    text: String,
}

impl Request for SendCaptions {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SendCaptions",
            "message-id": self.get_message_id(),
            "text": self.text,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStudioModeStatus {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStudioModeStatus {
    type Response = responses::GetStudioModeStatus;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetStudioModeStatus",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetPreviewScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetPreviewScene {
    type Response = responses::GetPreviewScene;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetPreviewScene",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetPreviewScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    scene_name: String,
}

impl Request for SetPreviewScene {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetPreviewScene",
            "message-id": self.get_message_id(),
            "scene-name": self.scene_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TransitionToProgram {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(default, setter(strip_option, into))]
    with_transition_name: Option<String>,
    #[builder(default, setter(strip_option, into))]
    with_transition_duration: Option<String>,
}

impl Request for TransitionToProgram {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "TransitionToProgram",
            "message-id": self.get_message_id(),
            "with-transition": {
                "name": self.with_transition_name,
                "duration": self.with_transition_duration,
            }
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct EnableStudioMode {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for EnableStudioMode {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "EnableStudioMode",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DisableStudioMode {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for DisableStudioMode {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "DisableStudioMode",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleStudioMode {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ToggleStudioMode {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "ToggleStudioMode",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTransitionList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetTransitionList {
    type Response = responses::GetTransitionList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTransitionList",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentTransition {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentTransition {
    type Response = responses::GetCurrentTransition;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetCurrentTransition",
            "message-id": self.get_message_id(),
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentTransition {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    #[builder(setter(into))]
    transition_name: String,
}

impl Request for SetCurrentTransition {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetCurrentTransition",
            "message-id": self.get_message_id(),
            "transition-name": self.transition_name,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTransitionDuration {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    duration: i32,
}

impl Request for SetTransitionDuration {
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "SetTransitionDuration",
            "message-id": self.get_message_id(),
            "duration": self.duration,
        })
    }
}

#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTransitionDuration {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetTransitionDuration {
    type Response = responses::GetTransitionDuration;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": "GetTransitionDuration",
            "message-id": self.get_message_id(),
        })
    }
}

// #### other typedefs ####
#[derive(Debug, PartialEq, Eq)]
pub enum ItemId {
    Name(String),
    Id(i32),
}

impl ItemId {
    fn to_name(&self) -> Option<String> {
        match self {
            Self::Name(s) => Some(s.clone()),
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
