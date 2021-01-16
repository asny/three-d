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

pub mod cpu_mesh;
pub use crate::cpu_mesh::*;

pub mod gpu_mesh;
pub use crate::gpu_mesh::*;

pub mod cpu_material;
pub use crate::cpu_material::*;

pub mod light;
pub use crate::light::*;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCreateShader {shader_type: String, message: String},
    FailedToLinkProgram {message: String},
    FailedToFindAttribute {message: String},
    FailedToFindUniform {message: String},
    FailedToCreateFramebuffer {message: String},
    FailedToCreateTexture {message: String},
    FailedToUpdateBuffer {message: String},
    FailedToCreateMesh {message: String}
}