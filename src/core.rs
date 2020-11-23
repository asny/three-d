pub mod buffer;
pub mod program;
pub mod rendertarget;
pub mod state;
pub mod texture;
pub mod types;
pub mod camera;
pub mod image_effect;
pub mod aabb;

pub use crate::gl::Gl;
pub use crate::gl::consts;

pub use buffer::*;
pub use program::*;
pub use rendertarget::*;
pub use state::*;
pub use texture::*;
pub use types::*;
pub use camera::*;
pub use image_effect::*;
pub use aabb::*;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCreateShader {shader_type: String, message: String},
    FailedToLinkProgram {message: String},
    FailedToFindAttribute {message: String},
    FailedToFindUniform {message: String},
    IO(std::io::Error),
    FailedToCreateFramebuffer {message: String},
    #[cfg(feature = "image-io")]
    Image(image::ImageError),
    #[cfg(feature = "3d-io")]
    Bincode(bincode::Error),
    #[cfg(feature = "obj-io")]
    Obj(wavefront_obj::ParseError),
    FailedToLoad {message: String},
    FailedToCreateTexture {message: String},
    BufferUpdateFailed {message: String},
    FailedToCreateMesh {message: String}
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


impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}