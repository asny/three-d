
//!
//! A 3D renderer which enables out-of-the-box build to both desktop and web with the same code.
//! See the [README](https://crates.io/crates/three-d) for more information and
//! the [examples](https://github.com/asny/three-d/tree/0.5/examples) for how to use it.
//!

pub mod context;

pub mod math;
#[doc(hidden)]
pub use crate::math::*;

pub mod core;
#[doc(inline)]
pub use crate::core::*;

pub mod objects;
#[doc(inline)]
pub use crate::objects::*;

pub mod effects;
#[doc(inline)]
pub use crate::effects::*;

pub mod lights;
#[doc(inline)]
pub use crate::lights::*;

pub mod io;
#[doc(hidden)]
pub use crate::io::*;

#[cfg(feature = "phong-renderer")]
pub mod phong;
#[doc(inline)]
#[cfg(feature = "phong-renderer")]
pub use crate::phong::*;

pub mod window;
#[doc(hidden)]
pub use window::*;

pub mod gui;
#[doc(hidden)]
pub use gui::*;
