use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

// used to separate successful and error responses
#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "status")]
pub(crate) enum ResponseWrapper {
    Ok(Response),
    Error(ResponseError),
}

impl ResponseWrapper {
    pub fn message_id(&self) -> &str {
        match self {
            ResponseWrapper::Ok(r) => &r.message_id,
            ResponseWrapper::Error(r) => &r.message_id,
        }
    }
}

// not really used for anything
// would be nice to convert directly to the response type
// but may not be worth the trouble
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub(crate) struct Response {
    #[serde(rename = "message-id")]
    pub message_id: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub(crate) struct ResponseError {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub error: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Empty {
    #[serde(rename = "message-id")]
    pub message_id: String,
}

// used to deserialize "a,b,c,d" => ["a", "b", "c", "d"]
fn deserialize_comma_separated_string<'de, D>(d: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct V {}

    impl<'de> de::Visitor<'de> for V {
        type Value = Vec<String>;

        fn expecting(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            unreachable!()
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(s.split(',').map(|s| s.to_owned()).collect::<Vec<_>>())
        }
    }

    d.deserialize_str(V {})
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetVersion {
    pub message_id: String,
    pub version: f64,
    pub obs_websocket_version: String,
    pub obs_studio_version: String,
    #[serde(deserialize_with = "deserialize_comma_separated_string")]
    pub available_requests: Vec<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthRequired {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub auth_required: bool,
    pub challenge: Option<String>,
    pub salt: Option<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetFilenameFormatting {
    pub message_id: String,
    pub filename_formatting: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetStats {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub stats: ObsStats,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVideoInfo {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub base_width: i32,
    pub base_height: i32,
    pub output_width: i32,
    pub output_height: i32,
    pub scale_type: ScaleType,
    pub fps: f64,
    pub video_format: VideoFormat,
    pub color_space: ColorSpace,
    pub color_range: ColorRange,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ListOutputs {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub outputs: Vec<Output>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOutputInfo {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub output_info: Output,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetCurrentProfile {
    pub message_id: String,
    pub profile_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ListProfiles {
    pub message_id: String,
    pub profiles: Vec<Profile>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetRecordingFolder {
    pub message_id: String,
    pub rec_folder: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetCurrentSceneCollection {
    pub message_id: String,
    pub sc_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ListSceneCollections {
    pub message_id: String,
    pub scene_collections: Vec<SceneCollection>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSceneItemProperties {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub position: Position,
    pub rotation: f64,
    pub scale: Scale,
    pub crop: Crop,
    pub visible: bool,
    pub locked: bool,
    pub bounds: Bounds,
    pub source_width: i32,
    pub source_height: i32,
    pub width: f64,
    pub height: f64,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DuplicateSceneItem {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub scene: String,
    pub item: Item,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GetCurrentScene {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub sources: Vec<SceneItem>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSceneList {
    pub message_id: String,
    pub current_scene: String,
    pub scenes: Vec<Scene>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GetSourcesList {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub sources: Vec<Source>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSourceTypesList {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub types: Vec<SourceTypes>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetVolume {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub volume: f64,
    pub muted: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetMute {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub muted: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSyncOffset {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub offset: i32,
}

// TODO: deserialize source_settings
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceSettings {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub source_name: String,
    pub source_type: String,
    pub source_settings: HashMap<String, Value>,
}

// TODO: deserialize source_settings
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceSettings {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub source_name: String,
    pub source_type: String,
    pub source_settings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTextGDIPlusProperties {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub source: String,
    pub align: Align,
    #[serde(rename = "bk-color")]
    pub bk_color: i32,
    #[serde(rename = "bk-opacity")]
    pub bk_opacity: i32,
    pub chatlog: bool,
    pub chatlog_lines: i32,
    pub color: i32,
    pub extents: bool,
    pub extents_cx: i32,
    pub extents_cy: i32,
    pub file: String,
    pub read_from_file: bool,
    pub font: Font,
    pub gradient: bool,
    pub gradient_color: i32,
    pub gradient_dir: f64,
    pub gradient_opacity: i32,
    pub outline: bool,
    pub outline_color: i32,
    pub outline_size: i32,
    pub outline_opacity: i32,
    pub text: String,
    pub valign: VerticalAlign,
    pub vertical: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTextFreetype2Properties {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub source: String,
    pub color1: i32,
    pub color2: i32,
    pub custom_width: i32,
    pub drop_shadow: bool,
    pub font: Font,
    pub from_file: bool,
    pub log_mode: bool,
    pub outline: bool,
    pub text: String,
    pub text_file: String,
    pub word_wrap: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetBrowserSourceProperties {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub source: String,
    pub is_local_file: bool,
    pub local_file: String,
    pub url: String,
    pub css: String,
    pub width: i32,
    pub height: i32,
    pub fps: i32,
    pub shutdown: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSpecialSources {
    pub message_id: String,
    pub desktop_1: Option<String>,
    pub desktop_2: Option<String>,
    pub mic_1: Option<String>,
    pub mic_2: Option<String>,
    pub mic_3: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSourceFilters {
    pub message_id: String,
    pub filters: Vec<Filter>,
}

// TODO: deserialize settings
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSourceFilterInfo {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub enabled: bool,
    #[serde(rename = "type")]
    pub filter_type: String,
    pub name: String,
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeSourceScreenshot {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub source_name: String,
    pub img: String,
    pub image_file: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStreamingStatus {
    pub message_id: String,
    pub streaming: bool,
    pub recording: bool,
    pub stream_timecode: Option<String>,
    pub rec_timecode: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStreamSettings {
    pub message_id: String,
    pub stream_type: StreamType,
    pub settings: StreamSettings,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStudioModeStatus {
    pub message_id: String,
    pub studio_mode: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetPreviewScene {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub sources: Vec<SceneItem>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetTransitionList {
    pub message_id: String,
    pub current_transition: String,
    pub transitions: Vec<Transition>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCurrentTransition {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub name: String,
    pub duration: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTransitionDuration {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub duration: i32,
}

// #### non-response typedefs ####

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum ColorRange {
    #[serde(rename = "VIDEO_RANGE_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_RANGE_PARTIAL")]
    Partial,
    #[serde(rename = "VIDEO_RANGE_FULL")]
    Full,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    pub raw_value: i32,
    pub audio: bool,
    pub video: bool,
    pub encoded: bool,
    pub multi_track: bool,
    pub service: bool,
}

// TODO: figure out what settings is used for
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub name: String,
    #[serde(rename = "type")]
    pub output_type: String,
    pub width: i32,
    pub height: i32,
    pub flags: Flags,
    pub settings: HashMap<String, Value>,
    pub active: bool,
    pub reconnecting: bool,
    pub congestion: f64,
    pub total_frames: i32,
    pub dropped_frames: i32,
    pub total_bytes: i32,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub profile_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct SceneCollection {
    pub sc_name: String,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub alignment: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Crop {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq)]
pub struct Bounds {
    #[serde(rename = "type")]
    pub bounds_type: BoundsType,
    pub alignment: i32,
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub id: i32,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SceneItemType {
    Input,
    Filter,
    Transition,
    Scene,
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SceneItem {
    pub cy: i32,
    pub cx: i32,
    pub name: String,
    pub id: i32,
    pub render: bool,
    pub locked: bool,
    pub source_cx: i32,
    pub source_cy: i32,
    #[serde(rename = "type")]
    pub scene_item_type: SceneItemType,
    pub volume: i32,
    pub x: i32,
    pub y: i32,
    pub parent_group_name: Option<String>,
    pub group_children: Option<Vec<SceneItem>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Scene {
    pub name: String,
    pub sources: Vec<SceneItem>,
}

pub type SourceType = SceneItemType;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub type_id: String,
    #[serde(rename = "type")]
    pub source_type: SourceType,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SourceTypesType {
    Input,
    Filter,
    Transition,
    Other,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Caps {
    pub is_async: bool,
    pub has_video: bool,
    pub has_audio: bool,
    pub can_interact: bool,
    pub is_composite: bool,
    pub do_not_duplicate: bool,
    pub do_not_self_monitor: bool,
}

// TODO: deserialize default_settings (probably not worth it)
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceTypes {
    pub type_id: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub source_type: SourceTypesType,
    pub default_settings: HashMap<String, Value>,
    pub caps: Caps,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Font {
    pub face: String,
    pub flags: i32,
    pub size: i32,
    pub style: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

// TODO: deserialize settings
#[derive(Deserialize, Debug, PartialEq)]
pub struct Filter {
    pub enabled: bool,
    #[serde(rename = "type")]
    pub filter_type: String,
    pub name: String,
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    #[serde(rename = "rtmp_custom")]
    Custom,
    #[serde(rename = "rtmp_common")]
    Common,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct StreamSettings {
    pub server: String,
    pub key: String,
    pub use_auth: bool,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Transition {
    pub name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_successful_response() {
        let successful = serde_json::json!(
            {
                "message-id": "id",
                "status": "ok",
                "other": "abcd",
            }
        );

        let res: ResponseWrapper = serde_json::from_value(successful).unwrap();
        if let ResponseWrapper::Error(_) = res {
            panic!();
        }
    }

    #[test]
    fn convert_error_response() {
        let successful = serde_json::json!(
            {
                "message-id": "id",
                "status": "error",
                "error": "errormsg",
            }
        );

        let res: ResponseWrapper = serde_json::from_value(successful).unwrap();
        if let ResponseWrapper::Ok(_) = res {
            panic!();
        }
    }
}
