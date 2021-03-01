
//!
//! Default windows for easy setup and event handling. Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop and canvas for web.
//!

pub mod frame_input;
pub use frame_input::*;

#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
pub mod glutin_window;
#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
pub use crate::glutin_window::*;

#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
pub mod canvas;
#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
pub use crate::canvas::*;