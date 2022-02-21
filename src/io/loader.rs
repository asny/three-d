use crate::core::*;
use crate::io::*;
use reqwest::Url;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

///
/// Convenience functionality to load some resources and, when loaded, use them to create one or more objects (for example a 3D model, a skybox, a texture etc).
/// To get the loaded object, use the `borrow()` or `borrow_mut()` methods which returns `Some` reference to the object if loaded and `None` otherwise.
///
pub struct Loading<T> {
    load: Rc<RefCell<Option<ThreeDResult<T>>>>,
}

impl<T: 'static> Loading<T> {
    ///
    /// Starts loading the resources defined by `paths` and calls the `on_load` closure when everything is loaded.
    ///
    pub fn new(
        context: &Context,
        paths: &[impl AsRef<Path>],
        on_load: impl 'static + FnOnce(Context, ThreeDResult<Loaded>) -> ThreeDResult<T>,
    ) -> Self {
        let load = Rc::new(RefCell::new(None));
        let load_clone = load.clone();
        let context_clone = context.clone();
        Loader::load(paths, move |loaded| {
            *load_clone.borrow_mut() = Some(on_load(context_clone, loaded));
        });
        Self { load }
    }

    ///
    /// Returns true if the object is loaded and mapped by the `on_load` closure.
    ///
    pub fn is_loaded(&self) -> bool {
        self.load.borrow().is_some()
    }
}

impl<T: 'static> std::ops::Deref for Loading<T> {
    type Target = Rc<RefCell<Option<ThreeDResult<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.load
    }
}

impl<T: 'static> std::ops::DerefMut for Loading<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.load
    }
}

///
/// Contains the resources loaded using the [Loader](crate::Loader) and/or manually inserted using the [insert_bytes](Self::insert_bytes) method.
/// Use the [remove_bytes](crate::Loaded::remove_bytes) or [get_bytes](crate::Loaded::get_bytes) function to extract the raw byte array for the loaded resource
/// or one of the other methods to both extract and deserialize a loaded resource.
///
#[derive(Default)]
pub struct Loaded {
    loaded: HashMap<PathBuf, Vec<u8>>,
}

impl Loaded {
    ///
    /// Constructs a new empty set of loaded files. Use this together with [insert_bytes](Self::insert_bytes) to load resources
    /// from an unsuported source and then parse them as usual using the functionality on Loaded.
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Remove and returns the loaded byte array for the resource at the given path.
    /// The byte array then has to be deserialized to whatever type this resource is (image, 3D model etc.).
    ///
    pub fn remove_bytes(&mut self, path: impl AsRef<Path>) -> ThreeDResult<Vec<u8>> {
        if let Some((_, bytes)) = self.loaded.remove_entry(path.as_ref()) {
            Ok(bytes)
        } else {
            let key = self
                .loaded
                .iter()
                .find(|(k, _)| {
                    k.to_str()
                        .unwrap()
                        .contains(path.as_ref().to_str().unwrap())
                })
                .ok_or(IOError::NotLoaded(
                    path.as_ref().to_str().unwrap().to_owned(),
                ))?
                .0
                .clone();
            Ok(self.loaded.remove(&key).unwrap())
        }
    }

    ///
    /// Returns a reference to the loaded byte array for the resource at the given path.
    /// The byte array then has to be deserialized to whatever type this resource is (image, 3D model etc.).
    ///
    pub fn get_bytes(&mut self, path: impl AsRef<Path>) -> ThreeDResult<&[u8]> {
        if let Some(bytes) = self.loaded.get(path.as_ref()) {
            Ok(bytes.as_ref())
        } else {
            let key = self
                .loaded
                .iter()
                .find(|(k, _)| {
                    k.to_str()
                        .unwrap()
                        .contains(path.as_ref().to_str().unwrap())
                })
                .ok_or(IOError::NotLoaded(
                    path.as_ref().to_str().unwrap().to_owned(),
                ))?
                .0;
            Ok(self.loaded.get(key).unwrap())
        }
    }

    ///
    /// Inserts the given bytes into the set of loaded files which is useful if you want to load the data from an unsuported source.
    /// The files can then be parsed as usual using the functionality on Loaded.
    ///
    pub fn insert_bytes(&mut self, path: impl AsRef<Path>, bytes: Vec<u8>) {
        self.loaded.insert(path.as_ref().to_path_buf(), bytes);
    }
}

impl std::fmt::Debug for Loaded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Loaded");
        for (key, value) in self.loaded.iter() {
            d.field("path", key);
            d.field("byte length", &value.len());
        }
        d.finish()
    }
}

///
/// Functionality for loading any type of resource runtime on both desktop and web.
///
pub struct Loader {}

impl Loader {
    ///
    /// Loads all of the resources in the given paths then calls `on_done` with all of the [Loaded] resources.
    /// Uses async loading when possible.
    /// Alternatively use [Loader::load_async] on both web and desktop or [Loader::load_blocking] on desktop.
    ///
    /// **Note:** This method must not be called from an async function. In that case, use [Loader::load_async] instead.
    ///
    pub fn load(paths: &[impl AsRef<Path>], on_done: impl 'static + FnOnce(ThreeDResult<Loaded>)) {
        #[cfg(target_arch = "wasm32")]
        {
            let paths: Vec<PathBuf> = paths.iter().map(|p| p.as_ref().to_path_buf()).collect();
            wasm_bindgen_futures::spawn_local(async move {
                let loaded = Self::load_async(&paths).await;
                on_done(loaded);
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            on_done(Self::load_blocking(paths));
        }
    }

    ///
    /// Loads all of the resources in the given paths and returns the [Loaded] resources.
    /// Uses async loading when possible.
    ///
    /// **Note:** This method must not be called from an async function. In that case, use [Loader::load_async] instead.
    ///
    #[cfg_attr(docsrs, doc(not(target_arch = "wasm32")))]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_blocking(paths: &[impl AsRef<Path>]) -> ThreeDResult<Loaded> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(Self::load_async(paths))
    }

    ///
    /// Async loads all of the resources in the given paths and returns the [Loaded] resources.
    ///
    #[cfg(target_arch = "wasm32")]
    pub async fn load_async(paths: &[impl AsRef<Path>]) -> ThreeDResult<Loaded> {
        let base_path = PathBuf::from(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .url()
                .unwrap(),
        );

        let mut handles = Vec::new();
        let client = reqwest::Client::new();
        for path in paths.iter() {
            let mut p = path.as_ref().to_path_buf();
            if !is_absolute_url(p.to_str().unwrap()) {
                p = base_path.join(p);
            }
            let url = Url::parse(p.to_str().unwrap())?;
            handles.push((p, get(client.clone(), url)));
        }

        let mut loaded = Loaded::new();
        for (path, handle) in handles.drain(..) {
            let bytes = handle
                .await
                .map_err(|e| IOError::FailedLoadingUrl(path.to_str().unwrap().to_string(), e))?;
            loaded.loaded.insert(path, bytes);
        }
        Ok(loaded)
    }

    ///
    /// Async loads all of the resources in the given paths and returns the [Loaded] resources.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn load_async(paths: &[impl AsRef<Path>]) -> ThreeDResult<Loaded> {
        let mut handles = Vec::new();
        let mut url_handles = Vec::new();
        let mut loaded = Loaded::new();
        let client = reqwest::Client::new();
        for path in paths.iter() {
            let path = path.as_ref().to_path_buf();
            if is_absolute_url(path.to_str().unwrap()) {
                let url = Url::parse(path.to_str().unwrap())?;
                url_handles.push((path.clone(), get(client.clone(), url)));
            } else {
                handles.push((path.clone(), tokio::fs::read(path)));
            }
        }

        for (path, handle) in handles.drain(..) {
            let bytes = handle
                .await
                .map_err(|e| IOError::FailedLoading(path.to_str().unwrap().to_string(), e))?;
            loaded.loaded.insert(path, bytes);
        }
        for (path, handle) in url_handles.drain(..) {
            let bytes = handle
                .await
                .map_err(|e| IOError::FailedLoadingUrl(path.to_str().unwrap().to_string(), e))?;
            loaded.loaded.insert(path, bytes);
        }
        Ok(loaded)
    }
}

async fn get(client: reqwest::Client, url: Url) -> reqwest::Result<Vec<u8>> {
    Ok(client.get(url).send().await?.bytes().await?.to_vec())
}

fn is_absolute_url(path: &str) -> bool {
    path.find("://").map(|i| i > 0).unwrap_or(false)
        || path.find("//").map(|i| i == 0).unwrap_or(false)
}
