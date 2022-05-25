//!
//! *[Deprecated: Use the [three-d-asset](https://github.com/asny/three-d-asset) crate instead.]*
//! Contains functionality to load any type of asset runtime on both desktop and web as well as parsers for different image and 3D model formats.
//! Also includes functionality to save data which is limited to desktop.
//!

#![allow(missing_docs)]
#![deprecated = "moved to the `three-d-asset` crate"]

use crate::{CpuMaterial, CpuMesh, CpuTexture, CpuVolume};
use std::path::Path;
use three_d_asset::Result;

///
/// Functionality for loading any type of resource runtime on both desktop and web.
///
pub struct Loader {}

impl Loader {
    ///
    /// Loads all of the resources in the given paths then calls `on_done` with all of the [Loaded] resources.
    /// Alternatively use [Self::load_async] on both web and desktop or [Self::load_blocking] on desktop.
    ///
    /// **Note:** This method must not be called from an async function. In that case, use [Self::load_async] instead.
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
        Ok(Loaded(three_d_asset::io::load(paths)?))
    }

    pub async fn load_async(paths: &[impl AsRef<Path>]) -> Result<Loaded> {
        Ok(Loaded(three_d_asset::io::load_async(paths).await?))
    }
}

pub struct Loaded(three_d_asset::io::RawAssets);

impl Loaded {
    pub fn gltf<P: AsRef<Path>>(&mut self, path: P) -> Result<(Vec<CpuMesh>, Vec<CpuMaterial>)> {
        let r: three_d_asset::Model = self.deserialize(path)?;
        Ok((r.geometries, r.materials))
    }

    pub fn obj<P: AsRef<Path>>(&mut self, path: P) -> Result<(Vec<CpuMesh>, Vec<CpuMaterial>)> {
        let r: three_d_asset::Model = self.deserialize(path)?;
        Ok((r.geometries, r.materials))
    }

    pub fn vol<P: AsRef<Path>>(&mut self, path: P) -> Result<CpuVolume> {
        self.deserialize(path)
    }

    ///
    /// Deserialize the image resource at the given path into a [CpuTexture].
    ///
    pub fn image<P: AsRef<Path>>(&mut self, path: P) -> Result<CpuTexture> {
        self.deserialize(path)
    }
}

impl std::ops::Deref for Loaded {
    type Target = three_d_asset::io::RawAssets;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Loaded {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

///
/// Functionality for saving resources. Only available on desktop at the moment.
///
#[cfg(not(target_arch = "wasm32"))]
pub struct Saver {}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {
    ///
    /// Save the byte array as a file.
    ///
    pub fn save_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> crate::ThreeDResult<()> {
        let mut file = std::fs::File::create(path)?;
        use std::io::prelude::*;
        file.write_all(bytes)?;
        Ok(())
    }
}
