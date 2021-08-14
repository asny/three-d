//!
//! Default windows for easy setup and event handling.
//! Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop
//! and canvas using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for web, but
//! can be replaced by any other window with similar functionality. Also contains camera control utilities.
//!

#[cfg(any(feature = "glutin-window", feature = "canvas"))]
mod settings;
#[doc(inline)]
#[cfg(any(feature = "glutin-window", feature = "canvas"))]
pub use settings::*;

pub mod control;
pub use control::*;

#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
mod glutin_window;
#[doc(inline)]
#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
pub use glutin_window::*;

#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
mod canvas;
#[doc(inline)]
#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
pub use canvas::*;

/// Type of mouse button.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// An input event (from mouse, keyboard or similar).
#[derive(Clone, Debug)]
pub enum Event {
    MousePress {
        button: MouseButton,
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool,
    },
    MouseRelease {
        button: MouseButton,
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool,
    },
    MouseMotion {
        button: Option<MouseButton>,
        delta: (f64, f64),
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool,
    },
    MouseWheel {
        delta: (f64, f64),
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool,
    },
    MouseEnter,
    MouseLeave,
    KeyPress {
        kind: Key,
        modifiers: Modifiers,
        handled: bool,
    },
    KeyRelease {
        kind: Key,
        modifiers: Modifiers,
        handled: bool,
    },
    ModifiersChange {
        modifiers: Modifiers,
    },
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

    /// Viewport of the window in physical pixels (the size of the [screen](crate::Screen)).
    pub viewport: crate::core::Viewport,

    /// Width of the window in logical pixels.
    pub window_width: u32,

    /// Height of the window in logical pixels.
    pub window_height: u32,

    /// Number of physical pixels for each logical pixel.
    pub device_pixel_ratio: f64,

    /// Whether or not this is the first frame.
    pub first_frame: bool,
}

///
/// Output from the rendering to the window each frame.
///
#[derive(Clone, Debug)]
pub struct FrameOutput {
    ///
    /// Closes the window and stops the renderloop if this is true.
    /// Only relevant on desktop, ignored on web.
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
    /// Takes a screenshot if this is set to some path and saves it at the given location.
    /// Only works on desktop, will be ignored on web.
    ///
    pub screenshot: Option<std::path::PathBuf>,

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
            screenshot: None,
            wait_next_event: false,
        }
    }
}
