use super::responses::SceneItem;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    update_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_timecode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rec_timecode: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub enum Response {
    SwitchScenes(SwitchScenes),
}

pub struct SwitchScenes {
    scene_name: String,
    sources: Vec<SceneItem>,
}
