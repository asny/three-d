//#![warn(clippy::all)]
//#![warn(missing_docs)]
//!
//! A 3D renderer which enables out-of-the-box build to both desktop and web with the same code.
//! See the [README](https://crates.io/crates/three-d) for more information and
//! the [examples](https://github.com/asny/three-d/tree/master/examples) for how to use it.
//!

pub mod context;

pub mod math;
#[doc(inline)]
pub use math::*;

pub mod definition;
#[doc(inline)]
pub use definition::*;

pub mod core;
#[doc(inline)]
pub use crate::core::*;

pub mod color;
#[doc(inline)]
pub use crate::color::*;

pub mod camera;
#[doc(inline)]
pub use camera::*;

pub mod frame;
#[doc(inline)]
pub use frame::*;

pub mod object;
#[doc(inline)]
pub use object::*;

pub mod effect;
#[doc(inline)]
pub use effect::*;

pub mod light;
#[doc(inline)]
pub use light::*;

pub mod function;
#[doc(inline)]
pub use function::*;

pub mod io;
#[doc(inline)]
pub use io::*;

pub mod shading;
#[doc(inline)]
pub use shading::*;

pub mod window;
#[doc(inline)]
pub use window::*;

pub mod gui;
#[doc(inline)]
pub use gui::*;
