//!
//! Mid-level modular abstractions of common graphics concepts such as buffer, texture, program, render target and so on.
//! Can be combined with low-level calls in the `context` module as long as any graphics state changes are reset.
//!

pub use crate::context::Context;

pub mod buffer;
pub use buffer::*;

pub mod math;
pub use math::*;

pub mod texture;
pub use texture::*;

pub mod object;
pub use object::*;

pub mod render_states;
pub use render_states::*;

pub mod render_target;
pub use render_target::*;

pub mod material;
pub use material::*;

mod camera;
#[doc(inline)]
pub use camera::*;

mod image_effect;
#[doc(inline)]
pub use image_effect::*;

mod program;
#[doc(inline)]
pub use program::*;

mod aabb;
#[doc(inline)]
pub use aabb::*;

mod color;
#[doc(inline)]
pub use color::*;

mod viewport;
#[doc(inline)]
pub use viewport::*;

///
/// Error in some part of the render engine.
///
#[derive(Debug)]
pub enum Error {
    /// An error in a shader program.
    ProgramError {
        /// Error message
        message: String,
    },
    /// An error when using a render target.
    RenderTargetError {
        /// Error message
        message: String,
    },
    /// An error when using a texture.
    TextureError {
        /// Error message
        message: String,
    },
    /// An error when using a buffer.
    BufferError {
        /// Error message
        message: String,
    },
    /// An error when using a mesh.
    MeshError {
        /// Error message
        message: String,
    },
    /// An error when using a camera.
    CameraError {
        /// Error message
        message: String,
    },
}

impl Error {
    pub fn message(&self) -> &String {
        return match self {
            Error::ProgramError { message } => message,
            Error::RenderTargetError { message } => message,
            Error::TextureError { message } => message,
            Error::BufferError { message } => message,
            Error::MeshError { message } => message,
            Error::CameraError { message } => message,
        };
    }
}

fn check_data_length(
    width: u32,
    height: u32,
    depth: u32,
    format: Format,
    length: usize,
) -> Result<(), Error> {
    let expected_pixels = width as usize * height as usize * depth as usize;
    let actual_pixels = length / format.color_channel_count() as usize;

    if expected_pixels != actual_pixels {
        Err(Error::TextureError {
            message: format!(
                "Wrong size of data for the texture (got {} pixels but expected {} pixels)",
                actual_pixels, expected_pixels
            ),
        })?;
    }
    Ok(())
}
