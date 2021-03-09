use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;
use std::path::{Path, PathBuf};
use crate::io::*;
use crate::definition::*;

type RefLoaded = Rc<RefCell<HashMap<PathBuf, Result<Vec<u8>, std::io::Error>>>>;

///
/// The resources loaded using the [Loader](crate::Loader).
/// Use the [get](crate::Loaded::get) function to extract the raw byte array for the loaded resource
/// or one of the built-in deserialize methods to both extract and deserialize a loaded resource.
///
pub struct Loaded<'a>  {
    loaded: &'a mut HashMap<PathBuf, Result<Vec<u8>, std::io::Error>>
}

impl<'a> Loaded<'a> {

    ///
    /// Returns the loaded byte array for the resource at the given path.
    /// The byte array then has to be deserialized to whatever type this resource is (image, 3D model etc.).
    ///
    pub fn get<P: AsRef<Path>>(&'a self, path: P) -> Result<&'a [u8], IOError> {
        let bytes = self.loaded.get(path.as_ref()).ok_or(
            IOError::FailedToLoad {message:format!("Tried to use a resource which was not loaded: {}", path.as_ref().to_str().unwrap())})?.as_ref()
            .map_err(|_| IOError::FailedToLoad {message:format!("Could not load resource: {}", path.as_ref().to_str().unwrap())})?;
        Ok(bytes)
    }

    ///
    /// Same as [Deserialize::obj](crate::Deserialize::obj).
    ///
    pub fn obj<P: AsRef<Path>>(&self, path: P) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), IOError> {
        Deserialize::obj(self, path)
    }

    ///
    /// Same as [Deserialize::three_d](crate::Deserialize::three_d).
    ///
    pub fn three_d<P: AsRef<Path>>(&self, path: P) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), IOError> {
        Deserialize::three_d(self, path)
    }

    ///
    /// Same as [Deserialize::image](crate::Deserialize::image).
    ///
    pub fn image<P: AsRef<Path>>(&self, path: P) -> Result<CPUTexture<u8>, IOError> {
        Deserialize::image(self, path)
    }

    ///
    /// Same as [Deserialize::cube_image](crate::Deserialize::cube_image).
    ///
    pub fn cube_image<P: AsRef<Path>>(&self, right_path: P, left_path: P,
                                      top_path: P, bottom_path: P, front_path: P, back_path: P) -> Result<CPUTexture<u8>, IOError> {
        Deserialize::cube_image(self, right_path, left_path, top_path, bottom_path, front_path, back_path)
    }
}

///
/// Functionality for loading any type of resource runtime on both desktop and web.
///
pub struct Loader {
}

impl Loader {

    ///
    /// Loads all of the resources in the given paths then calls **on_done** with all of the [loaded resources](crate::Loaded).
    ///
    pub fn load<F, P: AsRef<Path>>(paths: &[P], on_done: F)
        where F: 'static + FnOnce(&mut Loaded)
    {
        Self::load_with_progress(paths, |progress| {
                    info!("Progress: {}%", 100.0f32 * progress);
        }, on_done);
    }

    ///
    /// Loads all of the resources in the given paths then calls **on_done** with all of the [loaded resources](crate::Loaded).
    /// Will continuously call **progress_callback** while loading.
    ///
    pub fn load_with_progress<F, G, P>(paths: &[P], progress_callback: G, on_done: F)
        where
            G: 'static + Fn(f32),
            F: 'static + FnOnce(&mut Loaded),
            P: AsRef<Path>
    {
        let loads = Rc::new(RefCell::new(HashMap::new()));
        for path in paths {
            loads.borrow_mut().insert(path.as_ref().to_path_buf(), Ok(Vec::new()));
            Self::load_file(path,loads.clone());
        }
        info!("Loading started...");
        Self::wait_local(loads.clone(), progress_callback, on_done);
    }

    fn wait_local<F, G>(loads: RefLoaded, progress_callback: G, on_done: F)
        where
            G: 'static + Fn(f32),
            F: 'static + FnOnce(&mut Loaded)
    {
        Self::sleep(100, move || {

            let is_loading = match loads.try_borrow() {
                Ok(map) => {
                    let total_count = map.len();
                    let mut count = 0;
                    for bytes in map.values() {
                        if bytes.is_err() || bytes.as_ref().unwrap().len() > 0 {
                            count = count + 1;
                        }
                    }
                    progress_callback(count as f32 / total_count as f32);
                    count < total_count
                },
                Err(_) => true
            };

            if is_loading {
                Self::wait_local(loads, progress_callback, on_done);
            } else {
                info!("Loading done.");
                on_done(&mut Loaded{loaded: &mut loads.borrow_mut() });
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
    fn load_file<P: AsRef<Path>>(path: P, loads: RefLoaded)
    {
        let file = std::fs::File::open(path.as_ref());
        match file {
            Ok(mut f) => {
                use std::io::prelude::*;
                let mut bytes = Vec::new();
                let result = f.read_to_end(&mut bytes).and(Ok(bytes));
                loads.borrow_mut().insert(path.as_ref().to_path_buf(), result);
            },
            Err(e) => {loads.borrow_mut().insert(path.as_ref().to_path_buf(), Err(e));}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file<P: AsRef<Path>>(path: P, loads: RefLoaded)
    {
        wasm_bindgen_futures::spawn_local(Self::load_file_async(path.as_ref().to_path_buf(), loads));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_file_async<P: AsRef<Path>>(path: P, loads: RefLoaded)
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(path.as_ref().to_str().unwrap(), &opts).unwrap();
        request.headers().set("Accept", "application/octet-stream").unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let data: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        loads.borrow_mut().insert(path.as_ref().to_path_buf(), Ok(js_sys::Uint8Array::new(&data).to_vec()));
    }
}