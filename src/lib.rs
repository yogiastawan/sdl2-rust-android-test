mod game;

use game::run;
use sdl2::libc;

#[no_mangle]
pub extern "C" fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    run();
    return 0;
}
