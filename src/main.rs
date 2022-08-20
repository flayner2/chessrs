use sdl2::{event::Event, pixels::Color};

const WIDTH: u32 = 700;
const HEIGHT: u32 = 700;
const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);

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

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        // Draw graphics
        canvas.present();
    }
}
