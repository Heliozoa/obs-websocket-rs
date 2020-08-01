//! Common types used several other modules.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SceneItem {
    pub cy: f32,
    pub cx: f32,
    /// The name of this Scene Item.
    pub name: String,
    /// Scene item ID
    pub id: i32,
    /// Whether or not this Scene Item is set to "visible".
    pub render: bool,
    /// Whether or not this Scene Item is locked and can't be moved around
    pub locked: bool,
    pub source_cx: i32,
    pub source_cy: i32,
    /// Source type.
    #[serde(rename = "type")]
    pub scene_item_type: SceneItemType,
    pub volume: f32,
    pub x: f32,
    pub y: f32,
    /// Name of the item's parent (if this item belongs to a group)
    #[serde(rename = "parentGroupName")]
    pub parent_group_name: Option<String>,
    /// List of children (if this item is a group)
    #[serde(rename = "groupChildren")]
    pub group_children: Option<Vec<SceneItem>>,
}

/// Note: Contains more variants than documented in the reference, more variants may be missing.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SceneItemType {
    Input,
    Filter,
    Transition,
    Scene,
    #[serde(rename = "xcomposite_input")]
    XCompositeInput,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Position {
    /// x position from the left
    pub x: f64,
    /// y position from the top
    pub y: f64,
    /// point on the target that the item is manipulated from
    pub alignment: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Scale {
    /// x-scale factor
    pub x: f64,
    /// y-scale factor
    pub y: f64,
}

/// Rectangular crop for scene items.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Crop {
    /// pixels cropped off the top
    pub top: i32,
    /// pixels cropped off the right
    pub right: i32,
    /// pixels cropped off the bottom
    pub bottom: i32,
    /// pixels cropped off the left
    pub left: i32,
}

/// Bounding box for scene items.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Bounds {
    /// bounds scaling type
    #[serde(rename = "type")]
    pub bounds_type: BoundsType,
    /// new alignment of the bounding box. (0-2, 4-6, 8-10)
    pub alignment: i32,
    /// width of the bounding box
    pub x: f64,
    /// height of the bounding box
    pub y: f64,
}

/// Bounds scaling type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

/// Contains various statistics.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ObsStats {
    /// Current framerate.
    pub fps: f64,
    /// Number of frames rendered
    pub render_total_frames: i32,
    /// Number of frames missed due to rendering lag
    pub render_missed_frames: i32,
    /// Number of frames outputted
    pub output_total_frames: i32,
    /// Number of frames skipped due to encoding lag
    pub output_skipped_frames: i32,
    /// Average frame render time (in milliseconds)
    pub average_frame_time: f64,
    /// Current CPU usage (percentage)
    pub cpu_usage: f64,
    /// Current RAM usage (in megabytes)
    pub memory_usage: f64,
    /// Free recording disk space (in megabytes)
    pub free_disk_space: f64,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SourceTypesType {
    Input,
    Filter,
    Transition,
    Other,
}
