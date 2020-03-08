pub mod buffer;
pub mod program;
pub mod rendertarget;
pub(crate) mod shader;
pub mod state;
pub mod texture;
pub mod types;
pub mod camera;

pub use buffer::*;
pub use program::*;
pub use rendertarget::*;
pub use state::*;
pub use texture::*;
pub use types::*;
pub use camera::*;

pub type Gl = std::rc::Rc<gl::Gl>;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCreateShader {shader_type: String, message: String},
    FailedToCompileShader {shader_type: String, message: String},
    FailedToLinkProgram {message: String},
    FailedToCreateCString(std::ffi::NulError),
    FailedToFindPositions {message: String},
    FailedToFindAttribute {message: String},
    FailedToFindUniform {message: String},
    IO(std::io::Error),
    FailedToCreateFramebuffer {message: String},
    #[cfg(feature = "image-io")]
    Image(image::ImageError),
    FailedToCreateTexture {message: String},
    BufferUpdateFailed {message: String}
}

#[cfg(feature = "image-io")]
impl From<image::ImageError> for Error {
    fn from(other: image::ImageError) -> Self {
        Error::Image(other)
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(other: std::ffi::NulError) -> Self {
        Error::FailedToCreateCString(other)
    }
}