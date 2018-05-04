
#[cfg(target_os = "emscripten")]
pub mod emscripten {
    use std;
    use std::cell::RefCell;
    use std::ptr::null_mut;
    use std::os::raw::c_void;
    use std::ffi::{CStr, CString};
    use emscripten_sys::{emscripten_set_main_loop, emscripten_wget, emscripten_async_wget, em_str_callback_func,emscripten_fetch_t, emscripten_fetch_attr_t, emscripten_fetch_attr_init,
                     emscripten_fetch, emscripten_fetch_close};

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

    fn body_string(fetch: &emscripten_fetch_t) -> String {
        let data = unsafe { std::mem::transmute::<*const i8, *mut u8>((*fetch).data) };
        let len = (*fetch).totalBytes as usize;
        let slice = unsafe { std::slice::from_raw_parts(data, len) };
        let mut v = Vec::with_capacity(len);
        v.resize(len, 0);
        v.copy_from_slice(slice);
        String::from_utf8(v).ok().unwrap()
    }

    extern "C" fn handle_success(fetch: *mut emscripten_fetch_t) {
        unsafe {
            let body = body_string(&*fetch);
            println!("Success");
            println!("{}", body);
            emscripten_fetch_close(fetch);
        }
    }

    extern "C" fn handle_error(fetch: *mut emscripten_fetch_t) {
        unsafe {
            println!("error: status code {}", (*fetch).status);
            emscripten_fetch_close(fetch);
        }
    }

    // Needs -s FETCH=1
    pub fn fetch(url: &str)
    {
        unsafe {

            let mut fetch_arg: emscripten_fetch_attr_t = std::mem::uninitialized();
            emscripten_fetch_attr_init(&mut fetch_arg);
            fetch_arg.attributes = 1;// | 64;
            fetch_arg.onsuccess = Some(handle_success);
            fetch_arg.onerror = Some(handle_error);
            let url_c = std::ffi::CString::new(url).unwrap();
            emscripten_fetch(&mut fetch_arg, url_c.as_ptr());
        }
    }

    thread_local!(static ON_LOAD_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));
    thread_local!(static ON_ERROR_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

    pub fn async_wget<F, E>(name: &str, on_load: F, on_error: E) where F: FnMut(String), E: FnMut(String)
    {
        ON_LOAD_CALLBACK.with(|log| {
            *log.borrow_mut() = &on_load as *const _ as *mut c_void;
        });

        ON_ERROR_CALLBACK.with(|log| {
            *log.borrow_mut() = &on_error as *const _ as *mut c_void;
        });

        let mut path = String::from("http://localhost:8000/");
        path.push_str(name);
        let temp: &str = path.as_ref();
        println!("{}   {}", path, name);

        let url = CString::new(temp).unwrap();
        let file = CString::new(name).unwrap();

        unsafe {
            emscripten_async_wget(url.as_ptr(), file.as_ptr(),
                                       Some(on_load_wrapper::<F>),
                                       Some(on_error_wrapper::<E>));
        }

        unsafe extern "C" fn on_load_wrapper<F>(char_ptr: *const ::std::os::raw::c_char) where F: FnMut(String)
        {
            let arg = CStr::from_ptr(char_ptr).to_str().unwrap().to_string();
            ON_LOAD_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)(arg);
            });
        }

        unsafe extern "C" fn on_error_wrapper<E>(char_ptr: *const ::std::os::raw::c_char) where E: FnMut(String)
        {
            let arg = CStr::from_ptr(char_ptr).to_str().unwrap().to_string();
            ON_ERROR_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut E;
                (*closure)(arg);
            });
        }
    }

    // Needs -s ASYNCIFY=1
    pub fn wget(name: &str)
    {
        let mut path = String::from("http://localhost:8000/");
        path.push_str(name);
        let temp: &str = path.as_ref();
        println!("{}   {}", path, name);

        let url = CString::new(temp).unwrap();
        let file = CString::new(name).unwrap();
        unsafe {
            emscripten_wget(url.as_ptr(), file.as_ptr());
        }
    }
}
