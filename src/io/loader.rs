use crate::io::*;
use log::info;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

type RefLoaded = Rc<RefCell<HashMap<PathBuf, Result<Vec<u8>, std::io::Error>>>>;

///
/// The resources loaded using the [Loader](crate::Loader).
/// Use the [remove_bytes](crate::Loaded::remove_bytes) or [get_bytes](crate::Loaded::get_bytes) function to extract the raw byte array for the loaded resource
/// or one of the other methods to both extract and deserialize a loaded resource.
///
pub struct Loaded<'a> {
    loaded: &'a mut HashMap<PathBuf, Result<Vec<u8>, std::io::Error>>,
}

impl<'a> Loaded<'a> {
    ///
    /// Remove and returns the loaded byte array for the resource at the given path.
    /// The byte array then has to be deserialized to whatever type this resource is (image, 3D model etc.).
    ///
    pub fn remove_bytes<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<u8>, IOError> {
        let bytes = self
            .loaded
            .remove_entry(path.as_ref())
            .ok_or(IOError::FailedToLoad {
                message: format!(
                    "Tried to use a resource which was not loaded: {}",
                    path.as_ref().to_str().unwrap()
                ),
            })?
            .1
            .map_err(|e| IOError::FailedToLoad {
                message: format!(
                    "Could not load resource {} due to: {}",
                    path.as_ref().to_str().unwrap(),
                    e
                ),
            })?;
        Ok(bytes)
    }

    ///
    /// Returns a reference to the loaded byte array for the resource at the given path.
    /// The byte array then has to be deserialized to whatever type this resource is (image, 3D model etc.).
    ///
    pub fn get_bytes<P: AsRef<Path>>(&mut self, path: P) -> Result<&[u8], IOError> {
        let bytes = self
            .loaded
            .get(path.as_ref())
            .ok_or(IOError::FailedToLoad {
                message: format!(
                    "Tried to use a resource which was not loaded: {}",
                    path.as_ref().to_str().unwrap()
                ),
            })?
            .as_ref()
            .map_err(|e| IOError::FailedToLoad {
                message: format!(
                    "Could not load resource {} due to: {}",
                    path.as_ref().to_str().unwrap(),
                    e
                ),
            })?;
        Ok(bytes)
    }
}

///
/// Functionality for loading any type of resource runtime on both desktop and web.
///
pub struct Loader {}

impl Loader {
    ///
    /// Loads all of the resources in the given paths then calls `on_done` with all of the [loaded resources](crate::Loaded).
    ///
    pub fn load<F, P: AsRef<Path>>(paths: &[P], on_done: F)
    where
        F: 'static + FnOnce(&mut Loaded),
    {
        Self::load_with_progress(
            paths,
            |progress| {
                info!("Progress: {}%", 100.0f32 * progress);
            },
            on_done,
        );
    }

    ///
    /// Loads all of the resources in the given paths then calls `on_done` with all of the [loaded resources](crate::Loaded).
    /// Will continuously call `progress_callback` while loading.
    ///
    pub fn load_with_progress<F, G, P>(paths: &[P], progress_callback: G, on_done: F)
    where
        G: 'static + Fn(f32),
        F: 'static + FnOnce(&mut Loaded),
        P: AsRef<Path>,
    {
        let loads = Rc::new(RefCell::new(HashMap::new()));
        for path in paths {
            loads
                .borrow_mut()
                .insert(path.as_ref().to_path_buf(), Ok(Vec::new()));
            Self::load_file(path, loads.clone());
        }
        info!("Loading started...");
        Self::wait_local(loads.clone(), progress_callback, on_done);
    }

    fn wait_local<F, G>(loads: RefLoaded, progress_callback: G, on_done: F)
    where
        G: 'static + Fn(f32),
        F: 'static + FnOnce(&mut Loaded),
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
                }
                Err(_) => true,
            };

            if is_loading {
                Self::wait_local(loads, progress_callback, on_done);
            } else {
                info!("Loading done.");
                on_done(&mut Loaded {
                    loaded: &mut loads.borrow_mut(),
                });
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sleep<F>(millis: u64, fun: F)
    where
        F: 'static + FnOnce(),
    {
        std::thread::sleep(std::time::Duration::from_millis(millis));
        fun();
    }

    #[cfg(target_arch = "wasm32")]
    fn sleep<F>(millis: u64, fun: F)
    where
        F: 'static + FnOnce(),
    {
        use gloo_timers::callback::Timeout;
        let timeout = Timeout::new(millis as u32, fun);
        timeout.forget();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_file<P: AsRef<Path>>(path: P, loads: RefLoaded) {
        let result = if let Ok(url) = reqwest::Url::parse(path.as_ref().to_str().unwrap()) {
            Ok(reqwest::blocking::get(url)
                .map(|r| (*r.bytes().unwrap()).to_vec())
                .unwrap())
        } else {
            std::fs::read(path.as_ref())
        };
        loads
            .borrow_mut()
            .insert(path.as_ref().to_path_buf(), result);
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file<P: AsRef<Path>>(path: P, loads: RefLoaded) {
        let url = reqwest::Url::parse(path.as_ref().to_str().unwrap()).unwrap_or_else(|_| {
            let u = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .url()
                .unwrap();
            reqwest::Url::parse(
                std::path::PathBuf::from(u)
                    .join(path.as_ref())
                    .to_str()
                    .unwrap(),
            )
            .unwrap()
        });
        wasm_bindgen_futures::spawn_local(Self::load_file_async(
            path.as_ref().to_path_buf(),
            url,
            loads,
        ));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_file_async<P: AsRef<Path>>(path: P, url: reqwest::Url, loads: RefLoaded) {
        let data = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        loads
            .borrow_mut()
            .insert(path.as_ref().to_path_buf(), Ok((*data).to_vec()));
    }
}
