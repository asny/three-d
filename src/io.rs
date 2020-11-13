use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;

type Loads = Rc<RefCell<HashMap<&'static str, Vec<u8>>>>;

pub struct Loader {
    loads: Loads
}

impl Loader {

    pub fn new() -> Self {
        Self { loads: Rc::new(RefCell::new(HashMap::new()))}
    }

    pub fn start_loading(&mut self, path: &'static str)
    {
        self.loads.borrow_mut().insert(path, Vec::new());
        Self::load_file(path,self.loads.clone());
    }

    pub fn wait_all<F>(&mut self, callback: F)
        where F: 'static + FnOnce(&mut HashMap<&'static str, Vec<u8>>)
    {
        Self::wait(self.loads.clone(), callback);
    }

    fn wait<F>(loads: Loads, callback: F)
        where F: 'static + FnOnce(&mut HashMap<&'static str, Vec<u8>>)
    {
        info!("Wait");
        Self::sleep(100, move || {

            let mut is_loading = false;
            match loads.try_borrow() {
                Ok(map) => {

                    for bytes in map.values() {
                        if !is_loading {
                            is_loading = bytes.len() == 0
                        }
                    }
                },
                Err(_) => is_loading = true
            }
            info!("Is loading: {}", is_loading);

            if is_loading {
                Self::wait(loads, callback);
            } else {
                callback(&mut loads.borrow_mut());
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sleep<F>(millis: u64, fun: F)
    where
        F: 'static + FnOnce()
    {
        std::thread::sleep(std::time::Duration::from_millis(millis));
        fun();
    }

    #[cfg(target_arch = "wasm32")]
    fn sleep<F>(millis: u64, fun: F)
    where
        F: 'static + FnOnce()
    {
        use gloo_timers::callback::Timeout;
        let timeout = Timeout::new(millis as u32, fun);
        timeout.forget();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_file(path: &'static str, loads: Loads)
    {
        let mut file = std::fs::File::open(path).unwrap();
        use std::io::prelude::*;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        loads.borrow_mut().insert(path, bytes);
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file(path: &'static str, loads: Loads)
    {
        wasm_bindgen_futures::spawn_local(Self::load(path, loads));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load(url: &'static str, loads: Loads)
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
        request.headers().set("Accept", "application/octet-stream").unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let data: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        loads.borrow_mut().insert(url, js_sys::Uint8Array::new(&data).to_vec());
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_file(path: &str, bytes: &[u8]) -> Result<(), std::io::Error>
{
    let mut file = std::fs::File::create(path)?;
    use std::io::prelude::*;
    file.write_all(bytes)?;
    Ok(())
}