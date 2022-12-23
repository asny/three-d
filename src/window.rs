//!
//! Default windows for easy setup and event handling.
//! Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop
//! and canvas using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for web, but
//! can be replaced by any other window with similar functionality.
//!

#[cfg(feature = "window")]
#[cfg_attr(docsrs, doc(feature = "window"))]
mod winit_window;
#[cfg(feature = "window")]
pub use winit_window::*;

#[cfg(all(feature = "test", not(feature = "window"), not(target_arch = "wasm32")))]
mod test_window;
#[cfg(all(feature = "test", not(feature = "window"), not(target_arch = "wasm32")))]
pub use test_window::*;

#[cfg(all(feature = "headless", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(feature = "headless"))]
mod headless;
#[cfg(all(feature = "headless", not(target_arch = "wasm32")))]
pub use headless::*;
