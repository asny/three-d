
#[cfg(target_os = "emscripten")]
pub fn set_main_loop<F>(main_loop: F) where F: FnMut()
{
    use emscripten::{emscripten};
    emscripten::set_main_loop_callback(main_loop);
}

#[cfg(not(target_os = "emscripten"))]
pub fn set_main_loop<F>(mut main_loop: F) where F: FnMut()
{
    loop { main_loop(); }
}