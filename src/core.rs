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

pub(crate) use crate::Result;
use thiserror::Error;
///
/// Error in the [core](crate::core) module.
///
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CoreError {
    #[error("failed creating a new shader")]
    ShaderCreation,
    #[error("failed compiling {0} shader: {1}")]
    ShaderCompilation(String, String),
    #[error("failed to link shader program: {0}")]
    ShaderLink(String),
    #[error("the uniform {0} is sent to the shader but never used")]
    UnusedUniform(String),
    #[error("the attribute {0} is sent to the shader but never used")]
    UnusedAttribute(String),
    #[error("failed creating a new render target")]
    RenderTargetCreation,
    #[error("cannot copy {0} from a {1} texture")]
    RenderTargetCopy(String, String),
    #[error("cannot read color from anything else but an RGBA texture")]
    ReadWrongFormat,
    #[error("failed creating a new texture")]
    TextureCreation,
    #[error("invalid size of texture data (got {0} pixels but expected {1} pixels)")]
    InvalidTextureLength(usize, usize),
    #[error("the render call requires the {0} vertex buffer which is missing on the given mesh")]
    MissingMeshBuffer(String),
    #[error("{0} buffer length must be divisible by 3 for the mesh `{1}`, actual count is {2}")]
    InvalidMeshBufferLength(String, String, usize),
    #[error("index buffer for the mesh `{0}` contains values larger than the length of the buffer which is {1}")]
    InvalidMeshIndexBuffer(String, usize),
    #[error("when indices unspecified, positions length of mesh `{0}` must be divisible by 9, actual count is {1}")]
    InvalidMeshPositionBuffer(String, usize),
    #[error("data for element at index {0} has length {1} but a length of {2} was expected")]
    InvalidUniformBufferElementLength(u32, usize, usize),
    #[error("the index {0} is outside the expected range [0, {1}]")]
    IndexOutOfRange(usize, usize),
    #[error("cannot take as input a negative minimum distance")]
    NegativeDistance,
    #[error("a minimum must be smaller than a maximum")]
    MinimumLargerThanMaximum,
}
