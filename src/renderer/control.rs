//!
//! A collection of controls for example to control the camera.
//!

mod camera_control;
#[doc(inline)]
pub use camera_control::*;

mod orbit_control;
#[doc(inline)]
pub use orbit_control::*;

mod first_person_control;
#[doc(inline)]
pub use first_person_control::*;

mod fly_control;
#[doc(inline)]
pub use fly_control::*;

pub use three_d_asset::PixelPoint as PhysicalPoint;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LogicalPoint {
    pub x: f32,
    pub y: f32,
    pub(crate) scale_factor: f32,
    pub(crate) height: f32,
}

impl From<LogicalPoint> for PhysicalPoint {
    fn from(value: LogicalPoint) -> Self {
        Self::from(&value)
    }
}

impl From<&LogicalPoint> for PhysicalPoint {
    fn from(value: &LogicalPoint) -> Self {
        Self {
            x: value.x * value.scale_factor,
            y: value.height - value.y * value.scale_factor,
        }
    }
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
pub enum Event<T: 'static + Clone> {
    /// Fired when a button is pressed or the screen is touched.
    MousePress {
        /// Type of button
        button: MouseButton,
        /// The screen position in logical pixels.
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: LogicalPoint,
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired when a button is released or the screen is stopped being touched.
    MouseRelease {
        /// Type of button
        button: MouseButton,
        /// The screen position in logical pixels.
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: LogicalPoint,
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
        delta: (f32, f32),
        /// The screen position in logical pixels.
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: LogicalPoint,
        /// The state of modifiers.
        modifiers: Modifiers,
        /// Whether or not this event already have been handled.
        handled: bool,
    },
    /// Fired continuously when the mouse wheel or equivalent is applied.
    MouseWheel {
        /// The relative scrolling since the last [Event::MouseWheel] event.
        delta: (f32, f32),
        /// The screen position in logical pixels.
        /// The first value defines the position on the horizontal axis with zero being at the left border of the window
        /// and the second on the vertical axis with zero being at the top edge of the window.
        position: LogicalPoint,
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
    /// User created custom event.
    UserEvent(T),
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
