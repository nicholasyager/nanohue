use serde::{Deserialize, Serialize};

use crate::color::HSVColor;

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeValue {
    value: u32,
    max: u32,
    min: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Range {
    #[serde(rename = "minValue")]
    pub min: u32,
    #[serde(rename = "maxValue")]
    pub max: u32,
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
    brightness: RangeValue,
    #[serde(rename = "colorMode")]
    color_mode: String,
    hue: RangeValue,
    sat: RangeValue,
    ct: RangeValue,
    on: BoolValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Panel {
    name: String,
    state: PanelState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Effect {
    pub command: String,

    #[serde(rename = "animName")]
    pub animation_name: String,

    #[serde(rename = "animData")]
    pub animation_data: Option<String>,

    #[serde(rename = "colorType")]
    pub color_type: String,

    #[serde(rename = "brightnessRange")]
    pub brightness_range: Range,

    #[serde(rename = "loop")]
    pub loop_animation: bool,

    #[serde(rename = "animType")]
    pub animation_type: String,

    #[serde(rename = "transTime")]
    pub transition_time: Range,
    #[serde(rename = "delayTime")]
    pub delay_time: Range,

    pub palette: Vec<HSVColor>,
}
