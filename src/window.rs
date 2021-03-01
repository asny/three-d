
//!
//! Default windows for easy setup and event handling. Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop and canvas for web.
//!

#[doc(hidden)]
pub mod frame_input;
#[doc(inline)]
pub use frame_input::*;

#[doc(hidden)]
#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
pub mod glutin_window;
#[doc(inline)]
#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
pub use crate::glutin_window::*;

#[doc(hidden)]
#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
pub mod canvas;
#[doc(inline)]
#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
pub use crate::canvas::*;