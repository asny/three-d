//!
//! Contains functionality to load any type of asset runtime on both desktop and web as well as parsers for different image and 3D model formats.
//! Also includes functionality to save data which is limited to desktop.
//!

#![allow(missing_docs)]
#![deprecated = "moved to the `three-d-io` crate"]

use std::path::Path;
use three_d_asset::Result;

///
/// Functionality for loading any type of resource runtime on both desktop and web.
///
pub struct Loader {}

impl Loader {
    ///
    /// Loads all of the resources in the given paths then calls `on_done` with all of the [Loaded] resources.
    /// Alternatively use [load_async] on both web and desktop or [load_blocking] on desktop.
    ///
    /// **Note:** This method must not be called from an async function. In that case, use [load_async] instead.
    ///
    pub fn load(paths: &[impl AsRef<Path>], on_done: impl 'static + FnOnce(Result<Loaded>)) {
        #[cfg(target_arch = "wasm32")]
        {
            let paths: Vec<std::path::PathBuf> =
                paths.iter().map(|p| p.as_ref().to_path_buf()).collect();
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_blocking(paths: &[impl AsRef<Path>]) -> Result<Loaded> {
        three_d_asset::io::load_blocking(paths)
    }

    pub async fn load_async(paths: &[impl AsRef<Path>]) -> Result<Loaded> {
        three_d_asset::io::load_async(paths).await
    }
}

pub type Loaded = three_d_asset::io::Loaded;

///
/// Functionality for saving resources. Only available on desktop at the moment.
///
#[cfg(not(target_arch = "wasm32"))]
pub struct Saver {}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {
    pub fn save(path: impl AsRef<Path>, bytes: &[u8]) -> Result<()> {
        three_d_asset::io::save(path, bytes)
    }
}
