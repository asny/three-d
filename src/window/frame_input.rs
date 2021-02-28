
#[derive(Clone, Debug)]
pub struct FrameInput {
    pub events: Vec<Event>,
    pub elapsed_time: f64, // Milliseconds since last frame
    pub accumulated_time: f64, // Milliseconds accumulated time since start
    pub viewport: crate::Viewport, // Viewport of the window in physical pixels
    pub window_width: usize, // Width of the window in logical pixels
    pub window_height: usize, // Height of the window in logical pixels
    pub device_pixel_ratio: usize // Number of physical pixels for each logical pixel
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum State
{
    Pressed,
    Released
}

impl Default for State {
    fn default() -> Self {
        Self::Released
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Debug)]
pub enum Event
{
    MouseClick {
        state: State,
        button: MouseButton,
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool
    },
    MouseMotion {
        delta: (f64, f64),
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool
    },
    MouseWheel {
        delta: (f64, f64),
        position: (f64, f64),
        modifiers: Modifiers,
        handled: bool
    },
    MouseEnter,
    MouseLeave,
    Key {
        state: State,
        kind: Key,
        modifiers: Modifiers,
        handled: bool
    },
    ModifiersChange {
        modifiers: Modifiers
    },
    Text(String)
}

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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Modifiers {
    /// Either of the alt keys are down (option ⌥ on Mac).
    pub alt: State,
    /// Either of the control keys are down.
    /// When checking for keyboard shortcuts, consider using [`Self::command`] instead.
    pub ctrl: State,
    /// Either of the shift keys are down.
    pub shift: State,
    /// On Windows and Linux, set this to the same value as `ctrl`.
    /// On Mac, this should be set whenever one of the ⌘ Command keys are down.
    pub command: State,
}