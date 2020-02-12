use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Response {
    pub message_id: String,
    pub status: Status,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Empty {
    #[serde(flatten)]
    pub response_data: Response,
}

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
    pub version: f64,
    pub obs_websocket_version: String,
    pub obs_studio_version: String,
    #[serde(deserialize_with = "deserialize_comma_separated_string")]
    pub available_requests: Vec<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthRequired {
    pub auth_required: bool,
    pub challenge: Option<String>,
    pub salt: Option<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetFilenameFormatting {
    pub filename_formatting: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetStats {
    pub stats: ObsStats,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVideoInfo {
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
    pub outputs: Vec<Output>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOutputInfo {
    pub output_info: Output,
}

pub type GetCurrentProfile = Profile;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ListProfiles {
    pub profiles: Vec<Profile>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetRecordingFolder {
    pub rec_folder: String,
}

pub type GetCurrentSceneCollection = SceneCollection;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ListSceneCollections {
    pub scene_collections: Vec<SceneCollection>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSceneItemProperties {
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
    pub scene: String,
    pub item: Item,
}

pub type GetCurrentScene = Scene;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSceneList {
    pub current_scene: String,
    pub scenes: Vec<Scene>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GetSourcesList {
    sources: Vec<Source>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSourceTypesList {
    sources: Vec<SourceTypes>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetVolume {
    name: String,
    volume: f64,
    muted: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetMute {
    name: String,
    muted: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSyncOffset {
    name: String,
    offset: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceSettings {
    source_name: String,
    source_type: String,
    source_settings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceSettings {
    source_name: String,
    source_type: String,
    source_settings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTextGDIPlusProperties {
    source: String,
    align: Align,
    #[serde(rename = "bk-color")]
    bk_color: i32,
    #[serde(rename = "bk-opacity")]
    bk_opacity: i32,
    chatlog: bool,
    chatlog_lines: i32,
    color: i32,
    extents: bool,
    extents_cx: i32,
    extents_cy: i32,
    file: String,
    read_from_file: bool,
    font: Font,
    gradient: bool,
    gradient_color: i32,
    gradient_dir: f64,
    gradient_opacity: i32,
    outline: bool,
    outline_color: i32,
    outline_size: i32,
    outline_opacity: i32,
    text: String,
    valign: VerticalAlign,
    vertical: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTextFreetype2Properties {
    source: String,
    color1: i32,
    color2: i32,
    custom_width: i32,
    drop_shadow: bool,
    font: Font,
    from_file: bool,
    log_mode: bool,
    outline: bool,
    text: String,
    text_file: String,
    word_wrap: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetBrowserSourceProperties {
    source: String,
    is_local_file: bool,
    local_file: String,
    url: String,
    css: String,
    width: i32,
    height: i32,
    fps: i32,
    shutdown: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSpecialSources {
    desktop_1: Option<String>,
    desktop_2: Option<String>,
    mic_1: Option<String>,
    mic_2: Option<String>,
    mic_3: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSourceFilters {
    filters: Vec<Filter>,
}

pub type GetSourceFilterInfo = Filter;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeSourceScreenshot {
    source_name: String,
    img: String,
    image_file: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStreamingStatus {
    streaming: bool,
    recording: bool,
    stream_timecode: Option<String>,
    rec_timecode: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStreamSettings {
    stream_type: StreamType,
    settings: StreamSettings,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStudioModeStatus {
    studio_mode: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetPreviewScene {
    name: String,
    sources: Vec<SceneItem>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetTransitionList {
    current_transition: String,
    transitions: Vec<Transition>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCurrentTransition {
    name: String,
    duration: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTransitionDuration {
    duration: i32,
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

// TODO: settings
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
    name: String,
    id: i32,
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
    parent_group_name: Option<String>,
    group_children: Option<Vec<SceneItem>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Scene {
    name: String,
    sources: Vec<SceneItem>,
}

pub type SourceType = SceneItemType;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    name: String,
    type_id: String,
    #[serde(rename = "type")]
    source_type: SourceType,
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
    is_async: bool,
    has_video: bool,
    has_audio: bool,
    can_interact: bool,
    is_composite: bool,
    do_not_duplicate: bool,
    do_not_self_monitor: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceTypes {
    type_id: String,
    display_name: String,
    #[serde(rename = "type")]
    source_type: SourceTypesType,
    default_settings: HashMap<String, Value>,
    caps: Caps,
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
    face: String,
    flags: i32,
    size: i32,
    style: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Filter {
    enabled: bool,
    #[serde(rename = "type")]
    filter_type: String,
    name: String,
    settings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    #[serde(rename = "rtmp_custom")]
    Custom,
    #[serde(rename = "rtmp_common")]
    Common,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct StreamSettings {
    server: String,
    key: String,
    use_auth: bool,
    username: String,
    password: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Transition {
    name: String,
}
