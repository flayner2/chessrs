use sdl2::{event::Event, pixels::Color, rect::Rect};

const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;
const CELL_SIZE: u32 = WIDTH / 8;
const BLACK_COLOR: Color = Color::RGB(133, 77, 39);
const WHITE_COLOR: Color = Color::RGB(235, 210, 162);
const CLEAR_COLOR: Color = Color::RGB(0, 0, 0);

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

    let mut events = sdl_context.event_pump().expect("Failed to get event loop");

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
        for file in 0..8 {
            for rank in 0..8 {
                if (file + rank) % 2 == 0 {
                    canvas.set_draw_color(BLACK_COLOR);
                } else {
                    canvas.set_draw_color(WHITE_COLOR);
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
