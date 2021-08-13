//!
//! Contains functionality to load any type of asset runtime on both desktop and web as well as parsers for different image and 3D model formats.
//! Also includes functionality to save data which is limited to desktop.
//!

mod loader;
#[doc(inline)]
pub use loader::*;

mod parser;
#[doc(inline)]
pub use parser::*;

#[cfg(not(target_arch = "wasm32"))]
mod saver;
#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use saver::*;

///
/// Error message from the [core](crate::io) module.
///
#[derive(Debug)]
pub enum IOError {
    /// An image error.
    #[cfg(feature = "image-io")]
    Image(image::ImageError),
    /// A .3d parsing error.
    #[cfg(feature = "3d-io")]
    Bincode(bincode::Error),
    /// A .obj parsing error.
    #[cfg(feature = "obj-io")]
    Obj(wavefront_obj::ParseError),
    /// A .gltf parsing error.
    #[cfg(feature = "gltf-io")]
    Gltf(::gltf::Error),
    /// An IO error.
    #[cfg(not(target_arch = "wasm32"))]
    IO(std::io::Error),
    /// A loading error.
    FailedToLoad {
        /// Error message.
        message: String,
    },
    /// A saving error.
    FailedToSave {
        /// Error message.
        message: String,
    },
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

#[cfg(feature = "gltf-io")]
impl From<::gltf::Error> for IOError {
    fn from(other: ::gltf::Error) -> Self {
        IOError::Gltf(other)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for IOError {
    fn from(other: std::io::Error) -> Self {
        IOError::IO(other)
    }
}

#[cfg(feature = "image-io")]
pub(crate) fn image_from_bytes(bytes: &[u8]) -> Result<crate::core::CPUTexture<u8>, IOError> {
    use crate::core::*;
    use image::DynamicImage;
    use image::GenericImageView;
    let img = image::load_from_memory(bytes)?;
    let format = match img {
        DynamicImage::ImageLuma8(_) => Format::R,
        DynamicImage::ImageLumaA8(_) => Format::RG,
        DynamicImage::ImageRgb8(_) => Format::RGB,
        DynamicImage::ImageRgba8(_) => Format::RGBA,
        DynamicImage::ImageBgr8(_) => unimplemented!(),
        DynamicImage::ImageBgra8(_) => unimplemented!(),
        DynamicImage::ImageLuma16(_) => unimplemented!(),
        DynamicImage::ImageLumaA16(_) => unimplemented!(),
        DynamicImage::ImageRgb16(_) => unimplemented!(),
        DynamicImage::ImageRgba16(_) => unimplemented!(),
    };

    Ok(CPUTexture {
        data: img.to_bytes(),
        width: img.width(),
        height: img.height(),
        format,
        ..Default::default()
    })
}
