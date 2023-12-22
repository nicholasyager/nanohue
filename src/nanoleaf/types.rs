use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    value: u32,
    max: u32,
    min: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoolValue {
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionValue {
    pub value: u32,
    pub duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PanelState {
    brightness: Range,
    #[serde(rename = "colorMode")]
    color_mode: String,
    hue: Range,
    sat: Range,
    ct: Range,
    on: BoolValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Panel {
    name: String,
    state: PanelState,
}
