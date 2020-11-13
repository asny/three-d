use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;

type Loads = HashMap<&'static str, Result<Vec<u8>, std::io::Error>>;
type RefLoads = Rc<RefCell<Loads>>;

pub struct Loader {
    loads: RefLoads
}

impl Loader {

    pub fn new() -> Self {
        Self { loads: Rc::new(RefCell::new(HashMap::new()))}
    }

    pub fn start_loading(&mut self, path: &'static str)
    {
        self.loads.borrow_mut().insert(path, Ok(Vec::new()));
        Self::load_file(path,self.loads.clone());
    }

    pub fn wait<F>(&mut self, on_done: F)
        where F: 'static + FnOnce(&mut Loads)
    {
        self.wait_with_progress(|progress| {
                    info!("Progress: {}%", 100.0f32 * progress);
        }, on_done);
    }

    pub fn wait_with_progress<F, G>(&mut self, progress_callback: G, on_done: F)
        where
            G: 'static + Fn(f32),
            F: 'static + FnOnce(&mut Loads)
    {
        info!("Loading started...");
        Self::wait_local(self.loads.clone(), progress_callback, on_done);
    }

    fn wait_local<F, G>(loads: RefLoads, progress_callback: G, on_done: F)
        where
            G: 'static + Fn(f32),
            F: 'static + FnOnce(&mut Loads)
    {
        Self::sleep(100, move || {

            let mut is_loading = false;
            match loads.try_borrow() {
                Ok(map) => {
                    let total_count = map.len();
                    let mut count = 0;
                    for bytes in map.values() {
                        if bytes.is_err() || bytes.as_ref().unwrap().len() > 0 {
                            count = count + 1;
                        }
                    }
                    progress_callback(count as f32 / total_count as f32);
                    is_loading = count < total_count;
                },
                Err(_) => is_loading = true
            }

            if is_loading {
                Self::wait_local(loads, progress_callback, on_done);
            } else {
                info!("Loading done.");
                on_done(&mut loads.borrow_mut());
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
    fn load_file(path: &'static str, loads: RefLoads)
    {
        let file = std::fs::File::open(path);
        match file {
            Ok(mut f) => {
                use std::io::prelude::*;
                let mut bytes = Vec::new();
                let result = f.read_to_end(&mut bytes).and(Ok(bytes));
                loads.borrow_mut().insert(path, result);
            },
            Err(e) => {loads.borrow_mut().insert(path, Err(e));}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file(path: &'static str, loads: RefLoads)
    {
        wasm_bindgen_futures::spawn_local(Self::load(path, loads));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load(url: &'static str, loads: RefLoads)
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
        loads.borrow_mut().insert(url, Ok(js_sys::Uint8Array::new(&data).to_vec()));
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