
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

#[cfg(any(feature = "glutin-window", feature = "canvas"))]
pub mod window;
#[doc(hidden)]
#[cfg(any(feature = "glutin-window", feature = "canvas"))]
pub use window::*;

pub mod gui;
#[doc(hidden)]
pub use gui::*;
