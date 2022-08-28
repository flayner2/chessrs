use super::piece::Piece;
use super::ColorType;
use super::{BLACK_COLOR, WHITE_COLOR};
use sdl2::rect::Rect;

pub struct Square<'a> {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub color: ColorType,
    pub piece: Option<&'a Piece<'a>>,
}

impl<'a> Square<'a> {
    pub fn new(x: u32, y: u32, size: u32, color: ColorType, piece: Option<&'a Piece>) -> Self {
        Self {
            x,
            y,
            size,
            color,
            piece,
        }
    }

    pub fn display(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        match self.color {
            ColorType::White => canvas.set_draw_color(WHITE_COLOR),
            ColorType::Black => canvas.set_draw_color(BLACK_COLOR),
        };

        canvas.fill_rect(Rect::new(
            (self.x * self.size) as i32,
            (self.y * self.size) as i32,
            self.size,
            self.size,
        ))?;

        if let Some(piece) = self.piece {
            piece.draw(self, canvas)?;
        };

        Ok(())
    }
}
