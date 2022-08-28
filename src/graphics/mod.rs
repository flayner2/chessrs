use sdl2::pixels::Color;

pub mod piece;
pub mod square;

pub enum ColorType {
    Black,
    White,
}

pub const BLACK_COLOR: Color = Color::RGB(133, 77, 39);
pub const WHITE_COLOR: Color = Color::RGB(235, 210, 162);
pub const CLEAR_COLOR: Color = Color::RGB(0, 0, 0);
