
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;

type Ref<T> = Rc<RefCell<T>>;
type Loads = Ref<HashMap<String, Ref<Vec<u8>>>>;

pub struct Loader {
    loads: Loads
}

impl Loader {

    pub fn new() -> Self {
        Self { loads: Rc::new(RefCell::new(HashMap::new()))}
    }

    pub fn load_cpu_mesh(&mut self, path: &'static str)
    {
        let bytes = Rc::new(RefCell::new(Vec::new()));

        Self::load_file(path,bytes.clone());

        self.loads.borrow_mut().insert(path.to_string(), bytes);

    }

    pub fn wait_all<F>(&mut self, callback: F)
        where F: 'static + FnOnce(Loads)
    {
        Self::wait(self.loads.clone(), callback);
    }

    fn wait<F>(loads: Loads, callback: F)
        where F: 'static + FnOnce(Loads)
    {
        info!("Wait");
        Self::sleep(1000, move || {

            let mut is_loading = false;
            for bytes in loads.borrow_mut().values() {
                if let Ok(b) = bytes.try_borrow() {
                    is_loading = b.len() == 0;
                }
            }
            info!("Is loading: {}", is_loading);

            if is_loading {
                Self::wait(loads, callback);
            } else {
                callback(loads);
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
    fn load_file(path: &'static str, bytes: Rc<RefCell<Vec<u8>>>)
    {
        let mut file = std::fs::File::open(path).unwrap();
        use std::io::prelude::*;
        file.read_to_end(&mut bytes.borrow_mut()).unwrap();
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file(path: &'static str, bytes: Rc<RefCell<Vec<u8>>>)
    {
        wasm_bindgen_futures::spawn_local(Self::load(path, bytes));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load(url: &'static str, bytes: Rc<RefCell<Vec<u8>>>)
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
        *bytes.borrow_mut() = js_sys::Uint8Array::new(&data).to_vec();
    }
}

