use game::{my_game::MyGame, my_scene::MyScene};
use sdl2::log::log;

mod game;
fn main() {
    let home = MyScene::new();

    let game = match MyGame::new("test game", Box::new(home)) {
        Ok(x) => x,
        Err(e) => {
            log(&e);
            return;
        }
    };

    let run = game.run().err();

    if let Some(x) = run {
        log(&x);
    }
}
