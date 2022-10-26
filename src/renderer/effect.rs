//!
//! A collection of image based effects, ie. effects applied to each pixel of a rendered image.
//!

mod fog;
#[doc(inline)]
pub use fog::*;

mod fxaa;
#[doc(inline)]
pub use fxaa::*;
