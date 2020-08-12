//! Response types. Received from the server in response to requests.
//! For documentation on which response corresponds to which request, see the requests type.

use crate::common_types::*;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ResponseWrapper {
    #[serde(rename = "message-id")]
    pub message_id: String,
    #[serde(flatten)]
    pub response_data: ResponseData,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[serde(rename_all = "lowercase")]
pub(crate) enum ResponseData {
    // contains the rest of the JSON that can be used to deserialize the appropriate response
    Ok(Value),
    Error { error: String },
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Empty {}

// used to deserialize "a,b,c,d" => ["a", "b", "c", "d"]
fn deserialize_comma_separated_string<'de, D>(d: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct V {}

    impl<'de> de::Visitor<'de> for V {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a comma-separated string")
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
    /// OBSRemote compatible API version. Fixed to 1.1 for retrocompatibility.
    pub version: f64,
    /// obs-websocket plugin version.
    pub obs_websocket_version: String,
    /// OBS Studio program version.
    pub obs_studio_version: String,
    /// List of available request types.
    #[serde(deserialize_with = "deserialize_comma_separated_string")]
    pub available_requests: Vec<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthRequired {
    /// Indicates whether authentication is required.
    pub auth_required: bool,
    pub challenge: Option<String>,
    pub salt: Option<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetFilenameFormatting {
    /// Current filename formatting string.
    pub filename_formatting: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetStats {
    /// OBS stats
    pub stats: ObsStats,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVideoInfo {
    /// Base (canvas) width
    pub base_width: i32,
    /// Base (canvas) height
    pub base_height: i32,
    /// Output width
    pub output_width: i32,
    /// Output height
    pub output_height: i32,
    /// Scaling method used if output size differs from base size
    pub scale_type: ScaleType,
    /// Frames rendered per second
    pub fps: f64,
    /// Video color format
    pub video_format: VideoFormat,
    /// Color space for YUV
    pub color_space: ColorSpace,
    /// Color range (full or partial)
    pub color_range: ColorRange,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ListOutputs {
    /// Outputs list
    pub outputs: Vec<Output>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOutputInfo {
    /// Output info
    pub output_info: Output,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetCurrentProfile {
    /// Name of the currently active profile.
    pub profile_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ListProfiles {
    /// List of available profiles.
    pub profiles: Vec<Profile>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetRecordingFolder {
    /// Path of the recording folder.
    pub rec_folder: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GetCurrentSceneCollection {
    /// Name of the currently active scene collection.
    pub sc_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ListSceneCollections {
    /// Scene collections list
    pub scene_collections: Vec<SceneCollection>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSceneItemProperties {
    /// The name of the source.
    pub name: String,
    pub position: Position,
    /// The clockwise rotation of the item in degrees around the point of alignment.
    pub rotation: f64,
    pub scale: Scale,
    pub crop: Crop,
    /// If the source is visible.
    pub visible: bool,
    /// If the source's transform is locked.
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
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DuplicateSceneItem {
    /// Name of the scene where the new item was created
    pub scene: String,
    /// New item info
    pub item: Item,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCurrentScene {
    /// Name of the currently active scene.
    pub name: String,
    /// Ordered list of the current scene's source items.
    pub sources: Vec<SceneItem>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSceneList {
    /// Name of the currently active scene.
    pub current_scene: String,
    /// Ordered list of the current profile's scenes.
    pub scenes: Vec<Scene>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GetSourcesList {
    /// Array of sources
    pub sources: Vec<Source>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSourceTypesList {
    /// Array of source types
    pub types: Vec<SourceTypes>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetVolume {
    /// Source name.
    pub name: String,
    /// Volume of the source. Between 0.0 and 1.0.
    pub volume: f64,
    /// Indicates whether the source is muted.
    pub muted: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetMute {
    /// Source name.
    pub name: String,
    /// Mute status of the source.
    pub muted: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSyncOffset {
    /// Source name.
    pub name: String,
    /// The audio sync offset (in nanoseconds).
    pub offset: i32,
}

// TODO: deserialize source_settings
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceSettings {
    /// Source name
    pub source_name: String,
    /// Type of the specified source
    pub source_type: SourceKind,
    /// Source settings (varies between source types, may require some probing around).
    pub source_settings: Value,
}

// TODO: deserialize source_settings
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceSettings {
    /// Source name
    pub source_name: String,
    /// Type of the specified source
    pub source_type: SourceKind,
    /// Updated source settings
    pub source_settings: Value,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTextGDIPlusProperties {
    /// Source name.
    pub source: String,
    /// Text Alignment.
    pub align: Align,
    /// Background color.
    #[serde(rename = "bk-color")]
    pub bk_color: i32,
    /// Background opacity (0-100).
    #[serde(rename = "bk-opacity")]
    pub bk_opacity: i32,
    /// Chat log.
    pub chatlog: bool,
    /// Chat log lines.
    pub chatlog_lines: i32,
    /// Text color.
    pub color: i32,
    /// Extents wrap.
    pub extents: bool,
    /// Extents cx.
    pub extents_cx: i32,
    /// Extents cy.
    pub extents_cy: i32,
    /// File path name.
    pub file: String,
    /// Read text from the specified file.
    pub read_from_file: bool,
    /// Holds data for the font.
    pub font: Font,
    /// Gradient enabled.
    pub gradient: bool,
    /// Gradient color.
    pub gradient_color: i32,
    /// Gradient direction.
    pub gradient_dir: f64,
    /// Gradient opacity (0-100).
    pub gradient_opacity: i32,
    /// Outline.
    pub outline: bool,
    /// Outline color.
    pub outline_color: i32,
    /// Outline size.
    pub outline_size: i32,
    /// Outline opacity (0-100).
    pub outline_opacity: i32,
    /// Text content to be displayed.
    pub text: String,
    /// Text vertical alignment.
    pub valign: VerticalAlign,
    /// Vertical text enabled.
    pub vertical: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTextFreetype2Properties {
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
    /// Source name.
    pub source: String,
    /// Indicates that a local file is in use.
    pub is_local_file: bool,
    /// file path.
    pub local_file: String,
    /// Url.
    pub url: String,
    /// CSS to inject.
    pub css: String,
    /// Width.
    pub width: i32,
    /// Height.
    pub height: i32,
    /// Framerate.
    pub fps: i32,
    /// Indicates whether the source should be shutdown when not visible.
    pub shutdown: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSpecialSources {
    /// Name of the first Desktop Audio capture source.
    pub desktop_1: Option<String>,
    /// Name of the second Desktop Audio capture source.
    pub desktop_2: Option<String>,
    /// Name of the first Mic/Aux input source.
    pub mic_1: Option<String>,
    /// Name of the second Mic/Aux input source.
    pub mic_2: Option<String>,
    /// NAme of the third Mic/Aux input source.
    pub mic_3: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetSourceFilters {
    /// List of filters for the specified source
    pub filters: Vec<Filter>,
}

// TODO: deserialize settings
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetSourceFilterInfo {
    /// Filter status (enabled or not)
    pub enabled: bool,
    /// Filter type
    #[serde(rename = "type")]
    pub filter_type: FilterType,
    /// Filter name
    pub name: String,
    /// Filter settings
    pub settings: Value,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeSourceScreenshot {
    /// Source name
    pub source_name: String,
    /// Image Data URI (if embedPictureFormat was specified in the request)
    pub img: String,
    /// Absolute path to the saved image file (if saveToFilePath was specified in the request)
    pub image_file: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStreamingStatus {
    /// Current streaming status.
    pub streaming: bool,
    /// Current recording status.
    pub recording: bool,
    /// Time elapsed since streaming started (only present if currently streaming).
    pub stream_timecode: Option<String>,
    /// Time elapsed since recording started (only present if currently recording).
    pub rec_timecode: Option<String>,
    // ignore field preview-only: always false
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStreamSettings {
    /// The type of streaming service configuration.
    pub stream_type: StreamType,
    /// Stream settings object.
    pub settings: StreamSettings,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetStudioModeStatus {
    /// Indicates if Studio Mode is enabled.
    pub studio_mode: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetPreviewScene {
    /// The name of the active preview scene.
    pub name: String,
    pub sources: Vec<SceneItem>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GetTransitionList {
    /// Name of the currently active transition.
    pub current_transition: String,
    /// List of transitions.
    pub transitions: Vec<Transition>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCurrentTransition {
    /// Name of the selected transition.
    pub name: String,
    /// Transition duration (in milliseconds) if supported by the transition.
    pub duration: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetTransitionDuration {
    /// Duration of the current transition (in milliseconds).
    pub duration: i32,
}

// #### non-response typedefs ####

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorSpace {
    #[serde(rename = "VIDEO_CS_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_CS_601")]
    CS601,
    #[serde(rename = "VIDEO_CS_709")]
    CS709,
    #[serde(rename = "VIDEO_CS_SRGB")]
    SRGB,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorRange {
    #[serde(rename = "VIDEO_RANGE_DEFAULT")]
    Default,
    #[serde(rename = "VIDEO_RANGE_PARTIAL")]
    Partial,
    #[serde(rename = "VIDEO_RANGE_FULL")]
    Full,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    /// Raw flags value
    pub raw_value: i32,
    /// Output uses audio
    pub audio: bool,
    /// Output uses video
    pub video: bool,
    /// Output is encoded
    pub encoded: bool,
    /// Output uses several audio tracks
    pub multi_track: bool,
    /// Output uses a service
    pub service: bool,
}

// TODO: figure out what settings is used for
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    /// Output name
    pub name: String,
    /// Output type/kind
    // todo: enum
    #[serde(rename = "type")]
    pub output_type: String,
    /// Video output width
    pub width: i32,
    /// Video output height
    pub height: i32,
    /// Output flags
    pub flags: Flags,
    /// Output name
    pub settings: Value,
    /// Output status (active or not)
    pub active: bool,
    /// Output reconnection status (reconnecting or not)
    pub reconnecting: bool,
    /// Output congestion
    pub congestion: f64,
    /// Number of frames sent
    pub total_frames: i32,
    /// Number of frames dropped
    pub dropped_frames: i32,
    /// Total bytes sent
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    /// Item name
    pub name: String,
    /// Item ID
    pub id: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Scene {
    /// Name of the currently active scene.
    pub name: String,
    /// Ordered list of the current scene's source items.
    pub sources: Vec<SceneItem>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// Unique source name
    pub name: String,
    /// Non-unique source internal type (a.k.a type id)
    pub type_id: String,
    /// Source type.
    #[serde(rename = "type")]
    pub source_type: SourceType,
}

pub type SourceType = SceneItemType;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Caps {
    /// True if source of this type provide frames asynchronously
    pub is_async: bool,
    /// True if sources of this type provide video
    pub has_video: bool,
    /// True if sources of this type provide audio
    pub has_audio: bool,
    /// True if interaction with this sources of this type is possible
    pub can_interact: bool,
    /// True if sources of this type composite one or more sub-sources
    pub is_composite: bool,
    /// True if sources of this type should not be fully duplicated
    pub do_not_duplicate: bool,
    /// True if sources of this type may cause a feedback loop if it's audio is monitored and shouldn't be
    pub do_not_self_monitor: bool,
}

// TODO: deserialize default_settings (probably not worth it)
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceTypes {
    /// Non-unique internal source type ID
    pub type_id: String,
    /// Display name of the source type
    pub display_name: String,
    /// Type.
    #[serde(rename = "type")]
    pub source_type: SourceTypesType,
    /// Default settings of this source type
    pub default_settings: Value,
    /// Source type capabilities
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
    /// Font face.
    pub face: String,
    /// Font text styling flag.
    pub flags: FontFlags,
    /// Font text size.
    pub size: i32,
    /// Font Style (unknown function).
    pub style: String,
}

// font flags are sent from the server as an integer
// Bold=1, Italic=2, Bold Italic=3, Underline=5, Strikeout=8
#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(from = "i32")]
#[non_exhaustive]
pub enum FontFlags {
    Bold,
    Italic,
    BoldItalic,
    Underline,
    Strikeout,
    Unknown(i32),
}

impl From<i32> for FontFlags {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Bold,
            2 => Self::Italic,
            3 => Self::BoldItalic,
            5 => Self::Underline,
            8 => Self::Strikeout,
            unexpected => Self::Unknown(unexpected),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

// TODO: deserialize settings
#[derive(Deserialize, Debug, PartialEq)]
pub struct Filter {
    /// Filter status (enabled or not)
    pub enabled: bool,
    /// Filter type
    #[serde(rename = "type")]
    pub filter_type: FilterType,
    /// Filter name
    pub name: String,
    /// Filter settings
    pub settings: Value,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum StreamType {
    #[serde(rename = "rtmp_custom")]
    Custom,
    #[serde(rename = "rtmp_common")]
    Common,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct StreamSettings {
    /// The publish URL.
    pub server: String,
    /// The publish key of the stream.
    pub key: String,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    pub use_auth: bool,
    /// The username to use when accessing the streaming server. Only present if use-auth is true.
    pub username: String,
    /// The password to use when accessing the streaming server. Only present if use-auth is true.
    pub password: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Transition {
    /// Name of the transition.
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
        if let ResponseData::Error { .. } = res.response_data {
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
        if let ResponseData::Ok(_) = res.response_data {
            panic!();
        }
    }

    #[test]
    fn convert_successful_response_data() {
        let successful = serde_json::json!(
            {
                "message-id": "id",
                "status": "ok",
                "filename-formatting": "formatting",
            }
        );

        let res: ResponseWrapper = serde_json::from_value(successful).unwrap();
        match res.response_data {
            ResponseData::Ok(value) => {
                let data: GetFilenameFormatting = serde_json::from_value(value).unwrap();
                assert_eq!(data.filename_formatting, "formatting");
            }
            ResponseData::Error { .. } => panic!(),
        }
    }

    #[test]
    fn font_flags() {
        let font = serde_json::json!({
            "face": "f",
            "flags": 8,
            "size": 1,
            "style": "unknown",
        });

        let f: Font = serde_json::from_value(font).unwrap();
        assert_eq!(f.flags, FontFlags::Strikeout);
    }
}
