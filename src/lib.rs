//#![warn(clippy::all)]
//#![warn(missing_docs)]
//!
//! A 3D renderer which enables out-of-the-box build to both desktop and web with the same code.
//! See the [README](https://crates.io/crates/three-d/0.8.0) for more information and
//! the [examples](https://github.com/asny/three-d/tree/0.8/examples) for how to use it.
//!

pub mod context;

pub mod core;

pub mod renderer;
pub use renderer::*;

pub mod window;
pub use window::*;

pub mod io;
pub use io::*;

mod gui;
#[doc(inline)]
pub use gui::*;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
