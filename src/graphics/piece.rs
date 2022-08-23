use super::{common::ColorType, square::Square};
use sdl2::{image::LoadSurface, pixels::Color, rect::Rect, render::BlendMode, surface::Surface};

pub struct Piece {
    pub size: u32,
    pub texture: String,
    pub color: ColorType,
}

impl Piece {
    pub fn new(size: u32, texture: String, color: ColorType) -> Self {
        Self {
            size,
            texture,
            color,
        }
    }

    pub fn draw(
        &self,
        square: &Square,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let texture = self.load_texture(canvas)?;

        canvas.copy(
            &texture,
            None,
            Rect::new(
                (square.x + self.size / 2) as i32,
                (square.y + self.size / 2) as i32,
                self.size,
                self.size,
            ),
        )?;

        Ok(())
    }

    fn load_texture(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<sdl2::render::Texture, String> {
        let mut piece_surface = Surface::from_file("assets/white_pawn.png")?;

        piece_surface.set_color_key(true, Color::RGB(0, 0, 0))?;

        let texture_creator = canvas.texture_creator();
        let mut piece_sprite = texture_creator
            .create_texture_from_surface(piece_surface)
            .expect("Couldn't create texture from surface");

        piece_sprite.set_blend_mode(BlendMode::Add);

        Ok(piece_sprite)
    }
}
