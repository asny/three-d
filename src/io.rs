
//!
//! Loading any type of asset runtime on both desktop and web as well as parsers for .obj
//! (using the [wavefront-obj](https://crates.io/crates/wavefront_obj/main.rs) crate), .3d files (a custom format) and
//! most image formats (using the [image](https://crates.io/crates/image/main.rs) crate).
//!

#[cfg(any(feature = "3d-io", feature = "obj-io", feature = "image-io"))]
pub mod loader;
#[cfg(any(feature = "3d-io", feature = "obj-io", feature = "image-io"))]
pub use loader::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod saver;
#[cfg(not(target_arch = "wasm32"))]
pub use saver::*;

#[cfg(feature = "3d-io")]
pub mod threed;
#[cfg(feature = "3d-io")]
pub use threed::*;

#[cfg(feature = "obj-io")]
pub mod obj;
#[cfg(feature = "obj-io")]
pub use obj::*;

#[derive(Debug)]
pub enum Error {
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
impl From<image::ImageError> for Error {
    fn from(other: image::ImageError) -> Self {
        Error::Image(other)
    }
}

#[cfg(feature = "3d-io")]
impl From<bincode::Error> for Error {
    fn from(other: bincode::Error) -> Self {
        Error::Bincode(other)
    }
}

#[cfg(feature = "obj-io")]
impl From<wavefront_obj::ParseError> for Error {
    fn from(other: wavefront_obj::ParseError) -> Self {
        Error::Obj(other)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}
