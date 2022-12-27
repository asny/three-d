//!
//! Window, event handling and context creation for easy setup.
//! * Can be avoided fully by setting up a window, event handling etc. and creating a [Context](crate::core::Context)
//! from a [glow](https://crates.io/crates/glow) OpenGL/WebGL context.
//! * If full control over the window and event handling, but not the context creation, is desired, use a [WindowedContext] or [HeadlessContext].
//! * Finally, for an easy setup, use [Window::new] or [Window::from_winit_window], the latter will provide full control over the creation of the window.
//!
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
