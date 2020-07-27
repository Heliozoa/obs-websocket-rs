//! Event types. Sent by the server as they occur in OBS.

pub use crate::common_types::*;

use crate::responses::{ObsStats, SourceTypesType};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    stream_timecode: Option<String>,
    rec_timecode: Option<String>,
    #[serde(flatten)]
    update_type: EventType,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "update-type")]
pub enum EventType {
    // Scenes
    #[serde(rename_all = "kebab-case")]
    SwitchScenes {
        scene_name: String,
        sources: Vec<SceneItem>,
    },
    ScenesChanged,
    SceneCollectionChanged,
    SceneCollectionListChanged,

    // Transitions
    #[serde(rename_all = "kebab-case")]
    SwitchTransition {
        transition_name: String,
    },
    TransitionListChanged,
    #[serde(rename_all = "kebab-case")]
    TransitionDurationChanged {
        new_duration: i32,
    },
    #[serde(rename_all = "kebab-case")]
    TransitionBegin {
        name: String,
        duration: i32, // ms
        from_scene: String,
        to_scene: String,
    },

    // Profiles
    ProfileChanged,
    ProfileListChanged,

    // Streaming
    StreamStarting, // ignore preview-only; always false
    StreamStarted,
    StreamStopping, // ignore preview-only; always false
    StreamStopped,
    #[serde(rename_all = "kebab-case")]
    StreamStatus {
        streaming: bool,
        recording: bool,
        replay_buffer_active: bool,
        bytes_per_sec: i32,
        kbits_per_sec: i32,
        strain: f64,
        total_stream_time: i32, //seconds
        num_total_frames: i32,
        num_dropped_frames: i32,
        fps: f64,
        render_total_frames: i32,
        render_missed_frames: i32,
        output_total_frames: i32,
        output_skipped_frames: i32,
        average_frame_time: i32, // ms
        cpu_usage: f64,          // percentage
        memory_usage: f64,       // megabytes
        free_disk_space: f64,    // megabytes
    },

    // Recording
    RecordingStarting,
    RecordingStarted,
    RecordingStopping,
    RecordingStopped,
    RecordingPaused,
    RecordingResumed,

    // Replay Buffer
    ReplayStarting,
    ReplayStarted,
    ReplayStopping,
    ReplayStopped,

    // Other
    Exiting,

    // General
    #[serde(rename_all = "kebab-case")]
    Heartbeat {
        pulse: bool,
        current_profile: Option<String>,
        current_scene: Option<String>,
        streaming: Option<bool>,
        total_stream_time: Option<i32>, // seconds
        total_stream_bytes: Option<i32>,
        total_stream_frames: Option<i32>,
        recording: Option<bool>,
        total_record_time: Option<i32>, // seconds
        total_record_bytes: Option<i32>,
        total_record_frames: Option<i32>,
        stats: ObsStats,
    },
    #[serde(rename_all = "kebab-case")]
    BroadcastCustomMessage {
        realm: String,
        data: HashMap<String, Value>,
    },

    // Sources
    #[serde(rename_all = "camelCase")]
    SourceCreated {
        source_name: String,
        source_type: SourceTypesType,
        source_kind: String,
        source_settings: HashMap<String, Value>,
    },
    #[serde(rename_all = "camelCase")]
    SourceDestroyed {
        source_name: String,
        source_type: SourceTypesType,
        source_kind: String,
    },
    #[serde(rename_all = "camelCase")]
    SourceVolumeChanged {
        source_name: String,
        volume: f64,
    },
    #[serde(rename_all = "camelCase")]
    SourceMuteStateChanged {
        source_name: String,
        muted: bool,
    },
    #[serde(rename_all = "camelCase")]
    SourceAudioSyncOffsetChanged {
        source_name: String,
        sync_offset: i32,
    },
    #[serde(rename_all = "camelCase")]
    SourceAudioMixersChanged {
        source_name: String,
        mixers: Vec<Mixer>,
        hex_mixers_value: String,
    },
    #[serde(rename_all = "camelCase")]
    SourceRenamed {
        previous_name: String,
        new_name: String,
    },
    #[serde(rename_all = "camelCase")]
    SourceFilterAdded {
        source_name: String,
        filter_name: String,
        filter_type: String,
        filter_settings: HashMap<String, Value>,
    },
    #[serde(rename_all = "camelCase")]
    SourceFilterRemoved {
        source_name: String,
        filter_name: String,
        filter_type: String,
    },
    #[serde(rename_all = "camelCase")]
    SourceFilterVisibilityChanged {
        source_name: String,
        filter_name: String,
        filter_enabled: bool,
    },
    #[serde(rename_all = "camelCase")]
    SourceFiltersReordered {
        source_name: String,
        filters: Vec<Filter>,
    },
    #[serde(rename_all = "kebab-case")]
    SourceOrderChanged {
        scene_name: String,
        scene_items: Vec<EventSceneItem>,
    },
    #[serde(rename_all = "kebab-case")]
    SceneItemAdded {
        scene_name: String,
        item_name: String,
        item_id: i32,
    },
    #[serde(rename_all = "kebab-case")]
    SceneItemRemoved {
        scene_name: String,
        item_name: String,
        item_id: i32,
    },
    #[serde(rename_all = "kebab-case")]
    SceneItemVisibilityChanged {
        scene_name: String,
        item_name: String,
        item_id: i32,
        item_visible: bool,
    },
    #[serde(rename_all = "kebab-case")]
    SceneItemTransformChanged {
        scene_name: String,
        item_name: String,
        item_id: i32,
        transform: SceneItemTransform,
    },
    #[serde(rename_all = "kebab-case")]
    SceneItemSelected {
        scene_name: String,
        item_name: String,
        item_id: i32,
    },
    #[serde(rename_all = "kebab-case")]
    SceneItemDeselected {
        scene_name: String,
        item_name: String,
        item_id: i32,
    },

    // Studio Mode
    #[serde(rename_all = "kebab-case")]
    PreviewSceneChanged {
        scene_name: String,
        sources: Vec<SceneItem>,
    },
    #[serde(rename_all = "kebab-case")]
    StudioModeSwitched {
        new_state: bool,
    },
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    position: Position,
    rotation: f64,
    scale: Scale,
    crop: Crop,
    visible: bool,
    locked: bool,
    bounds: Bounds,
    source_width: i32,
    source_height: i32,
    width: f64,
    height: f64,
    parent_group_name: Option<String>,
    group_children: Option<Vec<SceneItemTransform>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Mixer {
    id: i32,
    enabled: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Filter {
    name: String,
    #[serde(rename = "type")]
    filter_type: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct EventSceneItem {
    source_name: String,
    item_id: String,
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
        let event: Event = serde_json::from_str(text).unwrap();
    }
}
