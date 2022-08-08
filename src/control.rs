//!
//! Contain a [CameraControl] struct that can be easily customized as well as a set of default camera controls.
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
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with the device pixel ratio.
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
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with the device pixel ratio.
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
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with the device pixel ratio.
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
        /// The screen position in logical pixels, to get it in physical pixels, multiply it with the device pixel ratio.
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

#[cfg(feature = "egui")]
pub use egui_conversions::*;

#[cfg(feature = "egui")]
mod egui_conversions {
    use super::*;
    impl FrameInput {
        pub fn from_egui_input(
            context: &Context,
            first_frame: bool,
            input: egui::RawInput,
        ) -> Self {
            let modifiers = input.modifiers.into();
            let mut events = Vec::new();
            /*egui::Event::ModifiersChange { modifiers } => {
                modifiers = modifiers;
                events.push(Event::ModifiersChange { modifiers })
            }*/

            for event in input.events.into_iter() {
                match event {
                    egui::Event::Key {
                        key,
                        modifiers,
                        pressed,
                    } => {
                        events.push(if pressed {
                            Event::KeyPress {
                                kind: key.into(),
                                modifiers: modifiers.into(),
                                handled: false,
                            }
                        } else {
                            Event::KeyRelease {
                                kind: key.into(),
                                modifiers: modifiers.into(),
                                handled: false,
                            }
                        });
                    }
                    egui::Event::PointerButton {
                        button,
                        pos,
                        modifiers,
                        pressed,
                    } => events.push(if pressed {
                        Event::MousePress {
                            position: (pos.x as f64, pos.y as f64),
                            button: button.into(),
                            modifiers: modifiers.into(),
                            handled: false,
                        }
                    } else {
                        Event::MouseRelease {
                            position: (pos.x as f64, pos.y as f64),
                            button: button.into(),
                            modifiers: modifiers.into(),
                            handled: false,
                        }
                    }),
                    egui::Event::PointerMoved(pos) => events.push(Event::MouseMotion {
                        modifiers,
                        delta: (0.0, 0.0), // TODO
                        button: None,      // TODO
                        position: (pos.x as f64, pos.y as f64),
                        handled: false,
                    }),
                    egui::Event::Text(text) => events.push(Event::Text(text)),
                    egui::Event::PointerGone => events.push(Event::MouseLeave),
                    egui::Event::Scroll(delta) => events.push(Event::MouseWheel {
                        delta: (delta.x as f64, delta.y as f64),
                        handled: false,
                        modifiers,
                        position: (0.0, 0.0), // TODO
                    }),
                    _ => {}
                };
            }
            let device_pixel_ratio = input.pixels_per_point.map(|v| v as f64).unwrap_or(1.0);
            let window_width = input.screen_rect.map(|r| r.max.x as f64).unwrap_or(4096.0)
                - input.screen_rect.map(|r| r.min.x as f64).unwrap_or(0.0);
            let window_height = input.screen_rect.map(|r| r.max.y as f64).unwrap_or(4096.0)
                - input.screen_rect.map(|r| r.min.y as f64).unwrap_or(0.0);
            Self {
                context: context.clone(),
                viewport: Viewport {
                    x: (input.screen_rect.map(|r| r.min.x as f64).unwrap_or(0.0)
                        * device_pixel_ratio) as i32,
                    y: (input.screen_rect.map(|r| r.min.x as f64).unwrap_or(0.0)
                        * device_pixel_ratio) as i32,
                    width: (window_width * device_pixel_ratio) as u32,
                    height: (window_height * device_pixel_ratio) as u32,
                },
                window_width: window_width as u32,
                window_height: window_height as u32,
                device_pixel_ratio,
                accumulated_time: input.time.unwrap_or(0.0) * 1000.0,
                events,
                first_frame,
                elapsed_time: input.predicted_dt as f64,
            }
        }
    }

    impl From<&FrameInput> for egui::RawInput {
        fn from(frame_input: &FrameInput) -> Self {
            let mut egui_modifiers = egui::Modifiers::default();
            let events = frame_input
                .events
                .iter()
                .filter_map(|event| match event {
                    Event::KeyPress {
                        kind,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::Key {
                                key: kind.into(),
                                pressed: true,
                                modifiers: modifiers.into(),
                            })
                        } else {
                            None
                        }
                    }
                    Event::KeyRelease {
                        kind,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::Key {
                                key: kind.into(),
                                pressed: false,
                                modifiers: modifiers.into(),
                            })
                        } else {
                            None
                        }
                    }
                    Event::MousePress {
                        button,
                        position,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::PointerButton {
                                pos: egui::Pos2 {
                                    x: position.0 as f32,
                                    y: position.1 as f32,
                                },
                                button: button.into(),
                                pressed: true,
                                modifiers: modifiers.into(),
                            })
                        } else {
                            None
                        }
                    }
                    Event::MouseRelease {
                        button,
                        position,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::PointerButton {
                                pos: egui::Pos2 {
                                    x: position.0 as f32,
                                    y: position.1 as f32,
                                },
                                button: button.into(),
                                pressed: false,
                                modifiers: modifiers.into(),
                            })
                        } else {
                            None
                        }
                    }
                    Event::MouseMotion {
                        position, handled, ..
                    } => {
                        if !handled {
                            Some(egui::Event::PointerMoved(egui::Pos2 {
                                x: position.0 as f32,
                                y: position.1 as f32,
                            }))
                        } else {
                            None
                        }
                    }
                    Event::Text(text) => Some(egui::Event::Text(text.clone())),
                    Event::MouseLeave => Some(egui::Event::PointerGone),
                    Event::MouseWheel { delta, handled, .. } => {
                        if !handled {
                            Some(egui::Event::Scroll(egui::Vec2::new(
                                delta.0 as f32,
                                delta.1 as f32,
                            )))
                        } else {
                            None
                        }
                    }
                    Event::ModifiersChange { modifiers } => {
                        egui_modifiers = modifiers.into();
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    Default::default(),
                    egui::Vec2 {
                        x: frame_input.window_width as f32,
                        y: frame_input.window_height as f32,
                    },
                )),
                pixels_per_point: Some(frame_input.device_pixel_ratio as f32),
                time: Some(frame_input.accumulated_time * 0.001),
                modifiers: egui_modifiers,
                events,
                ..Default::default()
            }
        }
    }

    impl From<egui::Key> for Key {
        fn from(key: egui::Key) -> Self {
            use egui::Key::*;
            match key {
                ArrowDown => Key::ArrowDown,
                ArrowLeft => Key::ArrowLeft,
                ArrowRight => Key::ArrowRight,
                ArrowUp => Key::ArrowUp,
                Escape => Key::Escape,
                Tab => Key::Tab,
                Backspace => Key::Backspace,
                Enter => Key::Enter,
                Space => Key::Space,
                Insert => Key::Insert,
                Delete => Key::Delete,
                Home => Key::Home,
                End => Key::End,
                PageUp => Key::PageUp,
                PageDown => Key::PageDown,
                Num0 => Key::Num0,
                Num1 => Key::Num1,
                Num2 => Key::Num2,
                Num3 => Key::Num3,
                Num4 => Key::Num4,
                Num5 => Key::Num5,
                Num6 => Key::Num6,
                Num7 => Key::Num7,
                Num8 => Key::Num8,
                Num9 => Key::Num9,
                A => Key::A,
                B => Key::B,
                C => Key::C,
                D => Key::D,
                E => Key::E,
                F => Key::F,
                G => Key::G,
                H => Key::H,
                I => Key::I,
                J => Key::J,
                K => Key::K,
                L => Key::L,
                M => Key::M,
                N => Key::N,
                O => Key::O,
                P => Key::P,
                Q => Key::Q,
                R => Key::R,
                S => Key::S,
                T => Key::T,
                U => Key::U,
                V => Key::V,
                W => Key::W,
                X => Key::X,
                Y => Key::Y,
                Z => Key::Z,
            }
        }
    }

    impl From<&Key> for egui::Key {
        fn from(key: &Key) -> Self {
            use super::Key::*;
            use egui::Key;
            match key {
                ArrowDown => Key::ArrowDown,
                ArrowLeft => Key::ArrowLeft,
                ArrowRight => Key::ArrowRight,
                ArrowUp => Key::ArrowUp,
                Escape => Key::Escape,
                Tab => Key::Tab,
                Backspace => Key::Backspace,
                Enter => Key::Enter,
                Space => Key::Space,
                Insert => Key::Insert,
                Delete => Key::Delete,
                Home => Key::Home,
                End => Key::End,
                PageUp => Key::PageUp,
                PageDown => Key::PageDown,
                Num0 => Key::Num0,
                Num1 => Key::Num1,
                Num2 => Key::Num2,
                Num3 => Key::Num3,
                Num4 => Key::Num4,
                Num5 => Key::Num5,
                Num6 => Key::Num6,
                Num7 => Key::Num7,
                Num8 => Key::Num8,
                Num9 => Key::Num9,
                A => Key::A,
                B => Key::B,
                C => Key::C,
                D => Key::D,
                E => Key::E,
                F => Key::F,
                G => Key::G,
                H => Key::H,
                I => Key::I,
                J => Key::J,
                K => Key::K,
                L => Key::L,
                M => Key::M,
                N => Key::N,
                O => Key::O,
                P => Key::P,
                Q => Key::Q,
                R => Key::R,
                S => Key::S,
                T => Key::T,
                U => Key::U,
                V => Key::V,
                W => Key::W,
                X => Key::X,
                Y => Key::Y,
                Z => Key::Z,
            }
        }
    }

    impl From<egui::Modifiers> for Modifiers {
        fn from(modifiers: egui::Modifiers) -> Self {
            Self {
                alt: modifiers.alt,
                ctrl: modifiers.ctrl,
                shift: modifiers.shift,
                command: modifiers.command || modifiers.mac_cmd,
            }
        }
    }

    impl From<&Modifiers> for egui::Modifiers {
        fn from(modifiers: &Modifiers) -> Self {
            Self {
                alt: modifiers.alt,
                ctrl: modifiers.ctrl,
                shift: modifiers.shift,
                command: modifiers.command,
                mac_cmd: cfg!(target_os = "macos") && modifiers.command,
            }
        }
    }

    impl From<egui::PointerButton> for MouseButton {
        fn from(button: egui::PointerButton) -> Self {
            match button {
                egui::PointerButton::Primary => MouseButton::Left,
                egui::PointerButton::Secondary => MouseButton::Right,
                egui::PointerButton::Middle => MouseButton::Middle,
            }
        }
    }

    impl From<&MouseButton> for egui::PointerButton {
        fn from(button: &MouseButton) -> Self {
            match button {
                MouseButton::Left => egui::PointerButton::Primary,
                MouseButton::Right => egui::PointerButton::Secondary,
                MouseButton::Middle => egui::PointerButton::Middle,
            }
        }
    }
}
