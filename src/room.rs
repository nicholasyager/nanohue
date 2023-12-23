use crate::color::Palette;

pub struct Room {
    pub on: bool,
    pub brightness: u8,
    pub dynamic: bool,

    pub palette: Option<Palette>,
}
