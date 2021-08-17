#![warn(missing_docs)]
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

pub(crate) use crate::Result;
use thiserror::Error;
///
/// Error from the [io](crate::io) module.
///
#[derive(Error, Debug)]
pub enum IOError {
    /// An image parsing error.
    #[cfg(feature = "image-io")]
    #[error("error while parsing an image file")]
    Image(#[from] image::ImageError),
    /// A .3d parsing error.
    #[cfg(feature = "3d-io")]
    #[error("error while parsing a .3d file")]
    ThreeD(#[from] bincode::Error),
    /// A .obj parsing error.
    #[cfg(feature = "obj-io")]
    #[error("error while parsing an .obj file")]
    Obj(#[from] wavefront_obj::ParseError),
    /// A .gltf parsing error.
    #[cfg(feature = "gltf-io")]
    #[error("error while parsing a .gltf file")]
    Gltf(#[from] ::gltf::Error),
    /// A .gltf parsing error.
    #[cfg(feature = "gltf-io")]
    #[error("the .gltf file contain corrupt buffer data")]
    GltfCorruptData,
    /// A .gltf parsing error.
    #[cfg(feature = "gltf-io")]
    #[error("the .gltf file contain missing buffer data")]
    GltfMissingData,
    /// An loading error.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("error while loading a file")]
    Load(#[from] std::io::Error),
    /// An error when the resource was not loaded.
    #[error("tried to use {0} which was not loaded")]
    NotLoaded(String),
}

#[cfg(feature = "image-io")]
pub(crate) fn image_from_bytes(bytes: &[u8]) -> Result<crate::core::CPUTexture<u8>> {
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
