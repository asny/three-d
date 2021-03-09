
//!
//! Contain a [loader](crate::Loader) for loading any type of asset runtime on both desktop and web and a [saver](crate::Saver) for saving on desktop only.
//! It also contain [deserialize](crate::Deserialize) and [serialize](crate::Serialize) functionality for deserializing/serializing the loaded resources to/from 3D models, textures etc.
//!

#[doc(hidden)]
pub mod loader;
#[doc(inline)]
pub use loader::*;

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod saver;
#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use saver::*;

#[cfg(any(feature = "3d-io", feature = "obj-io", feature = "image-io"))]
pub struct Deserialize {}

#[cfg(any(feature = "3d-io", feature = "obj-io", feature = "image-io"))]
pub struct Serialize {}

#[doc(hidden)]
#[cfg(feature = "image-io")]
pub mod texture;
#[doc(inline)]
#[cfg(feature = "image-io")]
pub use texture::*;

#[doc(hidden)]
#[cfg(feature = "3d-io")]
pub mod threed;
#[doc(inline)]
#[cfg(feature = "3d-io")]
pub use threed::*;

#[doc(hidden)]
#[cfg(feature = "obj-io")]
pub mod obj;
#[doc(inline)]
#[cfg(feature = "obj-io")]
pub use obj::*;

///
/// Error message from the [core](crate::io) module.
///
#[derive(Debug)]
pub enum IOError {
    #[cfg(feature = "image-io")]
    Image(image::ImageError),
    #[cfg(feature = "3d-io")]
    Bincode(bincode::Error),
    #[cfg(feature = "obj-io")]
    Obj(wavefront_obj::ParseError),
    #[cfg(not(target_arch = "wasm32"))]
    IO(std::io::Error),
    FailedToLoad {message: String},
    FailedToSave {message: String}
}

#[cfg(feature = "image-io")]
impl From<image::ImageError> for IOError {
    fn from(other: image::ImageError) -> Self {
        IOError::Image(other)
    }
}

#[cfg(feature = "3d-io")]
impl From<bincode::Error> for IOError {
    fn from(other: bincode::Error) -> Self {
        IOError::Bincode(other)
    }
}

#[cfg(feature = "obj-io")]
impl From<wavefront_obj::ParseError> for IOError {
    fn from(other: wavefront_obj::ParseError) -> Self {
        IOError::Obj(other)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for IOError {
    fn from(other: std::io::Error) -> Self {
        IOError::IO(other)
    }
}
