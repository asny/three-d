use crate::io::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

///
/// Contains the resources loaded using the [Loader](crate::Loader) and/or manually inserted using the [insert_bytes](Self::insert_bytes) method.
/// Use the [remove_bytes](crate::Loaded::remove_bytes) or [get_bytes](crate::Loaded::get_bytes) function to extract the raw byte array for the loaded resource
/// or one of the other methods to both extract and deserialize a loaded resource.
///
#[derive(Default, Debug)]
pub struct Loaded {
    loaded: HashMap<PathBuf, std::result::Result<Vec<u8>, std::io::Error>>,
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
    pub fn remove_bytes<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<u8>> {
        let bytes = self
            .loaded
            .remove_entry(path.as_ref())
            .ok_or(IOError::NotLoaded(
                path.as_ref().to_str().unwrap().to_owned(),
            ))?
            .1?;
        Ok(bytes)
    }

    ///
    /// Returns a reference to the loaded byte array for the resource at the given path.
    /// The byte array then has to be deserialized to whatever type this resource is (image, 3D model etc.).
    ///
    pub fn get_bytes<P: AsRef<Path>>(&mut self, path: P) -> Result<&[u8]> {
        let bytes = self
            .loaded
            .get(path.as_ref())
            .ok_or(IOError::NotLoaded(
                path.as_ref().to_str().unwrap().to_owned(),
            ))?
            .as_ref()
            .map_err(|e| std::io::Error::from(e.kind()))?;
        Ok(bytes)
    }

    ///
    /// Inserts the given bytes into the set of loaded files which is useful if you want to load the data from an unsuported source.
    /// The files can then be parsed as usual using the functionality on Loaded.
    ///
    pub fn insert_bytes<P: AsRef<Path>>(&mut self, path: P, bytes: Vec<u8>) {
        self.loaded.insert(path.as_ref().to_path_buf(), Ok(bytes));
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
        F: 'static + FnOnce(Loaded),
    {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(Self::load_files_async(
                paths.iter().map(|p| p.as_ref().to_path_buf()).collect(),
                on_done,
            ));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut loaded = Loaded::new();
            for path in paths {
                let result = if let Ok(url) = reqwest::Url::parse(path.as_ref().to_str().unwrap()) {
                    Ok(reqwest::blocking::get(url)
                        .map(|r| (*r.bytes().unwrap()).to_vec())
                        .unwrap())
                } else {
                    std::fs::read(path.as_ref())
                };
                loaded.loaded.insert(path.as_ref().to_path_buf(), result);
            }
            on_done(loaded)
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_files_async<F>(paths: Vec<PathBuf>, on_done: F)
    where
        F: 'static + FnOnce(Loaded),
    {
        let mut loads = Loaded::new();
        for path in paths.iter() {
            let url = reqwest::Url::parse(path.to_str().unwrap()).unwrap_or_else(|_| {
                let u = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .url()
                    .unwrap();
                let p = if !u.ends_with("/") {
                    std::path::PathBuf::from(u).parent().unwrap().join(path)
                } else {
                    std::path::PathBuf::from(u.clone()).join(path)
                };
                reqwest::Url::parse(p.to_str().unwrap()).unwrap()
            });
            let data = reqwest::get(url).await.unwrap().bytes().await.unwrap();
            loads.loaded.insert(path.clone(), Ok((*data).to_vec()));
        }
        on_done(loads)
    }
}
