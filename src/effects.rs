
//!
//! A customizable effect applied to each pixel of a render target, for example fog or anti-aliasing.
//!

#[doc(hidden)]
pub mod image_effect;
#[doc(inline)]
pub use image_effect::*;

#[doc(hidden)]
pub mod fog;
#[doc(inline)]
pub use crate::fog::*;

#[doc(hidden)]
pub mod fxaa;
#[doc(inline)]
pub use crate::fxaa::*;