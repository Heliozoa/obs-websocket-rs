use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SceneItemType {
    Input,
    Filter,
    Transition,
    Scene,
    Unknown,
}

#[derive(Serialize, Deserialize)]
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

impl SceneItem {
    pub fn new() -> SceneItem {
        SceneItem {
            cy: 0,
            cx: 0,
            name: "asd".to_string(),
            id: 0,
            render: false,
            locked: false,
            source_cx: 0,
            source_cy: 0,
            scene_item_type: SceneItemType::Unknown,
            volume: 0,
            x: 0,
            y: 0,
            parent_group_name: None,
            group_children: None,
        }
    }
}
