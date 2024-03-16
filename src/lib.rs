mod game;

use game::run;
use sdl2::{libc, log::log};

#[no_mangle]
pub extern "C" fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    let a = run().err();
    if let Some(x) = a {
        log(&x);
        return -1;
    }
    return 0;
}
