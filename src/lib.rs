
//!
//! A 3D renderer which enables out-of-the-box build to both desktop and web with the same code.
//! See the [README](https://crates.io/crates/three-d) for more information and
//! the [examples](https://github.com/asny/three-d/tree/0.6/examples) for how to use it.
//!

pub mod context;

pub mod math;
#[doc(inline)]
pub use crate::math::*;

pub mod definition;
#[doc(inline)]
pub use crate::definition::*;

pub mod core;
#[doc(inline)]
pub use crate::core::*;

pub mod camera;
#[doc(inline)]
pub use camera::*;

pub mod frame;
#[doc(inline)]
pub use frame::*;

pub mod object;
#[doc(inline)]
pub use crate::object::*;

pub mod effect;
#[doc(inline)]
pub use crate::effect::*;

pub mod light;
#[doc(inline)]
pub use crate::light::*;

pub mod io;
#[doc(inline)]
pub use crate::io::*;

#[cfg(feature = "phong-renderer")]
pub mod phong;
#[doc(inline)]
#[cfg(feature = "phong-renderer")]
pub use crate::phong::*;

pub mod window;
#[doc(inline)]
pub use window::*;

pub mod gui;
#[doc(inline)]
pub use gui::*;
