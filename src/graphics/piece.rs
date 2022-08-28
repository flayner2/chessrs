use super::{square::Square, ColorType};
use sdl2::{image::LoadSurface, pixels::Color, rect::Rect, render::BlendMode, surface::Surface};

pub enum PieceType {
    Pawn,
    King,
    Queen,
    Bishop,
    Rook,
    Knight,
}

pub struct Piece<'a> {
    pub width: u32,
    pub height: u32,
    pub texture: &'a str,
    pub color: ColorType,
    pub piece_type: PieceType,
}

impl<'a> Piece<'a> {
    pub fn new(
        width: u32,
        height: u32,
        texture: &'a str,
        color: ColorType,
        piece_type: PieceType,
    ) -> Self {
        Self {
            width,
            height,
            texture,
            color,
            piece_type,
        }
    }

    pub fn draw(
        &self,
        square: &Square,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let mut piece_surface = Surface::from_file("assets/white_pawn.png")?;

        piece_surface.set_color_key(true, Color::RGB(0, 0, 0))?;

        let texture_creator = canvas.texture_creator();
        let mut piece_sprite = texture_creator
            .create_texture_from_surface(piece_surface)
            .expect("Couldn't create texture from surface");

        piece_sprite.set_blend_mode(BlendMode::Add);

        canvas.copy(
            &piece_sprite,
            None,
            Rect::new(
                (square.x + self.width / 2) as i32,
                (square.y + self.height / 2) as i32,
                self.width,
                self.height,
            ),
        )?;

        Ok(())
    }
}
