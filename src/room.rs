use crate::color::Palette;

#[derive(Debug)]
pub struct Room {
    pub on: bool,
    pub brightness: u8,
    pub dynamic: bool,

    pub palette: Option<Palette>,
}
