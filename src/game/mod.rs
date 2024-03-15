use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

pub fn run() {
    let sdl_context = sdl2::init().expect("Cannot init SDL");
    let video_subsystem = sdl_context.video().expect("Cannot create video");

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .fullscreen()
        .opengl()
        .build()
        .expect("Cannot create window");

    let mut canvas = window.into_canvas().build().expect("Cannot create canvas");

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().expect("Cannot pump event");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
}
