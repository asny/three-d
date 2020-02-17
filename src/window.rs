

#[cfg(feature="desktop")]
pub mod frame_input;
#[cfg(feature="desktop")]
pub use frame_input::*;
#[cfg(feature="desktop")]
pub mod glutin_window;
#[cfg(feature="desktop")]
pub use crate::glutin_window::*;

#[cfg(feature="web")]
pub mod frame_input;
#[cfg(feature="web")]
pub use frame_input::*;
#[cfg(feature="web")]
pub mod canvas;
#[cfg(feature="web")]
pub use crate::canvas::*;