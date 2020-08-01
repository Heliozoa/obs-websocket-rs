//! Event types. Sent by the server as events occur in OBS.

use crate::common_types::*;
use serde::Deserialize;
use serde_json::Value;

/// Events are broadcast by the server to each connected client when a recognized action occurs within OBS.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    /// time elapsed between now and stream start (only present if OBS Studio is streaming)
    /// Format: HH:MM:SS.mmm
    pub stream_timecode: Option<String>,
    /// time elapsed between now and recording start (only present if OBS Studio is recording)
    /// Format: HH:MM:SS.mmm
    pub rec_timecode: Option<String>,
    /// the type of event
    #[serde(flatten)]
    pub update_type: EventType,
}

/// Contains all the different kinds of events that can occur.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "update-type")]
pub enum EventType {
    // Scenes
    /// Indicates a scene change.
    #[serde(rename_all = "kebab-case")]
    SwitchScenes {
        /// The new scene.
        scene_name: String,
        /// List of scene items in the new scene.
        sources: Vec<SceneItem>,
    },
    /// The scene list has been modified. Scenes have been added, removed, or renamed.
    ScenesChanged,
    /// Triggered when switching to another scene collection or when renaming the current scene collection.
    SceneCollectionChanged,
    /// Triggered when a scene collection is created, added, renamed, or removed.
    SceneCollectionListChanged,

    // Transitions
    /// The active transition has been changed.
    #[serde(rename_all = "kebab-case")]
    SwitchTransition {
        /// The name of the new active transition.
        transition_name: String,
    },
    /// The list of available transitions has been modified. Transitions have been added, removed, or renamed.
    TransitionListChanged,
    /// The active transition duration has been changed.
    #[serde(rename_all = "kebab-case")]
    TransitionDurationChanged {
        /// New transition duration.
        new_duration: i32,
    },
    /// A transition (other than "cut") has begun.
    #[serde(rename_all = "kebab-case")]
    TransitionBegin {
        /// Transition name.
        name: String,
        /// Transition duration (in milliseconds).
        duration: i32,
        /// Source scene of the transition
        from_scene: String,
        /// Destination scene of the transition
        to_scene: String,
    },

    // Profiles
    /// Triggered when switching to another profile or when renaming the current profile.
    ProfileChanged,
    /// Triggered when a profile is created, added, renamed, or removed.
    ProfileListChanged,

    // Streaming
    /// A request to start streaming has been issued.
    StreamStarting, // ignore field preview-only; always false
    /// Streaming started successfully.
    StreamStarted,
    /// A request to stop streaming has been issued.
    StreamStopping, // ignore field preview-only; always false
    /// Streaming stopped successfully.
    StreamStopped,
    /// Emit every 2 seconds.
    #[serde(rename_all = "kebab-case")]
    StreamStatus {
        /// Current streaming state.
        streaming: bool,
        /// Current recording state.
        recording: bool,
        /// Replay Buffer status
        replay_buffer_active: bool,
        /// Amount of data per second (in bytes) transmitted by the stream encoder.
        bytes_per_sec: i32,
        /// Amount of data per second (in kilobits) transmitted by the stream encoder.
        kbits_per_sec: i32,
        /// Percentage of dropped frames.
        strain: f64,
        /// Total time (in seconds) since the stream started.
        total_stream_time: i32,
        /// Total number of frames transmitted since the stream started.
        num_total_frames: i32,
        /// Number of frames dropped by the encoder since the stream started.
        num_dropped_frames: i32,
        /// Current framerate.
        fps: f64,
        /// Number of frames rendered
        render_total_frames: i32,
        /// Number of frames missed due to rendering lag
        render_missed_frames: i32,
        /// Number of frames outputted
        output_total_frames: i32,
        /// Number of frames skipped due to encoding lag
        output_skipped_frames: i32,
        ///Average frame time (in milliseconds)
        average_frame_time: i32,
        /// Current CPU usage (percentage)
        cpu_usage: f64,
        /// Current RAM usage (in megabytes)
        memory_usage: f64,
        /// Free recording disk space (in megabytes)
        free_disk_space: f64,
        // ignore field preview-only: always false
    },

    // Recording
    /// A request to start recording has been issued.
    RecordingStarting,
    /// Recording started successfully.
    RecordingStarted,
    /// A request to stop recording has been issued.
    RecordingStopping,
    /// Recording stopped successfully.
    RecordingStopped,
    /// Current recording paused
    RecordingPaused,
    /// Current recording resumed
    RecordingResumed,

    // Replay Buffer
    /// A request to start the replay buffer has been issued.
    ReplayStarting,
    /// Replay Buffer started successfully
    ReplayStarted,
    /// A request to stop the replay buffer has been issued.
    ReplayStopping,
    /// Replay Buffer stopped successfully
    ReplayStopped,

    // Other
    /// OBS is exiting.
    Exiting,

    // General
    /// Emitted every 2 seconds after enabling it by calling SetHeartbeat.
    #[serde(rename_all = "kebab-case")]
    Heartbeat {
        /// Toggles between every JSON message as an "I am alive" indicator.
        pulse: bool,
        /// Current active profile.
        current_profile: Option<String>,
        /// Current active scene.
        current_scene: Option<String>,
        /// Current streaming state.
        streaming: Option<bool>,
        /// Total time (in seconds) since the stream started.
        total_stream_time: Option<i32>,
        /// Total bytes sent since the stream started.
        total_stream_bytes: Option<i32>,
        /// Total frames streamed since the stream started.
        total_stream_frames: Option<i32>,
        /// Current recording state.
        recording: Option<bool>,
        /// Total time (in seconds) since recording started.
        total_record_time: Option<i32>,
        /// Total bytes recorded since the recording started.
        total_record_bytes: Option<i32>,
        /// Total frames recorded since the recording started.
        total_record_frames: Option<i32>,
        /// OBS Stats
        stats: ObsStats,
    },
    /// A custom broadcast message was received
    #[serde(rename_all = "kebab-case")]
    BroadcastCustomMessage {
        /// Identifier provided by the sender
        realm: String,
        /// User-defined data
        data: Value,
    },

    // Sources
    /// A source has been created. A source can be an input, a scene or a transition.
    #[serde(rename_all = "camelCase")]
    SourceCreated {
        /// Source name
        source_name: String,
        /// Source type.
        source_type: SourceTypesType,
        /// Source kind.
        source_kind: String,
        /// Source settings
        source_settings: Value,
    },
    /// A source has been destroyed/removed. A source can be an input, a scene or a transition.
    #[serde(rename_all = "camelCase")]
    SourceDestroyed {
        /// Source name
        source_name: String,
        /// Source type.
        source_type: SourceTypesType,
        /// Source kind.
        source_kind: String,
    },
    /// The volume of a source has changed.
    #[serde(rename_all = "camelCase")]
    SourceVolumeChanged {
        /// Source name
        source_name: String,
        /// Source volume
        volume: f64,
    },
    /// A source has been muted or unmuted.
    #[serde(rename_all = "camelCase")]
    SourceMuteStateChanged {
        /// Source name
        source_name: String,
        /// Mute status of the source
        muted: bool,
    },
    /// The audio sync offset of a source has changed.
    #[serde(rename_all = "camelCase")]
    SourceAudioSyncOffsetChanged {
        /// Source name
        source_name: String,
        /// Audio sync offset of the source (in nanoseconds)
        sync_offset: i32,
    },
    /// Audio mixer routing changed on a source.
    #[serde(rename_all = "camelCase")]
    SourceAudioMixersChanged {
        /// Source name
        source_name: String,
        /// Routing status of the source for each audio mixer (array of 6 values)
        mixers: Vec<Mixer>,
        /// Raw mixer flags (little-endian, one bit per mixer) as an hexadecimal value
        hex_mixers_value: String,
    },
    /// A source has been renamed.
    #[serde(rename_all = "camelCase")]
    SourceRenamed {
        /// Previous source name
        previous_name: String,
        /// New source name
        new_name: String,
    },
    /// A filter was added to a source.
    #[serde(rename_all = "camelCase")]
    SourceFilterAdded {
        /// Source name
        source_name: String,
        /// Filter name
        filter_name: String,
        /// Filter type
        filter_type: String,
        /// Filter settings
        filter_settings: Value,
    },
    /// A filter was removed from a source.
    #[serde(rename_all = "camelCase")]
    SourceFilterRemoved {
        /// Source name
        source_name: String,
        /// Filter name
        filter_name: String,
        /// Filter type
        filter_type: String,
    },
    /// The visibility/enabled state of a filter changed
    #[serde(rename_all = "camelCase")]
    SourceFilterVisibilityChanged {
        /// Source name
        source_name: String,
        /// Filter name
        filter_name: String,
        /// New filter state
        filter_enabled: bool,
    },
    /// Filters in a source have been reordered.
    #[serde(rename_all = "camelCase")]
    SourceFiltersReordered {
        /// Source name
        source_name: String,
        /// Ordered Filters list
        filters: Vec<Filter>,
    },
    /// Scene items have been reordered.
    #[serde(rename_all = "kebab-case")]
    SourceOrderChanged {
        /// Name of the scene where items have been reordered.
        scene_name: String,
        /// Ordered list of scene items
        scene_items: Vec<EventSceneItem>,
    },
    /// An item has been added to the current scene.
    #[serde(rename_all = "kebab-case")]
    SceneItemAdded {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item added to the scene.
        item_name: String,
        /// Scene item ID
        item_id: i32,
    },
    /// An item has been removed from the current scene.
    #[serde(rename_all = "kebab-case")]
    SceneItemRemoved {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item removed from the scene.
        item_name: String,
        /// Scene item ID
        item_id: i32,
    },
    /// An item's visibility has been toggled.
    #[serde(rename_all = "kebab-case")]
    SceneItemVisibilityChanged {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Scene item ID
        item_id: i32,
        /// New visibility state of the item.
        item_visible: bool,
    },
    /// An item's transform has been changed.
    #[serde(rename_all = "kebab-case")]
    SceneItemTransformChanged {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Scene item ID
        item_id: i32,
        /// Scene item transform properties
        transform: SceneItemTransform,
    },
    /// A scene item is selected.
    #[serde(rename_all = "kebab-case")]
    SceneItemSelected {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Name of the item in the scene.
        item_id: i32,
    },
    /// A scene item is deselected.
    #[serde(rename_all = "kebab-case")]
    SceneItemDeselected {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Name of the item in the scene.
        item_id: i32,
    },

    // Studio Mode
    /// The selected preview scene has changed (only available in Studio Mode).
    #[serde(rename_all = "kebab-case")]
    PreviewSceneChanged {
        /// Name of the scene being previewed.
        scene_name: String,
        /// List of sources composing the scene.
        sources: Vec<SceneItem>,
    },
    /// Studio Mode has been enabled or disabled.
    #[serde(rename_all = "kebab-case")]
    StudioModeSwitched {
        /// The new enabled state of Studio Mode.
        new_state: bool,
    },
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    pub position: Position,
    /// The clockwise rotation of the scene item in degrees around the point of alignment.
    pub rotation: f64,
    pub scale: Scale,
    pub crop: Crop,
    /// If the scene item is visible.
    pub visible: bool,
    /// If the scene item is locked in position.
    pub locked: bool,
    pub bounds: Bounds,
    /// Base width (without scaling) of the source
    pub source_width: i32,
    /// Base source (without scaling) of the source
    pub source_height: i32,
    /// Scene item width (base source width multiplied by the horizontal scaling factor)
    pub width: f64,
    /// Scene item height (base source height multiplied by the vertical scaling factor)
    pub height: f64,
    /// Name of the item's parent (if this item belongs to a group)
    pub parent_group_name: Option<String>,
    /// List of children (if this item is a group)
    pub group_children: Option<Vec<SceneItemTransform>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Mixer {
    /// Mixer number
    pub id: i32,
    /// Routing status
    pub enabled: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Filter {
    /// Filter name
    pub name: String,
    /// Filter type
    // todo: enum?
    #[serde(rename = "type")]
    pub filter_type: String,
}

/// Scene item.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct EventSceneItem {
    /// Item source name
    pub source_name: String,
    /// Scene item unique ID
    pub item_id: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_de() {
        let text = r#"{
            "stream-timecode": "12341234",
            "scene-name": "Scene",
            "sources": [
                {
                    "cx": 1848.0,
                    "cy": 1016.0,
                    "id": 2,
                    "locked": false,
                    "name": "asd",
                    "render": true,
                    "source_cx": 1848,
                    "source_cy": 1016,
                    "type": "xcomposite_input",
                    "volume": 1.0,
                    "x": 0.0,
                    "y": 0.0
                }
            ],
            "update-type": "SwitchScenes"
        }"#;
        let _event: Event = serde_json::from_str(text).unwrap();
    }
}
