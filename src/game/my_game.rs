use std::cell::{Cell, RefCell};

use sdl2::{event::Event, log::log, Sdl};

use super::my_scene::Scene;

pub struct MyGame {
    state: Cell<MyGameState>,
    sdl_context: RefCell<Sdl>,
    title: String,
    close_game: Cell<bool>,
    scene: RefCell<Box<dyn Scene>>,
}

pub enum MyGameState {
    Playing,
    Paused,
}

impl MyGame {
    pub fn new(str: &str, scene: Box<dyn Scene>) -> Result<MyGame, String> {
        let sdl_context = sdl2::init()?;

        Ok(MyGame {
            state: Cell::new(MyGameState::Paused),
            sdl_context: RefCell::new(sdl_context),
            title: str.to_owned(),
            close_game: Cell::new(false),
            scene: RefCell::new(scene),
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let sdl_ctx = self.sdl_context.borrow();
        let video = sdl_ctx.video()?;

        let window = video
            .window(&self.title, 0, 0)
            .position_centered()
            .fullscreen()
            .borderless()
            .build()
            .map_err(|e| e.to_string())?;

        let mut events = sdl_ctx.event_pump()?;

        loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { timestamp } => {
                        log(&format!("My Game exited after {}", timestamp));
                        self.close_game();
                    }
                    _ => {
                        self.set_event_handler(&event);
                    }
                }
            }

            if self.close_game.get() {
                break;
            }

            //draw scene
            self.draw_scene(&sdl_ctx.clone());
        }

        Ok(())
    }

    pub fn set_event_handler(&self, events: &Event) {
        let scene = self.scene.borrow();
        scene.update(events);
    }

    pub fn close_game(&self) {
        self.close_game.set(true);
    }

    pub fn draw_scene(&self, ctx: &Sdl) {
        let scene = self.scene.borrow();
        scene.draw()
    }

    pub fn set_state(&self, state: MyGameState) {
        self.state.set(state);
    }
}
