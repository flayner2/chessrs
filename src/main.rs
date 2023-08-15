use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    surface::Surface,
    video::{Window, WindowContext},
};

enum ChessColor {
    White,
    Black,
}

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct Piece {
    square_index: u8,
    width: u32,
    height: u32,
    piece_type: PieceType,
    color: ChessColor,
}

struct Square {
    rank: u32,
    file: u32,
    size: u32,
    color: ChessColor,
    piece: Option<Piece>,
}

impl Piece {
    fn display(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) -> Result<(), String> {
        let mut piece_surface = Surface::from_file("assets/white_pawn.png")?;

        piece_surface.set_color_key(true, Color::RGB(0, 0, 0))?;

        let texture_creator = canvas.texture_creator();
        let mut piece_sprite = texture_creator
            .create_texture_from_surface(piece_surface)
            .expect("Couldn't create texture from piece asset");

        piece_sprite.set_blend_mode(sdl2::render::BlendMode::Add);

        canvas.copy(
            &piece_sprite,
            None,
            Rect::new(x, y, self.width, self.height),
        )?;

        Ok(())
    }
}

impl Square {
    fn new(rank: u32, file: u32, size: u32, color: ChessColor, piece: Option<Piece>) -> Self {
        Square {
            rank,
            file,
            size,
            color,
            piece,
        }
    }

    fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        match self.color {
            ChessColor::White => canvas.set_draw_color(WHITE_COLOR),
            ChessColor::Black => canvas.set_draw_color(BLACK_COLOR),
        };

        let square_x = (self.rank * self.size) as i32;
        let square_y = (self.file * self.size) as i32;

        let new_square = Rect::new(square_x, square_y, self.size, self.size);

        canvas.fill_rect(new_square)?;

        // Maybe this will be moved somewhere else
        if let Some(piece) = self.piece.as_ref() {
            piece.display(canvas, square_x, square_y)?;
        }

        Ok(())
    }
}

const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;
const CELL_SIZE: u32 = WIDTH / 8;
const CLEAR_COLOR: Color = Color::RGB(0, 0, 0);
const WHITE_COLOR: Color = Color::RGB(235, 210, 162);
const BLACK_COLOR: Color = Color::RGB(133, 77, 39);

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2 context");

    let mut canvas = {
        let video = sdl_context
            .video()
            .expect("Failed to initialize video subsystem");

        let window = video
            .window("Chessrs", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to create window");

        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Failed to get render canvas")
    };

    let _image_context = sdl2::image::init(InitFlag::PNG).expect("Failed initializing SDL2_image");

    let mut events = sdl_context.event_pump().expect("Failed to get event loop");

    let mut squares: Vec<Square> = Vec::new();

    for file in 0..8 {
        for rank in 0..8 {
            // Just testing inserting a pawn
            if (file + rank) == 0 {
                squares.push(Square::new(
                    rank,
                    file,
                    CELL_SIZE,
                    ChessColor::White,
                    Some(Piece {
                        square_index: 0,
                        width: 80,
                        height: 80,
                        piece_type: PieceType::Pawn,
                        color: ChessColor::White,
                    }),
                ))
            } else if (file + rank) % 2 == 0 {
                squares.push(Square::new(rank, file, CELL_SIZE, ChessColor::White, None));
            } else {
                squares.push(Square::new(rank, file, CELL_SIZE, ChessColor::Black, None));
            }
        }
    }

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                _ => {}
            }
        }

        canvas.set_draw_color(CLEAR_COLOR);
        canvas.clear();

        // Draw squares
        for square in &squares {
            square.display(&mut canvas).unwrap();
        }

        // Draw graphics
        canvas.present();
    }
}
