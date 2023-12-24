use serde::{Deserialize, Serialize};

use crate::color::{ColorCoordinate, ColorGamut2};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventMessage {
    pub id: String,

    pub on: Option<OnStatus>,
    pub dimming: Option<Dimming>,
    pub color: Option<Color>,
    pub status: Option<SceneStatus>,

    pub owner: Option<Resource>,

    #[serde(rename = "type")]
    pub message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "creationtime")]
    pub creation_time: String,

    pub data: Vec<EventMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    #[serde(rename = "rid")]
    pub id: String,
    #[serde(rename = "rtype")]
    pub resource_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomMetadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub children: Vec<Resource>,
    pub services: Vec<Resource>,
    pub metadata: RoomMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HueResponse<T>
where
    T: Serialize,
{
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneStatus {
    pub active: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnStatus {
    pub on: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimming {
    pub brightness: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupedLight {
    pub id: String,
    pub on: OnStatus,
    pub dimming: Dimming,
}

// Lighting

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Color {
    pub xy: ColorCoordinate,
    pub gamut: Option<ColorGamut2>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dynamics {
    pub status: String,
    pub status_values: Vec<String>,
    pub speed: f32,
    pub speed_valid: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Light {
    pub id: String,
    pub owner: Resource,
    pub on: OnStatus,
    pub dimming: Dimming,
    pub color: Option<Color>,
    pub dynamics: Dynamics,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScenePaletteColor {
    pub color: Color,
    pub dimming: Dimming,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScenePalette {
    pub color: Vec<ScenePaletteColor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorTemperature {
    pub mirek: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub on: OnStatus,
    pub dimming: Dimming,
    pub color: Option<Color>,
    pub color_temperature: Option<ColorTemperature>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneAction {
    pub target: Resource,
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    pub id: String,
    pub group: Resource,
    pub palette: ScenePalette,
    pub status: SceneStatus,
    pub actions: Vec<SceneAction>,
}
