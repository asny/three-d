extern crate num_traits;
pub extern crate gl;
pub extern crate gust;
pub use gust::glm;
pub use gust::mesh;

pub mod core;
pub mod loader;

pub mod traits;
pub mod light;
pub mod input;

pub mod eventhandler;
pub mod camera;
pub mod screen;
pub mod scene;
pub mod renderer;

#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;

#[cfg(target_os = "emscripten")]
mod emscripten;