
pub use crate::context::Context;

pub mod math;
pub use math::*;

pub mod aabb;
pub use aabb::*;

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

pub mod viewport;
pub use crate::viewport::*;

pub mod cpu_mesh;
pub use crate::cpu_mesh::*;

pub mod cpu_material;
pub use crate::cpu_material::*;

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