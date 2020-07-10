use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct SceneItem {
    pub cy: f32,
    pub cx: f32,
    pub name: String,
    pub id: i32,
    pub render: bool,
    pub locked: bool,
    pub source_cx: i32,
    pub source_cy: i32,
    #[serde(rename = "type")]
    pub scene_item_type: SceneItemType,
    pub volume: f32,
    pub x: f32,
    pub y: f32,
    #[serde(rename = "parentGroupName")]
    pub parent_group_name: Option<String>,
    #[serde(rename = "groupChildren")]
    pub group_children: Option<Vec<SceneItem>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq)]
pub struct Bounds {
    #[serde(rename = "type")]
    pub bounds_type: BoundsType,
    pub alignment: i32,
    pub x: f64,
    pub y: f64,
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
