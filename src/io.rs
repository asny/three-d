//!
//! Contains functionality to load any type of asset runtime on both desktop and web as well as parsers for different image and 3D model formats.
//! Also includes functionality to save data which is limited to desktop.
//!

#![deprecated = "moved to the `three-d-io` crate"]

///
/// Functionality for loading any type of resource runtime on both desktop and web.
///
pub type Loader = three_d_asset::Loader;

///
/// Contains the resources loaded using the [Loader](crate::Loader) and/or manually inserted using the [insert_bytes](Self::insert_bytes) method.
/// Use the [remove_bytes](crate::Loaded::remove_bytes) or [get_bytes](crate::Loaded::get_bytes) function to extract the raw byte array for the loaded resource
/// or one of the other methods to both extract and deserialize a loaded resource.
///
pub type Loaded = three_d_asset::Loaded;

///
/// Functionality for saving resources. Only available on desktop at the moment.
///
#[cfg(not(target_arch = "wasm32"))]
pub type Saver = three_d_asset::Saver;
