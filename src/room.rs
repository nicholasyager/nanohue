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
}
