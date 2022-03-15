#![allow(missing_docs)]

//!
//! Low-level graphics abstraction layer which maps one-to-one with the OpenGL graphics API on desktop
//! and WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate on web.
//! Use this if you want to have complete control of a feature but be aware that there are no safety checks.
//!

pub use glow;
pub use glow::Context as GlContext;
pub use glow::HasContext;
