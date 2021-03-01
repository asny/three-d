
//!
//! Modular abstractions of common graphics concepts such as GPU shader program, buffer (vertex buffer, uniform buffer, element buffer),
//! texture (2D texture, cube texture, ..) and render target.
//! They are higher level than [context](crate::context) but lower level than other features.
//!

#[doc(inline)]
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

#[doc(hidden)]
pub mod camera;
#[doc(inline)]
pub use camera::*;

#[doc(hidden)]
pub mod cpu_mesh;
#[doc(inline)]
pub use crate::cpu_mesh::*;

#[doc(hidden)]
pub mod cpu_material;
#[doc(inline)]
pub use crate::cpu_material::*;

#[doc(hidden)]
pub mod cpu_texture;
#[doc(inline)]
pub use crate::cpu_texture::*;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCreateShader {shader_type: String, message: String},
    FailedToLinkProgram {message: String},
    FailedToFindAttribute {message: String},
    FailedToFindUniform {message: String},
    FailedToCreateFramebuffer {message: String},
    FailedToCopyFromRenderTarget {message: String},
    FailedToWriteToRenderTarget {message: String},
    FailedToCreateTexture {message: String},
    FailedToUpdateBuffer {message: String},
    FailedToCreateMesh {message: String}
}