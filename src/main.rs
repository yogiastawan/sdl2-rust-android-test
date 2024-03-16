use game::run;
use sdl2::log::log;

mod game;
fn main() {
    let a = run().err();
    if let Some(x) = a {
        log(&x);
    }
}
