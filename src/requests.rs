//! Request types. Sent to the server using the Obs struct.
//!
//! The request types will generate a running message-id by default, but they also support defining custom message-ids.
//! When using custom message-ids, avoid reusing them and if also using default message-ids, avoid using custom ones in the form `_{integer}` to avoid clashing which may cause responses to be parsed incorrectly.
//!
//! To find the response type of a given request, see the impl Request for the type in its docs.

use crate::{common_types::*, responses};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use typed_builder::TypedBuilder;

static RUNNING_MESSAGE_ID: AtomicU32 = AtomicU32::new(0);

// trait that all request types must implement
pub trait Request {
    // request-type
    const REQUEST_TYPE: &'static str;

    // type of the response from the server
    type Response: DeserializeOwned;

    // converts the struct into a JSON value
    // returns the generated message id and the JSON
    fn to_json(&self) -> (String, Value);
}

// creates a default value for message-id, using a running id
fn make_message_id() -> String {
    format!("_{}", RUNNING_MESSAGE_ID.fetch_add(1, Ordering::Relaxed))
}

/// Returns the latest version of the plugin and the API.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVersion {}

impl Request for GetVersion {
    const REQUEST_TYPE: &'static str = "GetVersion";
    type Response = responses::GetVersion;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Tells the client if authentication is required.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetAuthRequired {}

impl Request for GetAuthRequired {
    const REQUEST_TYPE: &'static str = "GetAuthRequired";
    type Response = responses::GetAuthRequired;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Attempt to authenticate the client to the server.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct Authenticate {
    /// Response to the auth challenge.
    #[builder(setter(into))]
    pub auth: String,
}

impl Request for Authenticate {
    const REQUEST_TYPE: &'static str = "Authenticate";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "auth": self.auth,
            }),
        )
    }
}

/// Enable/disable sending of the Heartbeat event
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetHeartbeat {
    /// Starts/Stops emitting heartbeat messages
    pub enable: bool,
}

impl Request for SetHeartbeat {
    const REQUEST_TYPE: &'static str = "SetHeartbeat";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "enable": self.enable,
            }),
        )
    }
}

/// Set the filename formatting string
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetFilenameFormatting {
    /// Filename formatting string to set.
    #[builder(setter(into))]
    pub filename_formatting: String,
}

impl Request for SetFilenameFormatting {
    const REQUEST_TYPE: &'static str = "SetFilenameFormatting";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "filename-formatting": self.filename_formatting,
            }),
        )
    }
}

/// Get the filename formatting string
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetFilenameFormatting {}

impl Request for GetFilenameFormatting {
    const REQUEST_TYPE: &'static str = "GetFilenameFormatting";
    type Response = responses::GetFilenameFormatting;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get OBS stats (almost the same info as provided in OBS' stats window)
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStats {}

impl Request for GetStats {
    const REQUEST_TYPE: &'static str = "GetStats";
    type Response = responses::GetStats;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Broadcast custom message to all connected WebSocket clients
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct BroadcastCustomMessage {
    /// Identifier to be choosen by the client
    #[builder(setter(into))]
    pub realm: String,
    /// User-defined data
    pub data: Value,
}

impl Request for BroadcastCustomMessage {
    const REQUEST_TYPE: &'static str = "BroadcastCustomMessage";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "realm": self.realm,
                "data": self.data,
            }),
        )
    }
}

/// Get basic OBS video information
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVideoInfo {}

impl Request for GetVideoInfo {
    const REQUEST_TYPE: &'static str = "GetVideoInfo";
    type Response = responses::GetVideoInfo;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// List existing outputs
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListOutputs {}

impl Request for ListOutputs {
    const REQUEST_TYPE: &'static str = "ListOutputs";
    type Response = responses::ListOutputs;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get information about a single output
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetOutputInfo {
    /// Output name
    #[builder(setter(into))]
    pub output_name: String,
}

impl Request for GetOutputInfo {
    const REQUEST_TYPE: &'static str = "GetOutputInfo";
    type Response = responses::GetOutputInfo;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "outputName": self.output_name,
            }),
        )
    }
}

/// Start an output
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartOutput {
    /// Output name
    #[builder(setter(into))]
    pub output_name: String,
}

impl Request for StartOutput {
    const REQUEST_TYPE: &'static str = "StartOutput";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "outputName": self.output_name,
            }),
        )
    }
}

/// Stop an output
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopOutput {
    /// Output name
    #[builder(setter(into))]
    pub output_name: String,
    /// Force stop (default: false)
    #[builder(default, setter(strip_option))]
    pub force: Option<bool>,
}

impl Request for StopOutput {
    const REQUEST_TYPE: &'static str = "StopOutput";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "outputName": self.output_name,
                "force": self.force,
            }),
        )
    }
}

/// Set the currently active profile.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentProfile {
    /// Name of the desired profile.
    #[builder(setter(into))]
    pub profile_name: String,
}

impl Request for SetCurrentProfile {
    const REQUEST_TYPE: &'static str = "SetCurrentProfile";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "profile-name": self.profile_name,
            }),
        )
    }
}

/// Get the name of the current profile.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentProfile {}

impl Request for GetCurrentProfile {
    const REQUEST_TYPE: &'static str = "GetCurrentProfile";
    type Response = responses::GetCurrentProfile;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get a list of available profiles.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListProfiles {}

impl Request for ListProfiles {
    const REQUEST_TYPE: &'static str = "ListProfiles";
    type Response = responses::ListProfiles;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Toggle recording on or off.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopRecording {}

impl Request for StartStopRecording {
    const REQUEST_TYPE: &'static str = "StartStopRecording";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Start recording. Will return an error if recording is already active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartRecording {}

impl Request for StartRecording {
    const REQUEST_TYPE: &'static str = "StartRecording";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Stop recording. Will return an error if recording is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopRecording {}

impl Request for StopRecording {
    const REQUEST_TYPE: &'static str = "StopRecording";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Pause the current recording. Returns an error if recording is not active or already paused.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct PauseRecording {}

impl Request for PauseRecording {
    const REQUEST_TYPE: &'static str = "PauseRecording";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Resume/unpause the current recording (if paused). Returns an error if recording is not active or not paused.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ResumeRecording {}

impl Request for ResumeRecording {
    const REQUEST_TYPE: &'static str = "ResumeRecording";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Please note: if SetRecordingFolder is called while a recording is in progress, the change won't be applied immediately and will be effective on the next recording.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetRecordingFolder {
    /// Path of the recording folder.
    #[builder(setter(into))]
    pub rec_folder: String,
}

impl Request for SetRecordingFolder {
    const REQUEST_TYPE: &'static str = "SetRecordingFolder";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "rec-folder": self.rec_folder,
            }),
        )
    }
}

/// Get the path of the current recording folder.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetRecordingFolder {}

impl Request for GetRecordingFolder {
    const REQUEST_TYPE: &'static str = "GetRecordingFolder";
    type Response = responses::GetRecordingFolder;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Toggle the Replay Buffer on/off.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopReplayBuffer {}

impl Request for StartStopReplayBuffer {
    const REQUEST_TYPE: &'static str = "StartStopReplayBuffer";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Start recording into the Replay Buffer. Will return an error if the Replay Buffer is already active or if the "Save Replay Buffer" hotkey is not set in OBS' settings. Setting this hotkey is mandatory, even when triggering saves only through obs-websocket.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartReplayBuffer {}

impl Request for StartReplayBuffer {
    const REQUEST_TYPE: &'static str = "StartReplayBuffer";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Stop recording into the Replay Buffer. Will return an error if the Replay Buffer is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopReplayBuffer {}

impl Request for StopReplayBuffer {
    const REQUEST_TYPE: &'static str = "StopReplayBuffer";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Flush and save the contents of the Replay Buffer to disk. This is basically the same as triggering the "Save Replay Buffer" hotkey. Will return an error if the Replay Buffer is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SaveReplayBuffer {}

impl Request for SaveReplayBuffer {
    const REQUEST_TYPE: &'static str = "SaveReplayBuffer";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Change the active scene collection.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentSceneCollection {
    /// Name of the desired scene collection.
    #[builder(setter(into))]
    pub sc_name: String,
}

impl Request for SetCurrentSceneCollection {
    const REQUEST_TYPE: &'static str = "SetCurrentSceneCollection";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sc-name": self.sc_name,
            }),
        )
    }
}

/// Get the name of the current scene collection.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentSceneCollection {}

impl Request for GetCurrentSceneCollection {
    const REQUEST_TYPE: &'static str = "GetCurrentSceneCollection";
    type Response = responses::GetCurrentSceneCollection;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// List available scene collections
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListSceneCollections {}

impl Request for ListSceneCollections {
    const REQUEST_TYPE: &'static str = "ListSceneCollections";
    type Response = responses::ListSceneCollections;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Gets the scene specific properties of the specified source item. Coordinates are relative to the item's parent (the scene or group it belongs to).
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneItemProperties {
    /// the name of the scene that the source item belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    pub scene_name: Option<String>,
    /// The name of the source.
    #[builder(setter(into))]
    pub item: String,
}

impl Request for GetSceneItemProperties {
    const REQUEST_TYPE: &'static str = "GetSceneItemProperties";
    type Response = responses::GetSceneItemProperties;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "scene-name": self.scene_name,
                "item": self.item,
            }),
        )
    }
}

/// Sets the scene specific properties of a source. Unspecified properties will remain unchanged. Coordinates are relative to the item's parent (the scene or group it belongs to).
#[derive(Debug, TypedBuilder, PartialEq)]
pub struct SetSceneItemProperties {
    /// the name of the scene that the source item belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    pub scene_name: Option<String>,
    /// The name of the source.
    #[builder(setter(into))]
    pub item: String,
    /// The new x position of the source.
    #[builder(default, setter(strip_option))]
    pub position_x: Option<f64>,
    /// The new y position of the source.
    #[builder(default, setter(strip_option))]
    pub position_y: Option<f64>,
    /// The new alignment of the source.
    #[builder(default, setter(strip_option))]
    pub position_alignment: Option<i32>,
    /// The new clockwise rotation of the item in degrees.
    #[builder(default, setter(strip_option))]
    pub rotation: Option<f64>,
    /// The new x scale of the item.
    #[builder(default, setter(strip_option))]
    pub scale_x: Option<f64>,
    /// The new y scale of the item.
    #[builder(default, setter(strip_option))]
    pub scale_y: Option<f64>,
    /// The new amount of pixels cropped off the top of the source before scaling.
    #[builder(default, setter(strip_option))]
    pub crop_top: Option<i32>,
    /// The new amount of pixels cropped off the bottom of the source before scaling.
    #[builder(default, setter(strip_option))]
    pub crop_bottom: Option<i32>,
    /// The new amount of pixels cropped off the left of the source before scaling.
    #[builder(default, setter(strip_option))]
    pub crop_left: Option<i32>,
    /// The new amount of pixels cropped off the right of the source before scaling.
    #[builder(default, setter(strip_option))]
    pub crop_right: Option<i32>,
    /// The new visibility of the source. 'true' shows source, 'false' hides source.
    #[builder(default, setter(strip_option))]
    pub visible: Option<bool>,
    /// The new locked status of the source. 'true' keeps it in its current position, 'false' allows movement.
    #[builder(default, setter(strip_option))]
    pub locked: Option<bool>,
    /// The new bounds type of the source.
    #[builder(default, setter(strip_option))]
    pub bounds_type: Option<BoundsType>,
    /// The new alignment of the bounding box. (0-2, 4-6, 8-10)
    #[builder(default, setter(strip_option))]
    pub bounds_alignment: Option<i32>,
    /// The new width of the bounding box.
    #[builder(default, setter(strip_option))]
    pub bounds_x: Option<f64>,
    /// The new height of the bounding box.
    #[builder(default, setter(strip_option))]
    pub bounds_y: Option<f64>,
}

impl Request for SetSceneItemProperties {
    const REQUEST_TYPE: &'static str = "SetSceneItemProperties";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "message-id": message_id,
                "request-type": Self::REQUEST_TYPE,
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
            }),
        )
    }
}

/// Reset a scene item.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ResetSceneItem {
    /// Name of the scene the source belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    pub scene_name: Option<String>,
    /// Name of the source item.
    #[builder(setter(into))]
    pub item: String,
}

impl Request for ResetSceneItem {
    const REQUEST_TYPE: &'static str = "ResetSceneItem";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "scene-name": self.scene_name,
                "item": self.item,
            }),
        )
    }
}

/// Deletes a scene item.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DeleteSceneItem {
    /// Name of the scene the source belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    pub scene: Option<String>,
    /// Id or name of the scene item, prefer id, including both is acceptable.
    #[builder(default, setter(strip_option))]
    pub item_id: Option<ItemId>,
}

impl Request for DeleteSceneItem {
    const REQUEST_TYPE: &'static str = "DeleteSceneItem";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "scene": self.scene,
                "item": {
                    "id": item_id,
                    "name": item_name,
                },
            }),
        )
    }
}

/// Duplicates a scene item.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DuplicateSceneItem {
    /// Name of the scene to copy the item from. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    pub from_scene: Option<String>,
    /// Name of the scene to create the item in. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    pub to_scene: Option<String>,
    /// Id or name of the scene item, prefer id, including both is acceptable.
    #[builder(default, setter(strip_option))]
    pub item_id: Option<ItemId>,
}

impl Request for DuplicateSceneItem {
    const REQUEST_TYPE: &'static str = "DuplicateSceneItem";
    type Response = responses::DuplicateSceneItem;

    fn to_json(&self) -> (String, Value) {
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "fromScene": self.from_scene,
                "toScene": self.to_scene,
                "item": {
                    "name": item_name,
                    "id": item_id,
                },
            }),
        )
    }
}

/// Switch to the specified scene.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentScene {
    /// Name of the scene to switch to.
    #[builder(setter(into))]
    pub scene_name: String,
}

impl Request for SetCurrentScene {
    const REQUEST_TYPE: &'static str = "SetCurrentScene";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "scene-name": self.scene_name,
            }),
        )
    }
}

/// Get the current scene's name and source items.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentScene {}

impl Request for GetCurrentScene {
    const REQUEST_TYPE: &'static str = "GetCurrentScene";
    type Response = responses::GetCurrentScene;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get a list of scenes in the currently active profile.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneList {}

impl Request for GetSceneList {
    const REQUEST_TYPE: &'static str = "GetSceneList";
    type Response = responses::GetSceneList;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Changes the order of scene items in the requested scene.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSceneItems {
    /// Name of the scene to reorder (defaults to current).
    #[builder(default, setter(strip_option, into))]
    pub scene: Option<String>,
    /// Ordered list of objects with name and/or id specified. Id preferred due to uniqueness per scene
    #[builder(default, setter(strip_option))]
    pub items: Option<Vec<ItemId>>,
}

impl Request for ReorderSceneItems {
    const REQUEST_TYPE: &'static str = "ReorderSceneItems";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
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
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "scene": self.scene,
                "items": items,
            }),
        )
    }
}

/// List all sources available in the running OBS instance
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourcesList {}

impl Request for GetSourcesList {
    const REQUEST_TYPE: &'static str = "GetSourcesList";
    type Response = responses::GetSourcesList;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get a list of all available sources types
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceTypesList {}

impl Request for GetSourceTypesList {
    const REQUEST_TYPE: &'static str = "GetSourceTypesList";
    type Response = responses::GetSourceTypesList;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get the volume of the specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVolume {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for GetVolume {
    const REQUEST_TYPE: &'static str = "GetVolume";
    type Response = responses::GetVolume;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

/// Set the volume of the specified source.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetVolume {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
    /// Desired volume. Must be between 0.0 and 1.0.
    pub volume: f64,
}

impl Request for SetVolume {
    const REQUEST_TYPE: &'static str = "SetVolume";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
                "volume": self.volume,
            }),
        )
    }
}

/// Get the mute status of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetMute {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for GetMute {
    const REQUEST_TYPE: &'static str = "GetMute";
    type Response = responses::GetMute;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

/// Sets the mute status of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetMute {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
    /// Desired mute status.
    pub mute: bool,
}

impl Request for SetMute {
    const REQUEST_TYPE: &'static str = "SetMute";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
                "mute": self.mute,
            }),
        )
    }
}

/// Inverts the mute status of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleMute {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for ToggleMute {
    const REQUEST_TYPE: &'static str = "ToggleMute";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

/// Set the audio sync offset of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSyncOffset {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
    /// The desired audio sync offset (in nanoseconds).
    pub offset: i32,
}

impl Request for SetSyncOffset {
    const REQUEST_TYPE: &'static str = "SetSyncOffset";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
                "offset": self.offset
            }),
        )
    }
}

/// Get the audio sync offset of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSyncOffset {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for GetSyncOffset {
    const REQUEST_TYPE: &'static str = "GetSyncOffset";
    type Response = responses::GetSyncOffset;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

/// Get settings of the specified source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceSettings {
    /// Source name.
    #[builder(setter(into))]
    pub source_name: String,
    /// Type of the specified source. Useful for type-checking if you expect a specific settings schema.
    #[builder(default, setter(strip_option, into))]
    pub source_type: Option<SourceKind>,
}

impl Request for GetSourceSettings {
    const REQUEST_TYPE: &'static str = "GetSourceSettings";
    type Response = responses::GetSourceSettings;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "sourceType": self.source_type,
            }),
        )
    }
}

/// Set settings of the specified source.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceSettings {
    /// Source name.
    #[builder(setter(into))]
    pub source_name: String,
    /// Type of the specified source. Useful for type-checking to avoid settings a set of settings incompatible with the actual source's type.
    #[builder(default, setter(strip_option))]
    pub source_type: Option<SourceKind>,
    /// Source settings (varies between source types, may require some probing around).
    // todo: serialize properly
    pub source_settings: Value,
}

impl Request for SetSourceSettings {
    const REQUEST_TYPE: &'static str = "SetSourceSettings";
    type Response = responses::SetSourceSettings;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "sourceType": self.source_type,
                "sourceSettings": self.source_settings,
            }),
        )
    }
}

/// Get the current properties of a Text GDI Plus source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextGDIPlusProperties {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for GetTextGDIPlusProperties {
    const REQUEST_TYPE: &'static str = "GetTextGDIPlusProperties";
    type Response = responses::GetTextGDIPlusProperties;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Set the current properties of a Text GDI Plus source.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetTextGDIPlusProperties {
    /// Name of the source.
    #[builder(setter(into))]
    pub source: String,
    /// Text Alignment.
    #[builder(default, setter(strip_option))]
    pub align: Option<Alignment>,
    /// Background color.
    #[builder(default, setter(strip_option))]
    pub bk_color: Option<i32>,
    /// Background opacity (0-100).
    #[builder(default, setter(strip_option))]
    pub bk_opacity: Option<i32>,
    /// Chat log.
    #[builder(default, setter(strip_option))]
    pub chatlog: Option<bool>,
    /// Chat log lines.
    #[builder(default, setter(strip_option))]
    pub chatlog_lines: Option<i32>,
    /// Text color.
    #[builder(default, setter(strip_option))]
    pub color: Option<i32>,
    /// Extents wrap.
    #[builder(default, setter(strip_option))]
    pub extents: Option<bool>,
    /// Extents cx.
    #[builder(default, setter(strip_option))]
    pub extents_cx: Option<i32>,
    /// Extents cy.
    #[builder(default, setter(strip_option))]
    pub extents_cy: Option<i32>,
    /// File path name.
    #[builder(default, setter(strip_option, into))]
    pub file: Option<String>,
    /// Read text from the specified file.
    #[builder(default, setter(strip_option))]
    pub read_from_file: Option<bool>,
    /// Font face.
    #[builder(default, setter(strip_option, into))]
    pub font_face: Option<String>,
    /// Font text styling flag.
    #[builder(default, setter(strip_option))]
    pub font_flags: Option<i32>,
    /// Font text size.
    #[builder(default, setter(strip_option))]
    pub font_size: Option<i32>,
    /// Font Style (unknown function).
    #[builder(default, setter(strip_option, into))]
    pub font_style: Option<String>,
    /// Gradient enabled.
    #[builder(default, setter(strip_option))]
    pub gradient: Option<bool>,
    /// Gradient color.
    #[builder(default, setter(strip_option))]
    pub gradient_color: Option<i32>,
    /// Gradient direction.
    #[builder(default, setter(strip_option))]
    pub gradient_dir: Option<f64>,
    /// Gradient opacity (0-100).
    #[builder(default, setter(strip_option))]
    pub gradient_opacity: Option<i32>,
    /// Outline.
    #[builder(default, setter(strip_option))]
    pub outline: Option<bool>,
    /// Outline color.
    #[builder(default, setter(strip_option))]
    pub outline_color: Option<i32>,
    /// Outline size.
    #[builder(default, setter(strip_option))]
    pub outline_size: Option<i32>,
    /// Outline opacity (0-100).
    #[builder(default, setter(strip_option))]
    pub outline_opacity: Option<i32>,
    /// Text content to be displayed.
    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    /// Text vertical alignment.
    #[builder(default, setter(strip_option, into))]
    pub valign: Option<String>,
    /// Vertical text enabled.
    #[builder(default, setter(strip_option, into))]
    pub vertical: Option<String>,
    /// Visibility of the scene item.
    #[builder(default, setter(strip_option))]
    pub render: Option<bool>,
}

impl Request for SetTextGDIPlusProperties {
    const REQUEST_TYPE: &'static str = "SetTextGDIPlusProperties";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
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
            }),
        )
    }
}

/// Get the current properties of a Text Freetype 2 source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextFreetype2Properties {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for GetTextFreetype2Properties {
    const REQUEST_TYPE: &'static str = "GetTextFreetype2Properties";
    type Response = responses::GetTextFreetype2Properties;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

/// Set the current properties of a Text Freetype 2 source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTextFreetype2Properties {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
    /// Gradient top color.
    #[builder(default, setter(strip_option))]
    pub color_1: Option<i32>,
    /// Gradient bottom color.
    #[builder(default, setter(strip_option))]
    pub color_2: Option<i32>,
    /// Custom width (0 to disable).
    #[builder(default, setter(strip_option))]
    pub custom_width: Option<i32>,
    /// Drop shadow.
    #[builder(default, setter(strip_option))]
    pub drop_shadow: Option<bool>,
    /// Font face.
    #[builder(default, setter(strip_option, into))]
    pub font_face: Option<String>,
    /// Font text styling flag.
    #[builder(default, setter(strip_option))]
    pub font_flags: Option<i32>,
    /// Font text size.
    #[builder(default, setter(strip_option))]
    pub font_size: Option<i32>,
    /// Font Style (unknown function).
    #[builder(default, setter(strip_option, into))]
    pub font_style: Option<String>,
    /// Read text from the specified file.
    #[builder(default, setter(strip_option))]
    pub from_file: Option<bool>,
    /// Chat log.
    #[builder(default, setter(strip_option))]
    pub log_mode: Option<bool>,
    /// Outline.
    #[builder(default, setter(strip_option))]
    pub outline: Option<bool>,
    /// Text content to be displayed.
    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    /// File path.
    #[builder(default, setter(strip_option, into))]
    pub text_file: Option<String>,
    /// Word wrap.
    #[builder(default, setter(strip_option))]
    pub word_wrap: Option<bool>,
}

impl Request for SetTextFreetype2Properties {
    const REQUEST_TYPE: &'static str = "SetTextFreetype2Properties";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
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
            }),
        )
    }
}

/// Get current properties for a Browser Source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetBrowserSourceProperties {
    /// Source name.
    #[builder(setter(into))]
    pub source: String,
}

impl Request for GetBrowserSourceProperties {
    const REQUEST_TYPE: &'static str = "GetBrowserSourceProperties";
    type Response = responses::GetBrowserSourceProperties;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "source": self.source,
            }),
        )
    }
}

/// Set current properties for a Browser Source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetBrowserSourceProperties {
    /// Name of the source.
    #[builder(setter(into))]
    pub source: String,
    /// Indicates that a local file is in use.
    #[builder(default, setter(strip_option))]
    pub is_local_file: Option<bool>,
    /// file path.
    #[builder(default, setter(strip_option, into))]
    pub local_file: Option<String>,
    /// Url.
    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,
    /// CSS to inject.
    #[builder(default, setter(strip_option, into))]
    pub css: Option<String>,
    /// Width.
    #[builder(default, setter(strip_option))]
    pub width: Option<i32>,
    /// Height.
    #[builder(default, setter(strip_option))]
    pub height: Option<i32>,
    /// Framerate.
    #[builder(default, setter(strip_option))]
    pub fps: Option<i32>,
    /// Indicates whether the source should be shutdown when not visible.
    #[builder(default, setter(strip_option))]
    pub shutdown: Option<bool>,
    /// Visibility of the scene item.
    #[builder(default, setter(strip_option))]
    pub render: Option<bool>,
}

impl Request for SetBrowserSourceProperties {
    const REQUEST_TYPE: &'static str = "SetBrowserSourceProperties";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
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
            }),
        )
    }
}

/// Get configured special sources like Desktop Audio and Mic/Aux sources.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSpecialSources {}

impl Request for GetSpecialSources {
    const REQUEST_TYPE: &'static str = "GetSpecialSources";
    type Response = responses::GetSpecialSources;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// List filters applied to a source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilters {
    /// Source name
    #[builder(setter(into))]
    pub source_name: String,
}

impl Request for GetSourceFilters {
    const REQUEST_TYPE: &'static str = "GetSourceFilters";
    type Response = responses::GetSourceFilters;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
            }),
        )
    }
}

/// List filters applied to a source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilterInfo {
    /// Source name
    #[builder(setter(into))]
    pub source_name: String,
    /// Source filter name
    #[builder(setter(into))]
    pub filter_name: String,
}

impl Request for GetSourceFilterInfo {
    const REQUEST_TYPE: &'static str = "GetSourceFilterInfo";
    type Response = responses::GetSourceFilterInfo;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
            }),
        )
    }
}

/// Add a new filter to a source. Available source types along with their settings properties are available from GetSourceTypesList.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct AddFilterToSource {
    /// Name of the source on which the filter is added
    #[builder(setter(into))]
    pub source_name: String,
    /// Name of the new filter
    #[builder(setter(into))]
    pub filter_name: String,
    /// Filter type
    #[builder(setter(into))]
    pub filter_type: FilterType,
    /// Filter settings
    // todo: serialize properly
    pub filter_settings: Value,
}

impl Request for AddFilterToSource {
    const REQUEST_TYPE: &'static str = "AddFilterToSource";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
                "filterType": self.filter_type,
                "filterSettings": self.filter_settings,
            }),
        )
    }
}

/// Remove a filter from a source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct RemoveFilterFromSource {
    /// Name of the source from which the specified filter is removed
    #[builder(setter(into))]
    pub source_name: String,
    /// Name of the filter to remove
    #[builder(setter(into))]
    pub filter_name: String,
}

impl Request for RemoveFilterFromSource {
    const REQUEST_TYPE: &'static str = "RemoveFilterFromSource";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
            }),
        )
    }
}

/// Move a filter in the chain (absolute index positioning)
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSourceFilter {
    /// Name of the source to which the filter belongs
    #[builder(setter(into))]
    pub source_name: String,
    /// Name of the filter to reorder
    #[builder(setter(into))]
    pub filter_name: String,
    /// Desired position of the filter in the chain
    pub new_index: i32,
}

impl Request for ReorderSourceFilter {
    const REQUEST_TYPE: &'static str = "ReorderSourceFilter";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
                "newIndex": self.new_index,
            }),
        )
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MovementType {
    Up,
    Down,
    Top,
    Bottom,
}

/// Move a filter in the chain (relative positioning)
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct MoveSourceFilter {
    /// Name of the source to which the filter belongs
    #[builder(setter(into))]
    pub source_name: String,
    /// Name of the filter to reorder
    #[builder(setter(into))]
    pub filter_name: String,
    /// How to move the filter around in the source's filter chain.
    pub movement_type: MovementType,
}

impl Request for MoveSourceFilter {
    const REQUEST_TYPE: &'static str = "MoveSourceFilter";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
                "movementType": self.movement_type,
            }),
        )
    }
}

/// Update settings of a filter
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceFilterSettings {
    /// Name of the source to which the filter belongs
    #[builder(setter(into))]
    pub source_name: String,
    /// Name of the filter to reconfigure
    #[builder(setter(into))]
    pub filter_name: String,
    /// New settings. These will be merged to the current filter settings.
    // todo: serialize properly
    pub filter_settings: Value,
}

impl Request for SetSourceFilterSettings {
    const REQUEST_TYPE: &'static str = "SetSourceFilterSettings";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
                "filterSettings": self.filter_settings,
            }),
        )
    }
}

/// Change the visibility/enabled state of a filter
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSourceFilterVisibility {
    /// Source name
    #[builder(setter(into))]
    pub source_name: String,
    /// Source filter name
    #[builder(setter(into))]
    pub filter_name: String,
    /// New filter state
    pub filter_enabled: bool,
}

impl Request for SetSourceFilterVisibility {
    const REQUEST_TYPE: &'static str = "SetSourceFilterVisibility";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "filterName": self.filter_name,
                "filterEnabled": self.filter_enabled,
            }),
        )
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

/// At least embedPictureFormat or saveToFilePath must be specified.
/// Clients can specify width and height parameters to receive scaled pictures. Aspect ratio is preserved if only one of these two parameters is specified.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TakeSourceScreenshot {
    /// Source name. Note that, since scenes are also sources, you can also provide a scene name.
    #[builder(setter(into))]
    pub source_name: String,
    /// Format of the Data URI encoded picture.
    #[builder(default, setter(strip_option))]
    pub embed_picture_format: Option<EmbedPictureFormat>,
    /// Full file path (file extension included) where the captured image is to be saved. Can be in a format different from pictureFormat. Can be a relative path.
    #[builder(default, setter(strip_option, into))]
    pub save_to_file_path: Option<String>,
    /// Screenshot width. Defaults to the source's base width.
    #[builder(default, setter(strip_option))]
    pub width: Option<i32>,
    /// Screenshot height. Defaults to the source's base height.
    #[builder(default, setter(strip_option))]
    pub height: Option<i32>,
}

impl Request for TakeSourceScreenshot {
    const REQUEST_TYPE: &'static str = "TakeSourceScreenshot";
    type Response = responses::TakeSourceScreenshot;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "sourceName": self.source_name,
                "embedPictureFormat": self.embed_picture_format,
                "saveToFilePath": self.save_to_file_path,
                "width": self.width,
                "height": self.height,
            }),
        )
    }
}

/// Get current streaming and recording status.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStreamingStatus {}

impl Request for GetStreamingStatus {
    const REQUEST_TYPE: &'static str = "GetStreamingStatus";
    type Response = responses::GetStreamingStatus;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Toggle streaming on or off.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopStreaming {}

impl Request for StartStopStreaming {
    const REQUEST_TYPE: &'static str = "StartStopStreaming";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Start streaming. Will return an error if streaming is already active.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct StartStreaming {
    /// If specified ensures the type of stream matches the given type (usually 'rtmp_custom' or 'rtmp_common'). If the currently configured stream type does not match the given stream type, all settings must be specified in the settings object or an error will occur when starting the stream.
    #[builder(default, setter(strip_option, into))]
    pub stream_type: Option<String>,
    /// Adds the given object parameters as encoded query string parameters to the 'key' of the RTMP stream. Used to pass data to the RTMP service about the streaming. May be any String, Numeric, or Boolean field.
    #[builder(default, setter(strip_option))]
    pub stream_metadata: Option<Value>,
    /// The publish URL.
    #[builder(default, setter(strip_option, into))]
    pub stream_server: Option<String>,
    /// The publish key of the stream.
    #[builder(default, setter(strip_option, into))]
    pub stream_key: Option<String>,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    #[builder(default, setter(strip_option, into))]
    pub stream_use_auth: Option<String>,
    /// If authentication is enabled, the username for the streaming server. Ignored if use-auth is not set to true.
    #[builder(default, setter(strip_option, into))]
    pub stream_username: Option<String>,
    /// If authentication is enabled, the password for the streaming server. Ignored if use-auth is not set to true.
    #[builder(default, setter(strip_option, into))]
    pub stream_password: Option<String>,
}

impl Request for StartStreaming {
    const REQUEST_TYPE: &'static str = "StartStreaming";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
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
            }),
        )
    }
}

/// Stop streaming. Will return an error if streaming is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopStreaming {}

impl Request for StopStreaming {
    const REQUEST_TYPE: &'static str = "StopStreaming";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Sets one or more attributes of the current streaming server settings. Any options not passed will remain unchanged. Returns the updated settings in response. If 'type' is different than the current streaming service type, all settings are required. Returns the full settings of the stream (the same as GetStreamSettings).
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetStreamSettings {
    /// The type of streaming service configuration, usually rtmp_custom or rtmp_common.
    #[builder(default, setter(strip_option, into))]
    pub stream_type: Option<String>,
    /// The publish URL.
    #[builder(default, setter(strip_option, into))]
    pub server: Option<String>,
    /// The publish key.
    #[builder(default, setter(strip_option, into))]
    pub key: Option<String>,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    #[builder(default, setter(strip_option, into))]
    pub use_auth: Option<String>,
    /// The username for the streaming service.
    #[builder(default, setter(strip_option, into))]
    pub username: Option<String>,
    /// The password for the streaming service.
    #[builder(default, setter(strip_option, into))]
    pub password: Option<String>,
    /// Persist the settings to disk.
    pub save: bool,
}

impl Request for SetStreamSettings {
    const REQUEST_TYPE: &'static str = "SetStreamSettings";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
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
            }),
        )
    }
}

/// Get the current streaming server settings.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStreamSettings {}

impl Request for GetStreamSettings {
    const REQUEST_TYPE: &'static str = "GetStreamSettings";
    type Response = responses::GetStreamSettings;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Save the current streaming server settings to disk.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SaveStreamSettings {}

impl Request for SaveStreamSettings {
    const REQUEST_TYPE: &'static str = "SaveStreamSettings";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Send the provided text as embedded CEA-608 caption data. As of OBS Studio 23.1, captions are not yet available on Linux.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SendCaptions {
    /// Captions text
    #[builder(setter(into))]
    pub text: String,
}

impl Request for SendCaptions {
    const REQUEST_TYPE: &'static str = "SendCaptions";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "text": self.text,
            }),
        )
    }
}

/// Indicates if Studio Mode is currently enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStudioModeStatus {}

impl Request for GetStudioModeStatus {
    const REQUEST_TYPE: &'static str = "GetStudioModeStatus";
    type Response = responses::GetStudioModeStatus;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get the name of the currently previewed scene and its list of sources. Will return an error if Studio Mode is not enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetPreviewScene {}

impl Request for GetPreviewScene {
    const REQUEST_TYPE: &'static str = "GetPreviewScene";
    type Response = responses::GetPreviewScene;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Set the active preview scene. Will return an error if Studio Mode is not enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetPreviewScene {
    /// The name of the scene to preview.
    #[builder(setter(into))]
    pub scene_name: String,
}

impl Request for SetPreviewScene {
    const REQUEST_TYPE: &'static str = "SetPreviewScene";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "scene-name": self.scene_name,
            }),
        )
    }
}

/// Transitions the currently previewed scene to the main output. Will return an error if Studio Mode is not enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TransitionToProgram {
    /// Name of the transition.
    #[builder(default, setter(strip_option, into))]
    pub with_transition_name: Option<String>,
    /// Transition duration (in milliseconds).
    #[builder(default, setter(strip_option, into))]
    pub with_transition_duration: Option<String>,
}

impl Request for TransitionToProgram {
    const REQUEST_TYPE: &'static str = "TransitionToProgram";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "with-transition": {
                    "name": self.with_transition_name,
                    "duration": self.with_transition_duration,
                }
            }),
        )
    }
}

/// Enables Studio Mode.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct EnableStudioMode {}

impl Request for EnableStudioMode {
    const REQUEST_TYPE: &'static str = "EnableStudioMode";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Disables Studio Mode.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DisableStudioMode {}

impl Request for DisableStudioMode {
    const REQUEST_TYPE: &'static str = "DisableStudioMode";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Toggles Studio Mode.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleStudioMode {}

impl Request for ToggleStudioMode {
    const REQUEST_TYPE: &'static str = "ToggleStudioMode";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// List of all transitions available in the frontend's dropdown menu.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTransitionList {}

impl Request for GetTransitionList {
    const REQUEST_TYPE: &'static str = "GetTransitionList";
    type Response = responses::GetTransitionList;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Get the name of the currently selected transition in the frontend's dropdown menu.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentTransition {}

impl Request for GetCurrentTransition {
    const REQUEST_TYPE: &'static str = "GetCurrentTransition";
    type Response = responses::GetCurrentTransition;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

/// Set the active transition.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentTransition {
    /// The name of the transition.
    #[builder(setter(into))]
    pub transition_name: String,
}

impl Request for SetCurrentTransition {
    const REQUEST_TYPE: &'static str = "SetCurrentTransition";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "transition-name": self.transition_name,
            }),
        )
    }
}

/// Set the duration of the currently selected transition if supported.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTransitionDuration {
    /// Desired duration of the transition (in milliseconds).
    pub duration: i32,
}

impl Request for SetTransitionDuration {
    const REQUEST_TYPE: &'static str = "SetTransitionDuration";
    type Response = responses::Empty;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
                "duration": self.duration,
            }),
        )
    }
}

/// Get the duration of the currently selected transition if supported.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTransitionDuration {}

impl Request for GetTransitionDuration {
    const REQUEST_TYPE: &'static str = "GetTransitionDuration";
    type Response = responses::GetTransitionDuration;

    fn to_json(&self) -> (String, Value) {
        let message_id = make_message_id();
        (
            message_id.clone(),
            json!({
                "request-type": Self::REQUEST_TYPE,
                "message-id": message_id,
            }),
        )
    }
}

// #### other typedefs ####
#[derive(Debug, PartialEq, Eq)]
pub enum ItemId {
    /// Name of a scene item. Sufficiently unique if no scene items share sources within the scene.
    Name(String),
    /// Id of a specific scene item. Unique on a scene by scene basis.
    Id(i32),
}

impl ItemId {
    fn to_name(&self) -> Option<&str> {
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
