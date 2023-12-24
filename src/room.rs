use crate::color::Palette;

#[derive(Debug)]
pub struct Room {
    pub on: bool,
    pub brightness: f32,
    pub dynamic: bool,

    pub palette: Option<Palette>,
    pub color_temperature: Option<u32>,

    pub has_updated: bool,
    pub scene_has_updated: bool,
    pub max_brightness: u8,
}

impl Room {
    pub fn get_brightness(&self) -> u32 {
        self.brightness.clamp(0.0, self.max_brightness as f32) as u32
    }
}
