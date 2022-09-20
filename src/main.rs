use sdl2::{
    event::Event, image::InitFlag 
};

mod graphics;

use graphics::{
    piece::Piece,
    square::Square,
    ColorType,
};

const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;
const CELL_SIZE: u32 = WIDTH / 8;

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
            if (file + rank) % 2 == 0 {
                squares.push(Square::new(rank, file, CELL_SIZE, ColorType::White, None));
            } else {
                squares.push(Square::new(rank, file, CELL_SIZE, ColorType::Black, None));
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

        canvas.set_draw_color(graphics::CLEAR_COLOR);
        canvas.clear();

        // Draw squares
        for square in &squares {
            square.display(&mut canvas).unwrap();
        }

        // Draw graphics
        canvas.present();
    }
}
