extern crate num_traits;
pub extern crate gl;
pub extern crate gust;
pub use gust::glm;
pub use gust::mesh;

pub mod core;
pub mod loader;
pub mod model;
mod utility;

pub mod light;
pub mod material;
pub mod input;

pub mod eventhandler;
pub mod camera;
pub mod scene;
pub mod renderer;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

#[cfg(target_os = "emscripten")]
mod emscripten;