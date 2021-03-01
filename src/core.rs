
//!
//! Modular abstractions of common graphics concepts such as GPU shader program, buffer (vertex buffer, uniform buffer, element buffer),
//! texture (2D texture, cube texture, ..) and render target.
//! They are higher level than [context](crate::context) but lower level than other features.
//!

pub use crate::context::Context;

pub mod render_states;
pub use render_states::*;

pub mod texture;
pub use texture::*;

pub mod element_buffer;
pub use element_buffer::*;

pub mod vertex_buffer;
pub use vertex_buffer::*;

pub mod uniform_buffer;
pub use uniform_buffer::*;

pub mod render_target;
pub use render_target::*;

pub mod program;
pub use program::*;

pub mod camera;
pub use camera::*;

pub mod cpu_mesh;
pub use crate::cpu_mesh::*;

pub mod cpu_material;
pub use crate::cpu_material::*;

pub mod cpu_texture;
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