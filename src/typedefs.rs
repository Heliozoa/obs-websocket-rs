use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SceneItemType {
    Input,
    Filter,
    Transition,
    Scene,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneItem {
    cy: i32,
    cx: i32,
    name: String,
    id: i32,
    render: bool,
    locked: bool,
    source_cx: i32,
    source_cy: i32,
    #[serde(rename = "type")]
    scene_item_type: SceneItemType,
    volume: i32,
    x: i32,
    y: i32,
    #[serde(rename = "parentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_group_name: Option<String>,
    #[serde(rename = "groupChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    group_children: Option<Vec<SceneItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<i32>,
}

impl Position {
    pub fn is_none(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.alignment.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum BoundsType {
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
    #[serde(rename = "OBS_BOUNDS_NONE")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_children: Option<Vec<SceneItemTransform>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ObsStats {
    pub fps: f64,
    pub render_total_frames: i32,
    pub render_missed_frames: i32,
    pub output_total_frames: i32,
    pub output_skipped_frames: i32,
    pub average_frame_time: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub free_disk_space: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    raw_value: i32,
    audio: bool,
    video: bool,
    encoded: bool,
    multi_track: bool,
    service: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputSettings {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    name: String,
    #[serde(rename = "type")]
    output_type: String,
    width: i32,
    height: i32,
    flags: Flags,
    settings: OutputSettings,
    active: bool,
    reconnecting: bool,
    congestion: f64,
    total_frames: i32,
    dropped_frames: i32,
    total_bytes: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    name: String,
    sources: Vec<SceneItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Font {
    face: Option<String>,
    flags: Option<i32>,
    size: Option<i32>,
    style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MovementType {
    Up,
    Down,
    Top,
    Bottom,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct StreamSettings {
    server: Option<String>,
    key: Option<String>,
    use_auth: Option<bool>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    #[serde(rename = "type")]
    stream_type: Option<String>,
    metadata: Option<Value>,
    settings: StreamSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithTransition {
    name: String,
    duration: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ScaleType {
    #[serde(rename = "VIDEO_SCALE_BILINEAR")]
    Bilinear,
    #[serde(rename = "VIDEO_SCALE_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_SCALE_FAST_BILINEAR")]
    FastBilinear,
    #[serde(rename = "VIDEO_SCALE_BICUBIC")]
    Bicubic,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum VideoFormat {
    #[serde(rename = "VIDEO_FORMAT_NV12")]
    NV12,
    #[serde(rename = "VIDEO_FORMAT_I420")]
    I420,
    #[serde(rename = "VIDEO_FORMAT_I444")]
    I444,
    #[serde(rename = "VIDEO_FORMAT_RGBA")]
    RGB,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ColorSpace {
    #[serde(rename = "VIDEO_CS_709")]
    CS709,
    #[serde(rename = "VIDEO_CS_601")]
    CS601,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ColorRange {
    #[serde(rename = "VIDEO_RANGE_PARTIAL")]
    Partial,
    #[serde(rename = "VIDEO_RANGE_FULL")]
    Full,
}

#[cfg(test)]
mod test {
    use super::*;
}
