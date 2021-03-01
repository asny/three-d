
//!
//! Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL/WebGL2 graphics APIs.
//! Can be used in combination with more high-level features or avoided entirely.
//!

// GL
#[cfg(not(target_arch = "wasm32"))]
pub mod ogl;

#[cfg(not(target_arch = "wasm32"))]
pub use ogl::*;

// WEBGL
#[cfg(target_arch = "wasm32")]
pub mod wgl2;

#[cfg(target_arch = "wasm32")]
pub use wgl2::*;