//! Request types. Sent to the server.
//!
//! The request types will generate a running message-id by default, but they also support defining custom message-ids.
//! When using custom IDs, avoid reusing them and if also using default IDs, avoid using custom ones in the form `_{integer}` to avoid clashing which may cause responses to be parsed incorrectly.

use crate::responses;

pub use crate::common_types::*;

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

    // returns the message_id
    fn get_message_id(&self) -> &str;

    // converts the struct into a JSON value
    fn to_json(&self) -> Value;
}

// creates a default value for message-id, using a running id
fn make_message_id() -> String {
    format!("_{}", RUNNING_MESSAGE_ID.fetch_add(1, Ordering::Relaxed))
}

/// Returns the latest version of the plugin and the API.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVersion {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetVersion {
    const REQUEST_TYPE: &'static str = "GetVersion";
    type Response = responses::GetVersion;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Tells the client if authentication is required.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetAuthRequired {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetAuthRequired {
    const REQUEST_TYPE: &'static str = "GetAuthRequired";
    type Response = responses::GetAuthRequired;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Attempt to authenticate the client to the server.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct Authenticate {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Response to the auth challenge.
    #[builder(setter(into))]
    auth: String,
}

impl Request for Authenticate {
    const REQUEST_TYPE: &'static str = "Authenticate";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "auth": self.auth,
        })
    }
}

/// Enable/disable sending of the Heartbeat event
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetHeartbeat {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Starts/Stops emitting heartbeat messages
    enable: bool,
}

impl Request for SetHeartbeat {
    const REQUEST_TYPE: &'static str = "SetHeartbeat";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "enable": self.enable,
        })
    }
}

/// Set the filename formatting string
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetFilenameFormatting {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Filename formatting string to set.
    #[builder(setter(into))]
    filename_formatting: String,
}

impl Request for SetFilenameFormatting {
    const REQUEST_TYPE: &'static str = "SetFilenameFormatting";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "filename-formatting": self.filename_formatting,
        })
    }
}

/// Get the filename formatting string
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetFilenameFormatting {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetFilenameFormatting {
    const REQUEST_TYPE: &'static str = "GetFilenameFormatting";
    type Response = responses::GetFilenameFormatting;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get OBS stats (almost the same info as provided in OBS' stats window)
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStats {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStats {
    const REQUEST_TYPE: &'static str = "GetStats";
    type Response = responses::GetStats;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Broadcast custom message to all connected WebSocket clients
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct BroadcastCustomMessage {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Identifier to be choosen by the client
    #[builder(setter(into))]
    realm: String,
    /// User-defined data
    data: Value,
}

impl Request for BroadcastCustomMessage {
    const REQUEST_TYPE: &'static str = "BroadcastCustomMessage";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "realm": self.realm,
            "data": self.data,
        })
    }
}

/// Get basic OBS video information
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVideoInfo {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetVideoInfo {
    const REQUEST_TYPE: &'static str = "GetVideoInfo";
    type Response = responses::GetVideoInfo;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// List existing outputs
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListOutputs {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ListOutputs {
    const REQUEST_TYPE: &'static str = "ListOutputs";
    type Response = responses::ListOutputs;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get information about a single output
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetOutputInfo {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Output name
    #[builder(setter(into))]
    output_name: String,
}

impl Request for GetOutputInfo {
    const REQUEST_TYPE: &'static str = "GetOutputInfo";
    type Response = responses::GetOutputInfo;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "outputName": self.output_name,
        })
    }
}

/// Start an output
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartOutput {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Output name
    #[builder(setter(into))]
    output_name: String,
}

impl Request for StartOutput {
    const REQUEST_TYPE: &'static str = "StartOutput";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "outputName": self.output_name,
        })
    }
}

/// Stop an output
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopOutput {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Output name
    #[builder(setter(into))]
    output_name: String,
    /// Force stop (default: false)
    #[builder(default, setter(strip_option))]
    force: Option<bool>,
}

impl Request for StopOutput {
    const REQUEST_TYPE: &'static str = "StopOutput";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "outputName": self.output_name,
            "force": self.force,
        })
    }
}

/// Set the currently active profile.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentProfile {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the desired profile.
    #[builder(setter(into))]
    profile_name: String,
}

impl Request for SetCurrentProfile {
    const REQUEST_TYPE: &'static str = "SetCurrentProfile";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "profile-name": self.profile_name,
        })
    }
}

/// Get the name of the current profile.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentProfile {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentProfile {
    const REQUEST_TYPE: &'static str = "GetCurrentProfile";
    type Response = responses::GetCurrentProfile;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get a list of available profiles.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListProfiles {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ListProfiles {
    const REQUEST_TYPE: &'static str = "ListProfiles";
    type Response = responses::ListProfiles;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Toggle recording on or off.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartStopRecording {
    const REQUEST_TYPE: &'static str = "StartStopRecording";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Start recording. Will return an error if recording is already active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartRecording {
    const REQUEST_TYPE: &'static str = "StartRecording";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Stop recording. Will return an error if recording is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StopRecording {
    const REQUEST_TYPE: &'static str = "StopRecording";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Pause the current recording. Returns an error if recording is not active or already paused.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct PauseRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for PauseRecording {
    const REQUEST_TYPE: &'static str = "PauseRecording";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Resume/unpause the current recording (if paused). Returns an error if recording is not active or not paused.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ResumeRecording {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ResumeRecording {
    const REQUEST_TYPE: &'static str = "ResumeRecording";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Please note: if SetRecordingFolder is called while a recording is in progress, the change won't be applied immediately and will be effective on the next recording.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetRecordingFolder {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Path of the recording folder.
    #[builder(setter(into))]
    rec_folder: String,
}

impl Request for SetRecordingFolder {
    const REQUEST_TYPE: &'static str = "SetRecordingFolder";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "rec-folder": self.rec_folder,
        })
    }
}

/// Get the path of the current recording folder.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetRecordingFolder {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetRecordingFolder {
    const REQUEST_TYPE: &'static str = "GetRecordingFolder";
    type Response = responses::GetRecordingFolder;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Toggle the Replay Buffer on/off.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartStopReplayBuffer {
    const REQUEST_TYPE: &'static str = "StartStopReplayBuffer";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Start recording into the Replay Buffer. Will return an error if the Replay Buffer is already active or if the "Save Replay Buffer" hotkey is not set in OBS' settings. Setting this hotkey is mandatory, even when triggering saves only through obs-websocket.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartReplayBuffer {
    const REQUEST_TYPE: &'static str = "StartReplayBuffer";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Stop recording into the Replay Buffer. Will return an error if the Replay Buffer is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StopReplayBuffer {
    const REQUEST_TYPE: &'static str = "StopReplayBuffer";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Flush and save the contents of the Replay Buffer to disk. This is basically the same as triggering the "Save Replay Buffer" hotkey. Will return an error if the Replay Buffer is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SaveReplayBuffer {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for SaveReplayBuffer {
    const REQUEST_TYPE: &'static str = "SaveReplayBuffer";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Change the active scene collection.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentSceneCollection {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the desired scene collection.
    #[builder(setter(into))]
    sc_name: String,
}

impl Request for SetCurrentSceneCollection {
    const REQUEST_TYPE: &'static str = "SetCurrentSceneCollection";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sc-name": self.sc_name,
        })
    }
}

/// Get the name of the current scene collection.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentSceneCollection {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentSceneCollection {
    const REQUEST_TYPE: &'static str = "GetCurrentSceneCollection";
    type Response = responses::GetCurrentSceneCollection;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// List available scene collections
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ListSceneCollections {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ListSceneCollections {
    const REQUEST_TYPE: &'static str = "ListSceneCollections";
    type Response = responses::ListSceneCollections;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Gets the scene specific properties of the specified source item. Coordinates are relative to the item's parent (the scene or group it belongs to).
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneItemProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// the name of the scene that the source item belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    scene_name: Option<String>,
    /// The name of the source.
    #[builder(setter(into))]
    item: String,
}

impl Request for GetSceneItemProperties {
    const REQUEST_TYPE: &'static str = "GetSceneItemProperties";
    type Response = responses::GetSceneItemProperties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "scene-name": self.scene_name,
            "item": self.item,
        })
    }
}

/// Sets the scene specific properties of a source. Unspecified properties will remain unchanged. Coordinates are relative to the item's parent (the scene or group it belongs to).
#[derive(Debug, TypedBuilder, PartialEq)]
pub struct SetSceneItemProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// the name of the scene that the source item belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    scene_name: Option<String>,
    /// The name of the source.
    #[builder(setter(into))]
    item: String,
    /// The new x position of the source.
    #[builder(default, setter(strip_option))]
    position_x: Option<f64>,
    /// The new y position of the source.
    #[builder(default, setter(strip_option))]
    position_y: Option<f64>,
    /// The new alignment of the source.
    #[builder(default, setter(strip_option))]
    position_alignment: Option<i32>,
    /// The new clockwise rotation of the item in degrees.
    #[builder(default, setter(strip_option))]
    rotation: Option<f64>,
    /// The new x scale of the item.
    #[builder(default, setter(strip_option))]
    scale_x: Option<f64>,
    /// The new y scale of the item.
    #[builder(default, setter(strip_option))]
    scale_y: Option<f64>,
    /// The new amount of pixels cropped off the top of the source before scaling.
    #[builder(default, setter(strip_option))]
    crop_top: Option<i32>,
    /// The new amount of pixels cropped off the bottom of the source before scaling.
    #[builder(default, setter(strip_option))]
    crop_bottom: Option<i32>,
    /// The new amount of pixels cropped off the left of the source before scaling.
    #[builder(default, setter(strip_option))]
    crop_left: Option<i32>,
    /// The new amount of pixels cropped off the right of the source before scaling.
    #[builder(default, setter(strip_option))]
    crop_right: Option<i32>,
    /// The new visibility of the source. 'true' shows source, 'false' hides source.
    #[builder(default, setter(strip_option))]
    visible: Option<bool>,
    /// The new locked status of the source. 'true' keeps it in its current position, 'false' allows movement.
    #[builder(default, setter(strip_option))]
    locked: Option<bool>,
    /// The new bounds type of the source.
    #[builder(default, setter(strip_option))]
    bounds_type: Option<BoundsType>,
    /// The new alignment of the bounding box. (0-2, 4-6, 8-10)
    #[builder(default, setter(strip_option))]
    bounds_alignment: Option<i32>,
    /// The new width of the bounding box.
    #[builder(default, setter(strip_option))]
    bounds_x: Option<f64>,
    /// The new height of the bounding box.
    #[builder(default, setter(strip_option))]
    bounds_y: Option<f64>,
}

impl Request for SetSceneItemProperties {
    const REQUEST_TYPE: &'static str = "SetSceneItemProperties";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "message-id": self.message_id,
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
        })
    }
}

/// Reset a scene item.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ResetSceneItem {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the scene the source belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    scene_name: Option<String>,
    /// Name of the source item.
    #[builder(setter(into))]
    item: String,
}

impl Request for ResetSceneItem {
    const REQUEST_TYPE: &'static str = "ResetSceneItem";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "scene-name": self.scene_name,
            "item": self.item,
        })
    }
}

/// Deletes a scene item.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DeleteSceneItem {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the scene the source belongs to. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    scene: Option<String>,
    /// Id or name of the scene item, prefer id, including both is acceptable.
    #[builder(default, setter(strip_option))]
    item_id: Option<ItemId>,
}

impl Request for DeleteSceneItem {
    const REQUEST_TYPE: &'static str = "DeleteSceneItem";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "scene": self.scene,
            "item": {
                "id": item_id,
                "name": item_name,
            },
        })
    }
}

/// Duplicates a scene item.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DuplicateSceneItem {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the scene to copy the item from. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    from_scene: Option<String>,
    /// Name of the scene to create the item in. Defaults to the current scene.
    #[builder(default, setter(strip_option, into))]
    to_scene: Option<String>,
    /// Id or name of the scene item, prefer id, including both is acceptable.
    #[builder(default, setter(strip_option))]
    item_id: Option<ItemId>,
}

impl Request for DuplicateSceneItem {
    const REQUEST_TYPE: &'static str = "DuplicateSceneItem";
    type Response = responses::DuplicateSceneItem;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        let item_name = self.item_id.as_ref().and_then(ItemId::to_name);
        let item_id = self.item_id.as_ref().and_then(ItemId::to_id);
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "fromScene": self.from_scene,
            "toScene": self.to_scene,
            "item": {
                "name": item_name,
                "id": item_id,
            },
        })
    }
}

/// Switch to the specified scene.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the scene to switch to.
    #[builder(setter(into))]
    scene_name: String,
}

impl Request for SetCurrentScene {
    const REQUEST_TYPE: &'static str = "SetCurrentScene";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "scene-name": self.scene_name,
        })
    }
}

/// Get the current scene's name and source items.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentScene {
    const REQUEST_TYPE: &'static str = "GetCurrentScene";
    type Response = responses::GetCurrentScene;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get a list of scenes in the currently active profile.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSceneList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSceneList {
    const REQUEST_TYPE: &'static str = "GetSceneList";
    type Response = responses::GetSceneList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Changes the order of scene items in the requested scene.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSceneItems {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the scene to reorder (defaults to current).
    #[builder(default, setter(strip_option, into))]
    scene: Option<String>,
    /// Ordered list of objects with name and/or id specified. Id preferred due to uniqueness per scene
    #[builder(default, setter(strip_option))]
    items: Option<Vec<ItemId>>,
}

impl Request for ReorderSceneItems {
    const REQUEST_TYPE: &'static str = "ReorderSceneItems";
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
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "scene": self.scene,
            "items": items,
        })
    }
}

/// List all sources available in the running OBS instance
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourcesList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSourcesList {
    const REQUEST_TYPE: &'static str = "GetSourcesList";
    type Response = responses::GetSourcesList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get a list of all available sources types
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceTypesList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSourceTypesList {
    const REQUEST_TYPE: &'static str = "GetSourceTypesList";
    type Response = responses::GetSourceTypesList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get the volume of the specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetVolume {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for GetVolume {
    const REQUEST_TYPE: &'static str = "GetVolume";
    type Response = responses::GetVolume;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
        })
    }
}

/// Set the volume of the specified source.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetVolume {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
    /// Desired volume. Must be between 0.0 and 1.0.
    volume: f64,
}

impl Request for SetVolume {
    const REQUEST_TYPE: &'static str = "SetVolume";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
            "volume": self.volume,
        })
    }
}

/// Get the mute status of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetMute {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for GetMute {
    const REQUEST_TYPE: &'static str = "GetMute";
    type Response = responses::GetMute;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
        })
    }
}

/// Sets the mute status of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetMute {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
    /// Desired mute status.
    mute: bool,
}

impl Request for SetMute {
    const REQUEST_TYPE: &'static str = "SetMute";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
            "mute": self.mute,
        })
    }
}

/// Inverts the mute status of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleMute {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for ToggleMute {
    const REQUEST_TYPE: &'static str = "ToggleMute";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
        })
    }
}

/// Set the audio sync offset of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSyncOffset {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
    /// The desired audio sync offset (in nanoseconds).
    offset: i32,
}

impl Request for SetSyncOffset {
    const REQUEST_TYPE: &'static str = "SetSyncOffset";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
            "offset": self.offset
        })
    }
}

/// Get the audio sync offset of a specified source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSyncOffset {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for GetSyncOffset {
    const REQUEST_TYPE: &'static str = "GetSyncOffset";
    type Response = responses::GetSyncOffset;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
        })
    }
}

/// Get settings of the specified source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source_name: String,
    /// Type of the specified source. Useful for type-checking if you expect a specific settings schema.
    #[builder(default, setter(strip_option, into))]
    source_type: Option<String>,
}

impl Request for GetSourceSettings {
    const REQUEST_TYPE: &'static str = "GetSourceSettings";
    type Response = responses::GetSourceSettings;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "sourceType": self.source_type,
        })
    }
}

// TODO: source settings
/// Set settings of the specified source.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source_name: String,
    /// Type of the specified source. Useful for type-checking to avoid settings a set of settings incompatible with the actual source's type.
    #[builder(default, setter(strip_option, into))]
    source_type: Option<String>,
    /// Source settings (varies between source types, may require some probing around).
    source_settings: Value,
}

impl Request for SetSourceSettings {
    const REQUEST_TYPE: &'static str = "SetSourceSettings";
    type Response = responses::SetSourceSettings;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "sourceType": self.source_type,
            "sourceSettings": self.source_settings,
        })
    }
}

/// Get the current properties of a Text GDI Plus source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextGDIPlusProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for GetTextGDIPlusProperties {
    const REQUEST_TYPE: &'static str = "GetTextGDIPlusProperties";
    type Response = responses::GetTextGDIPlusProperties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// Set the current properties of a Text GDI Plus source.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetTextGDIPlusProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source.
    #[builder(setter(into))]
    source: String,
    /// Text Alignment.
    #[builder(default, setter(strip_option))]
    align: Option<Alignment>,
    /// Background color.
    #[builder(default, setter(strip_option))]
    bk_color: Option<i32>,
    /// Background opacity (0-100).
    #[builder(default, setter(strip_option))]
    bk_opacity: Option<i32>,
    /// Chat log.
    #[builder(default, setter(strip_option))]
    chatlog: Option<bool>,
    /// Chat log lines.
    #[builder(default, setter(strip_option))]
    chatlog_lines: Option<i32>,
    /// Text color.
    #[builder(default, setter(strip_option))]
    color: Option<i32>,
    /// Extents wrap.
    #[builder(default, setter(strip_option))]
    extents: Option<bool>,
    /// Extents cx.
    #[builder(default, setter(strip_option))]
    extents_cx: Option<i32>,
    /// Extents cy.
    #[builder(default, setter(strip_option))]
    extents_cy: Option<i32>,
    /// File path name.
    #[builder(default, setter(strip_option, into))]
    file: Option<String>,
    /// Read text from the specified file.
    #[builder(default, setter(strip_option))]
    read_from_file: Option<bool>,
    /// Font face.
    #[builder(default, setter(strip_option, into))]
    font_face: Option<String>,
    /// Font text styling flag.
    #[builder(default, setter(strip_option))]
    font_flags: Option<i32>,
    /// Font text size.
    #[builder(default, setter(strip_option))]
    font_size: Option<i32>,
    /// Font Style (unknown function).
    #[builder(default, setter(strip_option, into))]
    font_style: Option<String>,
    /// Gradient enabled.
    #[builder(default, setter(strip_option))]
    gradient: Option<bool>,
    /// Gradient color.
    #[builder(default, setter(strip_option))]
    gradient_color: Option<i32>,
    /// Gradient direction.
    #[builder(default, setter(strip_option))]
    gradient_dir: Option<f64>,
    /// Gradient opacity (0-100).
    #[builder(default, setter(strip_option))]
    gradient_opacity: Option<i32>,
    /// Outline.
    #[builder(default, setter(strip_option))]
    outline: Option<bool>,
    /// Outline color.
    #[builder(default, setter(strip_option))]
    outline_color: Option<i32>,
    /// Outline size.
    #[builder(default, setter(strip_option))]
    outline_size: Option<i32>,
    /// Outline opacity (0-100).
    #[builder(default, setter(strip_option))]
    outline_opacity: Option<i32>,
    /// Text content to be displayed.
    #[builder(default, setter(strip_option, into))]
    text: Option<String>,
    /// Text vertical alignment.
    #[builder(default, setter(strip_option, into))]
    valign: Option<String>,
    /// Vertical text enabled.
    #[builder(default, setter(strip_option, into))]
    vertical: Option<String>,
    /// Visibility of the scene item.
    #[builder(default, setter(strip_option))]
    render: Option<bool>,
}

impl Request for SetTextGDIPlusProperties {
    const REQUEST_TYPE: &'static str = "SetTextGDIPlusProperties";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// Get the current properties of a Text Freetype 2 source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTextFreetype2Properties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for GetTextFreetype2Properties {
    const REQUEST_TYPE: &'static str = "GetTextFreetype2Properties";
    type Response = responses::GetTextFreetype2Properties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
        })
    }
}

/// Set the current properties of a Text Freetype 2 source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTextFreetype2Properties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
    /// Gradient top color.
    #[builder(default, setter(strip_option))]
    color_1: Option<i32>,
    /// Gradient bottom color.
    #[builder(default, setter(strip_option))]
    color_2: Option<i32>,
    /// Custom width (0 to disable).
    #[builder(default, setter(strip_option))]
    custom_width: Option<i32>,
    /// Drop shadow.
    #[builder(default, setter(strip_option))]
    drop_shadow: Option<bool>,
    /// Font face.
    #[builder(default, setter(strip_option, into))]
    font_face: Option<String>,
    /// Font text styling flag.
    #[builder(default, setter(strip_option))]
    font_flags: Option<i32>,
    /// Font text size.
    #[builder(default, setter(strip_option))]
    font_size: Option<i32>,
    /// Font Style (unknown function).
    #[builder(default, setter(strip_option, into))]
    font_style: Option<String>,
    /// Read text from the specified file.
    #[builder(default, setter(strip_option))]
    from_file: Option<bool>,
    /// Chat log.
    #[builder(default, setter(strip_option))]
    log_mode: Option<bool>,
    /// Outline.
    #[builder(default, setter(strip_option))]
    outline: Option<bool>,
    /// Text content to be displayed.
    #[builder(default, setter(strip_option, into))]
    text: Option<String>,
    /// File path.
    #[builder(default, setter(strip_option, into))]
    text_file: Option<String>,
    /// Word wrap.
    #[builder(default, setter(strip_option))]
    word_wrap: Option<bool>,
}

impl Request for SetTextFreetype2Properties {
    const REQUEST_TYPE: &'static str = "SetTextFreetype2Properties";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// Get current properties for a Browser Source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetBrowserSourceProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name.
    #[builder(setter(into))]
    source: String,
}

impl Request for GetBrowserSourceProperties {
    const REQUEST_TYPE: &'static str = "GetBrowserSourceProperties";
    type Response = responses::GetBrowserSourceProperties;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "source": self.source,
        })
    }
}

/// Set current properties for a Browser Source.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetBrowserSourceProperties {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source.
    #[builder(setter(into))]
    source: String,
    /// Indicates that a local file is in use.
    #[builder(default, setter(strip_option))]
    is_local_file: Option<bool>,
    /// file path.
    #[builder(default, setter(strip_option, into))]
    local_file: Option<String>,
    /// Url.
    #[builder(default, setter(strip_option, into))]
    url: Option<String>,
    /// CSS to inject.
    #[builder(default, setter(strip_option, into))]
    css: Option<String>,
    /// Width.
    #[builder(default, setter(strip_option))]
    width: Option<i32>,
    /// Height.
    #[builder(default, setter(strip_option))]
    height: Option<i32>,
    /// Framerate.
    #[builder(default, setter(strip_option))]
    fps: Option<i32>,
    /// Indicates whether the source should be shutdown when not visible.
    #[builder(default, setter(strip_option))]
    shutdown: Option<bool>,
    /// Visibility of the scene item.
    #[builder(default, setter(strip_option))]
    render: Option<bool>,
}

impl Request for SetBrowserSourceProperties {
    const REQUEST_TYPE: &'static str = "SetBrowserSourceProperties";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// Get configured special sources like Desktop Audio and Mic/Aux sources.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSpecialSources {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetSpecialSources {
    const REQUEST_TYPE: &'static str = "GetSpecialSources";
    type Response = responses::GetSpecialSources;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// List filters applied to a source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilters {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name
    #[builder(setter(into))]
    source_name: String,
}

impl Request for GetSourceFilters {
    const REQUEST_TYPE: &'static str = "GetSourceFilters";
    type Response = responses::GetSourceFilters;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
        })
    }
}

/// List filters applied to a source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetSourceFilterInfo {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name
    #[builder(setter(into))]
    source_name: String,
    /// Source filter name
    #[builder(setter(into))]
    filter_name: String,
}

impl Request for GetSourceFilterInfo {
    const REQUEST_TYPE: &'static str = "GetSourceFilterInfo";
    type Response = responses::GetSourceFilterInfo;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "filterName": self.filter_name,
        })
    }
}

// TODO: filter settings
/// Add a new filter to a source. Available source types along with their settings properties are available from GetSourceTypesList.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct AddFilterToSource {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source on which the filter is added
    #[builder(setter(into))]
    source_name: String,
    /// Name of the new filter
    #[builder(setter(into))]
    filter_name: String,
    /// Filter type
    #[builder(setter(into))]
    filter_type: String,
    /// Filter settings
    filter_settings: Value,
}

impl Request for AddFilterToSource {
    const REQUEST_TYPE: &'static str = "AddFilterToSource";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterType": self.filter_type,
            "filterSettings": self.filter_settings,
        })
    }
}

/// Remove a filter from a source
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct RemoveFilterFromSource {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source from which the specified filter is removed
    #[builder(setter(into))]
    source_name: String,
    /// Name of the filter to remove
    #[builder(setter(into))]
    filter_name: String,
}

impl Request for RemoveFilterFromSource {
    const REQUEST_TYPE: &'static str = "RemoveFilterFromSource";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "filterName": self.filter_name,
        })
    }
}

/// Move a filter in the chain (absolute index positioning)
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ReorderSourceFilter {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source to which the filter belongs
    #[builder(setter(into))]
    source_name: String,
    /// Name of the filter to reorder
    #[builder(setter(into))]
    filter_name: String,
    /// Desired position of the filter in the chain
    new_index: i32,
}

impl Request for ReorderSourceFilter {
    const REQUEST_TYPE: &'static str = "ReorderSourceFilter";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "newIndex": self.new_index,
        })
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
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source to which the filter belongs
    #[builder(setter(into))]
    source_name: String,
    /// Name of the filter to reorder
    #[builder(setter(into))]
    filter_name: String,
    /// How to move the filter around in the source's filter chain.
    movement_type: MovementType,
}

impl Request for MoveSourceFilter {
    const REQUEST_TYPE: &'static str = "MoveSourceFilter";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "movementType": self.movement_type,
        })
    }
}

// TODO: filter settings
/// Update settings of a filter
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct SetSourceFilterSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the source to which the filter belongs
    #[builder(setter(into))]
    source_name: String,
    /// Name of the filter to reconfigure
    #[builder(setter(into))]
    filter_name: String,
    /// New settings. These will be merged to the current filter settings.
    filter_settings: Value,
}

impl Request for SetSourceFilterSettings {
    const REQUEST_TYPE: &'static str = "SetSourceFilterSettings";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "filterName": self.filter_name,
            "filterSettings": self.filter_settings,
        })
    }
}

/// Change the visibility/enabled state of a filter
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetSourceFilterVisibility {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name
    #[builder(setter(into))]
    source_name: String,
    /// Source filter name
    #[builder(setter(into))]
    filter_name: String,
    /// New filter state
    filter_enabled: bool,
}

impl Request for SetSourceFilterVisibility {
    const REQUEST_TYPE: &'static str = "SetSourceFilterVisibility";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// At least embedPictureFormat or saveToFilePath must be specified.
/// Clients can specify width and height parameters to receive scaled pictures. Aspect ratio is preserved if only one of these two parameters is specified.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TakeSourceScreenshot {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Source name. Note that, since scenes are also sources, you can also provide a scene name.
    #[builder(setter(into))]
    source_name: String,
    /// Format of the Data URI encoded picture.
    #[builder(default, setter(strip_option))]
    embed_picture_format: Option<EmbedPictureFormat>,
    /// Full file path (file extension included) where the captured image is to be saved. Can be in a format different from pictureFormat. Can be a relative path.
    #[builder(default, setter(strip_option, into))]
    save_to_file_path: Option<String>,
    /// Screenshot width. Defaults to the source's base width.
    #[builder(default, setter(strip_option))]
    width: Option<i32>,
    /// Screenshot height. Defaults to the source's base height.
    #[builder(default, setter(strip_option))]
    height: Option<i32>,
}

impl Request for TakeSourceScreenshot {
    const REQUEST_TYPE: &'static str = "TakeSourceScreenshot";
    type Response = responses::TakeSourceScreenshot;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "sourceName": self.source_name,
            "embedPictureFormat": self.embed_picture_format,
            "saveToFilePath": self.save_to_file_path,
            "width": self.width,
            "height": self.height,
        })
    }
}

/// Get current streaming and recording status.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStreamingStatus {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStreamingStatus {
    const REQUEST_TYPE: &'static str = "GetStreamingStatus";
    type Response = responses::GetStreamingStatus;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Toggle streaming on or off.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StartStopStreaming {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StartStopStreaming {
    const REQUEST_TYPE: &'static str = "StartStopStreaming";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Start streaming. Will return an error if streaming is already active.
#[derive(TypedBuilder, Debug, PartialEq)]
pub struct StartStreaming {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// If specified ensures the type of stream matches the given type (usually 'rtmp_custom' or 'rtmp_common'). If the currently configured stream type does not match the given stream type, all settings must be specified in the settings object or an error will occur when starting the stream.
    #[builder(default, setter(strip_option, into))]
    stream_type: Option<String>,
    /// Adds the given object parameters as encoded query string parameters to the 'key' of the RTMP stream. Used to pass data to the RTMP service about the streaming. May be any String, Numeric, or Boolean field.
    #[builder(default, setter(strip_option))]
    stream_metadata: Option<Value>,
    /// The publish URL.
    #[builder(default, setter(strip_option, into))]
    stream_server: Option<String>,
    /// The publish key of the stream.
    #[builder(default, setter(strip_option, into))]
    stream_key: Option<String>,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    #[builder(default, setter(strip_option, into))]
    stream_use_auth: Option<String>,
    /// If authentication is enabled, the username for the streaming server. Ignored if use-auth is not set to true.
    #[builder(default, setter(strip_option, into))]
    stream_username: Option<String>,
    /// If authentication is enabled, the password for the streaming server. Ignored if use-auth is not set to true.
    #[builder(default, setter(strip_option, into))]
    stream_password: Option<String>,
}

impl Request for StartStreaming {
    const REQUEST_TYPE: &'static str = "StartStreaming";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// Stop streaming. Will return an error if streaming is not active.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct StopStreaming {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for StopStreaming {
    const REQUEST_TYPE: &'static str = "StopStreaming";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Sets one or more attributes of the current streaming server settings. Any options not passed will remain unchanged. Returns the updated settings in response. If 'type' is different than the current streaming service type, all settings are required. Returns the full settings of the stream (the same as GetStreamSettings).
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetStreamSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// The type of streaming service configuration, usually rtmp_custom or rtmp_common.
    #[builder(default, setter(strip_option, into))]
    stream_type: Option<String>,
    /// The publish URL.
    #[builder(default, setter(strip_option, into))]
    server: Option<String>,
    /// The publish key.
    #[builder(default, setter(strip_option, into))]
    key: Option<String>,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    #[builder(default, setter(strip_option, into))]
    use_auth: Option<String>,
    /// The username for the streaming service.
    #[builder(default, setter(strip_option, into))]
    username: Option<String>,
    /// The password for the streaming service.
    #[builder(default, setter(strip_option, into))]
    password: Option<String>,
    /// Persist the settings to disk.
    save: bool,
}

impl Request for SetStreamSettings {
    const REQUEST_TYPE: &'static str = "SetStreamSettings";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
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

/// Get the current streaming server settings.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStreamSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStreamSettings {
    const REQUEST_TYPE: &'static str = "GetStreamSettings";
    type Response = responses::GetStreamSettings;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Save the current streaming server settings to disk.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SaveStreamSettings {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for SaveStreamSettings {
    const REQUEST_TYPE: &'static str = "SaveStreamSettings";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Send the provided text as embedded CEA-608 caption data. As of OBS Studio 23.1, captions are not yet available on Linux.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SendCaptions {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Captions text
    #[builder(setter(into))]
    text: String,
}

impl Request for SendCaptions {
    const REQUEST_TYPE: &'static str = "SendCaptions";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "text": self.text,
        })
    }
}

/// Indicates if Studio Mode is currently enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetStudioModeStatus {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetStudioModeStatus {
    const REQUEST_TYPE: &'static str = "GetStudioModeStatus";
    type Response = responses::GetStudioModeStatus;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get the name of the currently previewed scene and its list of sources. Will return an error if Studio Mode is not enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetPreviewScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetPreviewScene {
    const REQUEST_TYPE: &'static str = "GetPreviewScene";
    type Response = responses::GetPreviewScene;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Set the active preview scene. Will return an error if Studio Mode is not enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetPreviewScene {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// The name of the scene to preview.
    #[builder(setter(into))]
    scene_name: String,
}

impl Request for SetPreviewScene {
    const REQUEST_TYPE: &'static str = "SetPreviewScene";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "scene-name": self.scene_name,
        })
    }
}

/// Transitions the currently previewed scene to the main output. Will return an error if Studio Mode is not enabled.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct TransitionToProgram {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Name of the transition.
    #[builder(default, setter(strip_option, into))]
    with_transition_name: Option<String>,
    /// Transition duration (in milliseconds).
    #[builder(default, setter(strip_option, into))]
    with_transition_duration: Option<String>,
}

impl Request for TransitionToProgram {
    const REQUEST_TYPE: &'static str = "TransitionToProgram";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "with-transition": {
                "name": self.with_transition_name,
                "duration": self.with_transition_duration,
            }
        })
    }
}

/// Enables Studio Mode.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct EnableStudioMode {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for EnableStudioMode {
    const REQUEST_TYPE: &'static str = "EnableStudioMode";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Disables Studio Mode.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct DisableStudioMode {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for DisableStudioMode {
    const REQUEST_TYPE: &'static str = "DisableStudioMode";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Toggles Studio Mode.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct ToggleStudioMode {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for ToggleStudioMode {
    const REQUEST_TYPE: &'static str = "ToggleStudioMode";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// List of all transitions available in the frontend's dropdown menu.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTransitionList {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetTransitionList {
    const REQUEST_TYPE: &'static str = "GetTransitionList";
    type Response = responses::GetTransitionList;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Get the name of the currently selected transition in the frontend's dropdown menu.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetCurrentTransition {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetCurrentTransition {
    const REQUEST_TYPE: &'static str = "GetCurrentTransition";
    type Response = responses::GetCurrentTransition;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
    }
}

/// Set the active transition.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetCurrentTransition {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// The name of the transition.
    #[builder(setter(into))]
    transition_name: String,
}

impl Request for SetCurrentTransition {
    const REQUEST_TYPE: &'static str = "SetCurrentTransition";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "transition-name": self.transition_name,
        })
    }
}

/// Set the duration of the currently selected transition if supported.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct SetTransitionDuration {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
    /// Desired duration of the transition (in milliseconds).
    duration: i32,
}

impl Request for SetTransitionDuration {
    const REQUEST_TYPE: &'static str = "SetTransitionDuration";
    type Response = responses::Empty;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
            "duration": self.duration,
        })
    }
}

/// Get the duration of the currently selected transition if supported.
#[derive(TypedBuilder, Debug, PartialEq, Eq)]
pub struct GetTransitionDuration {
    #[builder(default = make_message_id(), setter(into))]
    message_id: String,
}

impl Request for GetTransitionDuration {
    const REQUEST_TYPE: &'static str = "GetTransitionDuration";
    type Response = responses::GetTransitionDuration;

    fn get_message_id(&self) -> &str {
        &self.message_id
    }

    fn to_json(&self) -> Value {
        json!({
            "request-type": Self::REQUEST_TYPE,
            "message-id": self.message_id,
        })
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
