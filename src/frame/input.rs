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
    pub viewport: crate::Viewport,

    /// Width of the window in logical pixels.
    pub window_width: u32,

    /// Height of the window in logical pixels.
    pub window_height: u32,

    /// Number of physical pixels for each logical pixel.
    pub device_pixel_ratio: f64,

    /// Whether or not this is the first frame.
    pub first_frame: bool,
}

impl FrameInput {
    /// Has a keyboard quit event.
    pub fn has_key_quit(&self) -> bool {
        self.events.iter().any(Event::is_key_quit)
    }
}

/// State of a key or button click.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum State {
    Pressed,
    Released,
}

impl Default for State {
    fn default() -> Self {
        Self::Released
    }
}

/// Type of mouse button.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Mouse click event.
#[derive(Clone, Debug)]
pub struct MouseClickEvent {
    pub state: State,
    pub button: MouseButton,
    pub position: (f64, f64),
    pub modifiers: Modifiers,
    pub handled: bool,
}

/// Mouse motion event.
#[derive(Clone, Debug)]
pub struct MouseMotionEvent {
    pub delta: (f64, f64),
    pub position: (f64, f64),
    pub modifiers: Modifiers,
    pub handled: bool,
}

#[derive(Clone, Debug)]
pub struct MouseWheelEvent {
    pub delta: (f64, f64),
    pub position: (f64, f64),
    pub modifiers: Modifiers,
    pub handled: bool,
}

/// Keyboard event.
#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub state: State,
    pub kind: Key,
    pub modifiers: Modifiers,
    pub handled: bool,
}

impl KeyEvent {
    /// Is this a quit event (Cmd-Q on mac, Ctrl-Q elsewhere).
    pub fn is_quit(&self) -> bool {
        self.modifiers.command == State::Pressed && self.kind == Key::Q
    }
}

/// Keyboard modifiers changed.
#[derive(Clone, Debug)]
pub struct ModifiersChangeEvent {
    pub modifiers: Modifiers,
}

/// An input event (from mouse, keyboard or similar).
#[derive(Clone, Debug)]
pub enum Event {
    MouseClick(MouseClickEvent),
    MouseMotion(MouseMotionEvent),
    MouseWheel(MouseWheelEvent),
    MouseEnter,
    MouseLeave,
    Key(KeyEvent),
    ModifiersChange(ModifiersChangeEvent),
    Text(String),
}

impl Event {
    pub fn is_key_quit(&self) -> bool {
        match self {
            Event::Key(e) => e.is_quit(),
            _ => false,
        }
    }
}

/// Keyboard key input.
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
