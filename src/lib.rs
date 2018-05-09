extern crate num_traits;
pub extern crate gl;
pub extern crate gust;
pub use gust::glm;
pub use gust::mesh;

pub mod loader;
mod model;
mod buffer;
mod shader;
mod utility;

pub mod material;
pub mod program;
pub mod input;
pub mod state;

pub mod rendertarget;
pub mod texture;
pub mod eventhandler;
pub mod camera;
pub mod scene;
pub mod renderer;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

#[cfg(target_os = "emscripten")]
mod emscripten;