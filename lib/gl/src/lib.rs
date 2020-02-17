
// GL
#[cfg(feature = "desktop")]
pub mod ogl;

#[cfg(feature = "desktop")]
pub use crate::ogl::*;

// WEBGL
#[cfg(feature = "web")]
pub mod wgl2;

#[cfg(feature = "web")]
pub use crate::wgl2::*;