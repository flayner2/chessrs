use sdl2::{
    event::Event, image::InitFlag, image::LoadSurface, pixels::Color, rect::Rect,
    render::BlendMode, surface::Surface,
};

mod graphics;

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
        for file in 0..8 {
            for rank in 0..8 {
                if (file + rank) % 2 == 0 {
                    canvas.set_draw_color(graphics::WHITE_COLOR);
                } else {
                    canvas.set_draw_color(graphics::BLACK_COLOR);
                }
                canvas
                    .fill_rect(Rect::new(
                        (rank * CELL_SIZE) as i32,
                        (file * CELL_SIZE) as i32,
                        CELL_SIZE,
                        CELL_SIZE,
                    ))
                    .unwrap();
            }
        }

        // Draw graphics
        canvas.present();
    }
}
