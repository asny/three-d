//#![warn(clippy::all)]
//#![warn(missing_docs)]
//!
//! A 3D renderer which enables out-of-the-box build to both desktop and web with the same code.
//! See the [README](https://crates.io/crates/three-d) for more information and
//! the [examples](https://github.com/asny/three-d/tree/master/examples) for how to use it.
//!

pub mod context;

pub mod core;
#[doc(inline)]
pub use crate::core::*;

pub mod renderer;
#[doc(inline)]
pub use renderer::*;

pub mod window;
#[doc(inline)]
pub use window::*;
