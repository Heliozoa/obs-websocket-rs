use super::responses::SceneItem;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    update_type: String,
    stream_timecode: Option<String>,
    rec_timecode: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub struct SwitchScenes {
    scene_name: String,
    sources: Vec<SceneItem>,
}
