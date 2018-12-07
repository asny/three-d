pub extern crate gl;
pub extern crate geo_proc;
extern crate image;

#[macro_export]
macro_rules! att {
    ($( $name: expr => ($data: expr, $no_components: expr)),*) => {{
         let mut vec = Vec::new();
         $( vec.push(geo_proc::mesh::Attribute::new($name, $no_components, $data)); )*
         vec
    }}
}

pub mod core;
pub mod objects;
mod loader;

pub mod traits;
pub mod light;
pub mod screen;

pub mod eventhandler;
pub mod camerahandler;
pub mod camera;
pub mod pipeline;
pub mod renderer;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

#[cfg(target_os = "emscripten")]
mod emscripten;

pub use geo_proc::types::*;
pub use geo_proc::mesh as mesh;
pub use geo_proc::loader as mesh_loader;
pub use geo_proc::models as mesh_generator;

pub use crate::camera::Camera;

pub use crate::core::*;
pub use crate::texture::Texture;