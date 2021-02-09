
pub struct FrameInput {
    pub events: Vec<Event>,
    pub elapsed_time: f64, // ms since last frame
    pub viewport: crate::Viewport,
    pub window_width: usize,
    pub window_height: usize
}

#[derive(Debug, Clone, PartialEq)]
pub enum State
{
    Pressed,
    Released
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event
{
    MouseClick {
        state: State,
        button: MouseButton,
        position: (f64, f64)
    },
    MouseMotion {
        /*Note: The 'delta' variable is not entirely accurate, especially on web. For better accuracy, use the 'position' variable instead.*/
        delta: (f64, f64),
        position: (f64, f64)
    },
    MouseWheel {
        delta: f64,
    },
    Key {
        state: State,
        kind: String
    },
}