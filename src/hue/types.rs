use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::color::{ColorCoordinate, ColorGamut};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupActions {
    pub on: bool,

    #[serde(rename = "bri")]
    pub brightness: u8,
    hue: u32,

    #[serde(rename = "sat")]
    saturation: u8,

    #[serde(rename = "ct")]
    color_temperature: u16,

    effect: String,
    xy: ColorCoordinate,

    alert: String,

    #[serde(rename = "colormode")]
    color_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightState {
    on: bool,

    #[serde(rename = "bri")]
    pub brightness: u8,
    hue: Option<u32>,

    #[serde(rename = "sat")]
    saturation: Option<u8>,

    #[serde(rename = "ct")]
    color_temperature: Option<u16>,

    effect: Option<String>,
    pub xy: Option<ColorCoordinate>,

    alert: String,

    #[serde(rename = "colormode")]
    color_mode: Option<String>,

    reachable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightControl {
    #[serde(rename = "colorgamut")]
    pub color_gamut: Option<ColorGamut>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LightCapabilities {
    pub control: LightControl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub name: String,
    pub state: LightState,
    pub capabilities: LightCapabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub lights: Vec<String>,

    #[serde(rename = "type")]
    pub group_type: String,
    pub action: GroupActions,
}

#[derive(Debug, Serialize, Deserialize)]
enum EventData {
    Variant1(LightState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "creationtime")]
    pub creation_time: String,

    pub data: Vec<serde_json::Value>,
}
