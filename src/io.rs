
//!
//! Contain a [loader](crate::Loader) for loading any type of asset runtime on both desktop and web
//! as well as parsers for 3D and image files and a [saver](crate::Saver) for saving.
//!

#[doc(hidden)]
#[cfg(any(feature = "3d-io", feature = "obj-io", feature = "image-io"))]
pub mod loader;
#[doc(inline)]
#[cfg(any(feature = "3d-io", feature = "obj-io", feature = "image-io"))]
pub use loader::*;

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod saver;
#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use saver::*;

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
