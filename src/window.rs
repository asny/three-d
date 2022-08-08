//!
//! Default windows for easy setup and event handling.
//! Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop
//! and canvas using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for web, but
//! can be replaced by any other window with similar functionality. Also contains camera control utilities.
//!

mod settings;
#[doc(inline)]
pub use settings::*;

#[cfg(not(target_arch = "wasm32"))]
mod glutin_window;
#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use glutin_window::*;

#[cfg(not(target_arch = "wasm32"))]
mod headless;
#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use headless::*;

#[cfg(target_arch = "wasm32")]
mod canvas;
#[doc(inline)]
#[cfg(target_arch = "wasm32")]
pub use canvas::*;

use thiserror::Error;
///
/// Error in the [window](crate::window) module.
///
#[cfg(not(target_arch = "wasm32"))]
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WindowError {
    #[error("failed creating a new window")]
    WindowCreation(#[from] glutin::CreationError),
    #[error("failed creating a new context")]
    ContextCreation(#[from] glutin::ContextError),
    #[error("error in three-d")]
    ThreeDError(#[from] CoreError),
    #[error("the number of MSAA samples must be a power of two")]
    InvalidNumberOfMSAASamples,
}

///
/// Error in the [window](crate::window) module.
///
#[cfg(target_arch = "wasm32")]
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WindowError {
    #[error("failed creating a new window")]
    WindowCreation,
    #[error("unable to get document from canvas")]
    DocumentMissing,
    #[error("unable to convert canvas to html canvas: {0}")]
    CanvasConvertFailed(String),
    #[error("unable to get webgl2 context for the given canvas, maybe the browser doesn't support WebGL2{0}")]
    WebGL2NotSupported(String),
    #[error("unable to get EXT_color_buffer_float extension for the given canvas, maybe the browser doesn't support EXT_color_buffer_float: {0}")]
    ColorBufferFloatNotSupported(String),
    #[error("unable to get OES_texture_float extension for the given canvas, maybe the browser doesn't support OES_texture_float: {0}")]
    OESTextureFloatNotSupported(String),
    #[error("error in three-d")]
    ThreeDError(#[from] CoreError),
}

use crate::control::*;
use crate::core::*;

///
/// Output from the rendering to the window each frame.
///
#[derive(Clone, Debug)]
pub struct FrameOutput {
    ///
    /// If this is true:
    /// - On desktop, the window is closed and the renderloop is stopped.
    /// - On web, the render loop is stopped, the event handlers are removed and the `Window` dropped. Note that the canvas is not removed.
    ///
    pub exit: bool,

    ///
    /// Swaps the back and front buffer if this is true.
    /// Set this to true if something have been rendered this frame and you want to display it.
    /// Set it to false if nothing have been rendered this frame, for example if nothing has changed,
    /// and you want to reuse the image from an old frame.
    /// Currently ignored on web, since it does not use double buffering.
    ///
    pub swap_buffers: bool,

    ///
    /// Whether to stop the render loop until next event.
    ///
    pub wait_next_event: bool,
}

impl Default for FrameOutput {
    fn default() -> Self {
        Self {
            exit: false,
            swap_buffers: true,
            wait_next_event: false,
        }
    }
}
