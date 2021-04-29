//!
//! Modular abstractions of common graphics concepts such as GPU shader program, buffer (vertex buffer, uniform buffer, element buffer),
//! texture (2D texture, cube texture, ..) and render target.
//! They are higher level than [context](crate::context) but lower level than other features.
//!

#[doc(hidden)]
pub use crate::context::Context;

#[doc(hidden)]
pub mod render_states;
#[doc(inline)]
pub use render_states::*;

#[doc(hidden)]
pub mod texture;
#[doc(inline)]
pub use texture::*;

#[doc(hidden)]
pub mod element_buffer;
#[doc(inline)]
pub use element_buffer::*;

#[doc(hidden)]
pub mod vertex_buffer;
#[doc(inline)]
pub use vertex_buffer::*;

#[doc(hidden)]
pub mod uniform_buffer;
#[doc(inline)]
pub use uniform_buffer::*;

#[doc(hidden)]
pub mod render_target;
#[doc(inline)]
pub use render_target::*;

#[doc(hidden)]
pub mod program;
#[doc(inline)]
pub use program::*;

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
