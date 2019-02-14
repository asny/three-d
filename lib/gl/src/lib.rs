
// GL
#[cfg(target_arch = "x86_64")]
pub mod ogl;

#[cfg(target_arch = "x86_64")]
pub use crate::ogl::*;

// WEBGL
#[cfg(target_arch = "wasm32")]
pub mod wgl2;

#[cfg(target_arch = "wasm32")]
pub use crate::wgl2::*;