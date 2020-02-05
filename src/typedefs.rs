use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<i32>,
}

impl Position {
    pub fn is_none(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.alignment.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    pub raw_value: i32,
    pub audio: bool,
    pub video: bool,
    pub encoded: bool,
    pub multi_track: bool,
    pub service: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub name: String,
    #[serde(rename = "type")]
    pub output_type: String,
    pub width: i32,
    pub height: i32,
    pub flags: Flags,
    pub settings: HashMap<String, String>,
    pub active: bool,
    pub reconnecting: bool,
    pub congestion: f64,
    pub total_frames: i32,
    pub dropped_frames: i32,
    pub total_bytes: i32,
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
    #[serde(rename = "VIDEO_SCALE_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_SCALE_POINT")]
    Point,
    #[serde(rename = "VIDEO_SCALE_FAST_BILINEAR")]
    FastBilinear,
    #[serde(rename = "VIDEO_SCALE_BILINEAR")]
    Bilinear,
    #[serde(rename = "VIDEO_SCALE_BICUBIC")]
    Bicubic,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum VideoFormat {
    #[serde(rename = "VIDEO_FORMAT_NONE")]
    None,
    #[serde(rename = "VIDEO_FORMAT_I420")]
    I420,
    #[serde(rename = "VIDEO_FORMAT_NV12")]
    NV12,
    #[serde(rename = "VIDEO_FORMAT_YVYU")]
    YVYU,
    #[serde(rename = "VIDEO_FORMAT_YUY2")]
    YUY2,
    #[serde(rename = "VIDEO_FORMAT_UYVY")]
    UYVY,
    #[serde(rename = "VIDEO_FORMAT_RGBA")]
    RGBA,
    #[serde(rename = "VIDEO_FORMAT_BGRA")]
    BGRA,
    #[serde(rename = "VIDEO_FORMAT_BGRX")]
    BGRX,
    #[serde(rename = "VIDEO_FORMAT_Y800")]
    Y800,
    #[serde(rename = "VIDEO_FORMAT_I444")]
    I444,
    #[serde(rename = "VIDEO_FORMAT_BGR3")]
    BGR3,
    #[serde(rename = "VIDEO_FORMAT_I422")]
    I422,
    #[serde(rename = "VIDEO_FORMAT_I40A")]
    I40A,
    #[serde(rename = "VIDEO_FORMAT_I42A")]
    I42A,
    #[serde(rename = "VIDEO_FORMAT_YUVA")]
    YUVA,
    #[serde(rename = "VIDEO_FORMAT_AYUV")]
    AYUV,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ColorSpace {
    #[serde(rename = "VIDEO_CS_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_CS_601")]
    CS601,
    #[serde(rename = "VIDEO_CS_709")]
    CS709,
    #[serde(rename = "VIDEO_CS_SRGB")]
    SRGB,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ColorRange {
    #[serde(rename = "VIDEO_RANGE_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_RANGE_PARTIAL")]
    Partial,
    #[serde(rename = "VIDEO_RANGE_FULL")]
    Full,
}

#[cfg(test)]
mod test {
    use super::*;
}
