
#[cfg(target_os = "emscripten")]
use emscripten::{emscripten};

pub fn set_main_loop<F>(mut main_loop: F) where F: FnMut()
{
    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
