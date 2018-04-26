
#[cfg(target_os = "emscripten")]
pub mod emscripten {
    use std::cell::RefCell;
    use std::ptr::null_mut;
    use std::os::raw::c_void;
    use emscripten_sys::{emscripten_set_main_loop};

    thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

    pub fn set_main_loop_callback<F>(callback: F) where F: FnMut() {
        MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
        });

        unsafe { emscripten_set_main_loop(Some(wrapper::<F>), 0, 1); }

        unsafe extern "C" fn wrapper<F>() where F: FnMut() {
            MAIN_LOOP_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)();
            });
        }
    }
}
