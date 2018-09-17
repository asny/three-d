pub extern crate gl;
pub extern crate gust;

#[macro_export]
macro_rules! att {
    ($( $name: expr => ($data: expr, $no_components: expr)),*) => {{
         let mut vec = Vec::new();
         $( vec.push(gust::mesh::Attribute::new($name, $no_components, $data)); )*
         vec
    }}
}

pub mod core;
mod loader;

pub mod traits;
pub mod light;

pub mod eventhandler;
pub mod camerahandler;
pub mod camera;
pub mod pipeline;
pub mod scene;
pub mod renderer;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

#[cfg(target_os = "emscripten")]
mod emscripten;

pub use gust::types::*;
pub use gust::mesh as mesh;
pub use gust::mesh::Renderable;
pub use gust::loader as mesh_loader;
pub use gust::models as mesh_generator;

pub use core::*;
pub use texture::Texture;