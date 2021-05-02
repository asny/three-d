#![allow(missing_docs)]

//!
//! Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL graphics API on desktop
//! and WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate on web.
//! Can be used in combination with more high-level features or be ignored entirely.
//!

// GL
#[cfg(not(target_arch = "wasm32"))]
mod ogl;

#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use ogl::*;

// WEBGL
#[cfg(target_arch = "wasm32")]
mod wgl2;

#[doc(inline)]
#[cfg(target_arch = "wasm32")]
pub use wgl2::*;
