//!
//! Default windows for easy setup and event handling.
//! Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop
//! and canvas using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for web, but
//! can be replaced by any other window with similar functionality. Also contains camera control utilities.
//!

mod settings;
#[doc(inline)]
pub use settings::*;

pub mod control;
pub use control::*;

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

/// Type of mouse button.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum MouseButton {
    /// Left mouse button or one finger on touch.
    Left,
    /// Left mouse button or two fingers on touch.
    Right,
    /// Middle mouse button.
    Middle,
}

/// An input event (from mouse, keyboard or similar).
#[derive(Clone, Debug)]
pub enum Event {
    /// Fired when a button is pressed or the screen is touched.
    MousePress {
        /// Type of button
        button: MouseButton,
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with [FrameInput::device_pixel_ratio].
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: (f64, f64),
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired when a button is released or the screen is stopped being touched.
    MouseRelease {
        /// Type of button
        button: MouseButton,
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with [FrameInput::device_pixel_ratio].
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: (f64, f64),
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired continuously when the mouse or a finger on the screen is moved.
    MouseMotion {
        /// Type of button if a button is pressed.
        button: Option<MouseButton>,
        /// The relative movement of the mouse/finger since last [Event::MouseMotion] event.
        delta: (f64, f64),
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with [FrameInput::device_pixel_ratio].
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: (f64, f64),
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired continuously when the mouse wheel or equivalent is applied.
    MouseWheel {
        /// The relative scrolling since the last [Event::MouseWheel] event.
        delta: (f64, f64),
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with [FrameInput::device_pixel_ratio].
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: (f64, f64),
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired when the mouse enters the window.
    MouseEnter,
    /// Fired when the mouse leaves the window.
    MouseLeave,
    /// Fired when a key is pressed.
    KeyPress {
        /// The type of key.
        kind: Key,
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired when a key is released.
    KeyRelease {
        /// The type of key.
        kind: Key,
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired when the modifiers change.
    ModifiersChange {
        /// The state of modifiers after the change.
        modifiers: Modifiers,
    },
    /// Fires when some text has been written.
    Text(String),
}

/// Keyboard key input.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Key {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    /// Either from the main row or from the numpad.
    Num0,
    /// Either from the main row or from the numpad.
    Num1,
    /// Either from the main row or from the numpad.
    Num2,
    /// Either from the main row or from the numpad.
    Num3,
    /// Either from the main row or from the numpad.
    Num4,
    /// Either from the main row or from the numpad.
    Num5,
    /// Either from the main row or from the numpad.
    Num6,
    /// Either from the main row or from the numpad.
    Num7,
    /// Either from the main row or from the numpad.
    Num8,
    /// Either from the main row or from the numpad.
    Num9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

/// State of modifiers (alt, ctrl, shift and command).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Modifiers {
    /// Either of the alt keys are down (option ⌥ on Mac).
    pub alt: bool,
    /// Either of the control keys are down.
    /// When checking for keyboard shortcuts, consider using [`Self::command`] instead.
    pub ctrl: bool,
    /// Either of the shift keys are down.
    pub shift: bool,
    /// On Windows and Linux, set this to the same value as `ctrl`.
    /// On Mac, this should be set whenever one of the ⌘ Command keys are down.
    pub command: bool,
}

use crate::core::*;

///
/// Input from the window to the rendering (and whatever else needs it) each frame.
///
#[derive(Clone, Debug)]
pub struct FrameInput {
    /// A list of [events](crate::Event) which has occurred since last frame.
    pub events: Vec<Event>,

    /// Milliseconds since last frame.
    pub elapsed_time: f64,

    /// Milliseconds accumulated time since start.
    pub accumulated_time: f64,

    /// Viewport of the window in physical pixels (the size of the screen [RenderTarget] which is returned from [FrameInput::screen]).
    pub viewport: Viewport,

    /// Width of the window in logical pixels.
    pub window_width: u32,

    /// Height of the window in logical pixels.
    pub window_height: u32,

    /// Number of physical pixels for each logical pixel.
    pub device_pixel_ratio: f64,

    /// Whether or not this is the first frame.
    pub first_frame: bool,

    /// The graphics context for the window.
    pub context: Context,
}

impl FrameInput {
    ///
    /// Returns the screen render target, which is used for drawing to the screen, for this window.
    /// Same as
    ///
    /// ```notrust
    /// RenderTarget::screen(&frame_input.context, frame_input.viewport.width, frame_input.viewport.height)
    /// ```
    ///
    pub fn screen(&self) -> RenderTarget {
        RenderTarget::screen(&self.context, self.viewport.width, self.viewport.height)
    }
}

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
