use crate::control::*;
use crate::core::*;
use egui_glow::Painter;
use std::cell::RefCell;

#[doc(hidden)]
pub use egui;

///
/// Integration of [egui](https://crates.io/crates/egui), an immediate mode GUI.
///
pub struct GUI {
    painter: RefCell<Painter>,
    egui_context: egui::Context,
    output: RefCell<Option<egui::FullOutput>>,
    viewport: Viewport,
    modifiers: Modifiers,
}

impl GUI {
    ///
    /// Creates a new GUI from a mid-level [Context].
    ///
    pub fn new(context: &Context) -> Self {
        use std::ops::Deref;
        Self::from_gl_context(context.deref().clone())
    }

    ///
    /// Creates a new GUI from a low-level graphics [Context](crate::context::Context).
    ///
    pub fn from_gl_context(context: std::sync::Arc<crate::context::Context>) -> Self {
        GUI {
            egui_context: egui::Context::default(),
            painter: RefCell::new(Painter::new(context, "", None, true).unwrap()),
            output: RefCell::new(None),
            viewport: Viewport::new_at_origo(1, 1),
            modifiers: Modifiers::default(),
        }
    }

    ///
    /// Get the egui context.
    ///
    pub fn context(&self) -> &egui::Context {
        &self.egui_context
    }

    ///
    /// Initialises a new frame of the GUI and handles events.
    /// Construct the GUI (Add panels, widgets etc.) using the [egui::Context] in the callback function.
    /// This function returns whether or not the GUI has changed, ie. if it consumes any events, and therefore needs to be rendered again.
    ///
    pub fn update(
        &mut self,
        events: &mut [Event],
        accumulated_time_in_ms: f64,
        viewport: Viewport,
        device_pixel_ratio: f32,
        callback: impl FnMut(&mut egui::Ui),
    ) -> bool {
        self.egui_context.set_pixels_per_point(device_pixel_ratio);
        self.viewport = viewport;
        let egui_input = egui::RawInput {
            screen_rect: Some(egui::Rect {
                min: egui::Pos2 {
                    x: viewport.x as f32 / device_pixel_ratio,
                    y: viewport.y as f32 / device_pixel_ratio,
                },
                max: egui::Pos2 {
                    x: viewport.x as f32 / device_pixel_ratio
                        + viewport.width as f32 / device_pixel_ratio,
                    y: viewport.y as f32 / device_pixel_ratio
                        + viewport.height as f32 / device_pixel_ratio,
                },
            }),
            time: Some(accumulated_time_in_ms * 0.001),
            modifiers: (&self.modifiers).into(),
            events: events
                .iter()
                .filter_map(|event| match event {
                    Event::KeyPress {
                        kind,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            translate_key(kind).map(|key| egui::Event::Key {
                                key,
                                pressed: true,
                                modifiers: modifiers.into(),
                                repeat: false,
                                physical_key: None,
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
                            translate_key(kind).map(|key| egui::Event::Key {
                                key,
                                pressed: false,
                                modifiers: modifiers.into(),
                                repeat: false,
                                physical_key: None,
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
                                    x: position.x / device_pixel_ratio,
                                    y: (viewport.height as f32 - position.y) / device_pixel_ratio,
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
                                    x: position.x / device_pixel_ratio,
                                    y: (viewport.height as f32 - position.y) / device_pixel_ratio,
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
                                x: position.x / device_pixel_ratio,
                                y: (viewport.height as f32 - position.y) / device_pixel_ratio,
                            }))
                        } else {
                            None
                        }
                    }
                    Event::Text(text) => Some(egui::Event::Text(text.clone())),
                    Event::MouseLeave => Some(egui::Event::PointerGone),
                    Event::MouseWheel {
                        delta,
                        handled,
                        modifiers,
                        ..
                    } => {
                        if !handled {
                            Some(egui::Event::MouseWheel {
                                delta: egui::Vec2::new(delta.0, delta.1),
                                unit: egui::MouseWheelUnit::Point,
                                modifiers: modifiers.into(),
                                phase: egui::TouchPhase::Move,
                            })
                        } else {
                            None
                        }
                    }
                    Event::PinchGesture { delta, handled, .. } => {
                        if !handled {
                            Some(egui::Event::Zoom(delta.exp()))
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        };

        *self.output.borrow_mut() = Some(self.egui_context.run_ui(egui_input, callback));

        for event in events.iter_mut() {
            if let Event::ModifiersChange { modifiers } = event {
                self.modifiers = *modifiers;
            }
            if self.egui_context.egui_is_using_pointer() {
                match event {
                    Event::MousePress {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::MouseRelease {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::MouseWheel {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::MouseMotion {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::PinchGesture {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::RotationGesture {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    _ => {}
                }
            }

            if self.egui_context.egui_wants_keyboard_input() {
                match event {
                    Event::KeyRelease {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::KeyPress {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    _ => {}
                }
            }
        }
        self.egui_context.egui_wants_pointer_input()
            || self.egui_context.egui_wants_keyboard_input()
    }

    ///
    /// Render the GUI defined in the [update](Self::update) function.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(&self) -> Result<(), crate::CoreError> {
        let output = self
            .output
            .borrow_mut()
            .take()
            .expect("need to call GUI::update before GUI::render");
        let scale = self.egui_context.pixels_per_point();
        let clipped_meshes = self.egui_context.tessellate(output.shapes, scale);
        self.painter.borrow_mut().paint_and_update_textures(
            [self.viewport.width, self.viewport.height],
            scale,
            &clipped_meshes,
            &output.textures_delta,
        );
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(unsafe_code)]
        unsafe {
            use glow::HasContext as _;
            self.painter.borrow().gl().disable(glow::FRAMEBUFFER_SRGB);
        }
        Ok(())
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        self.painter.borrow_mut().destroy();
    }
}

fn translate_key(key: &Key) -> Option<egui::Key> {
    use crate::control::Key as InputKey;
    use egui::Key;

    Some(match key {
        InputKey::ArrowDown => Key::ArrowDown,
        InputKey::ArrowLeft => Key::ArrowLeft,
        InputKey::ArrowRight => Key::ArrowRight,
        InputKey::ArrowUp => Key::ArrowUp,
        InputKey::Escape => Key::Escape,
        InputKey::Tab => Key::Tab,
        InputKey::Backspace => Key::Backspace,
        InputKey::Enter => Key::Enter,
        InputKey::Space => Key::Space,
        InputKey::Insert => Key::Insert,
        InputKey::Delete => Key::Delete,
        InputKey::Home => Key::Home,
        InputKey::End => Key::End,
        InputKey::PageUp => Key::PageUp,
        InputKey::PageDown => Key::PageDown,
        InputKey::Copy => Key::Copy,
        InputKey::Paste => Key::Paste,
        InputKey::Cut => Key::Cut,
        InputKey::Num0 => Key::Num0,
        InputKey::Num1 => Key::Num1,
        InputKey::Num2 => Key::Num2,
        InputKey::Num3 => Key::Num3,
        InputKey::Num4 => Key::Num4,
        InputKey::Num5 => Key::Num5,
        InputKey::Num6 => Key::Num6,
        InputKey::Num7 => Key::Num7,
        InputKey::Num8 => Key::Num8,
        InputKey::Num9 => Key::Num9,
        InputKey::A => Key::A,
        InputKey::B => Key::B,
        InputKey::C => Key::C,
        InputKey::D => Key::D,
        InputKey::E => Key::E,
        InputKey::F => Key::F,
        InputKey::G => Key::G,
        InputKey::H => Key::H,
        InputKey::I => Key::I,
        InputKey::J => Key::J,
        InputKey::K => Key::K,
        InputKey::L => Key::L,
        InputKey::M => Key::M,
        InputKey::N => Key::N,
        InputKey::O => Key::O,
        InputKey::P => Key::P,
        InputKey::Q => Key::Q,
        InputKey::R => Key::R,
        InputKey::S => Key::S,
        InputKey::T => Key::T,
        InputKey::U => Key::U,
        InputKey::V => Key::V,
        InputKey::W => Key::W,
        InputKey::X => Key::X,
        InputKey::Y => Key::Y,
        InputKey::Z => Key::Z,
        InputKey::F1 => Key::F1,
        InputKey::F2 => Key::F2,
        InputKey::F3 => Key::F3,
        InputKey::F4 => Key::F4,
        InputKey::F5 => Key::F5,
        InputKey::F6 => Key::F6,
        InputKey::F7 => Key::F7,
        InputKey::F8 => Key::F8,
        InputKey::F9 => Key::F9,
        InputKey::F10 => Key::F10,
        InputKey::F11 => Key::F11,
        InputKey::F12 => Key::F12,
        InputKey::F13 => Key::F13,
        InputKey::F14 => Key::F14,
        InputKey::F15 => Key::F15,
        InputKey::F16 => Key::F16,
        InputKey::F17 => Key::F17,
        InputKey::F18 => Key::F18,
        InputKey::F19 => Key::F19,
        InputKey::F20 => Key::F20,
        InputKey::F21 => Key::F21,
        InputKey::F22 => Key::F22,
        InputKey::F23 => Key::F23,
        InputKey::F24 => Key::F24,
        InputKey::Apostrophe => Key::Quote,
        InputKey::Backslash => Key::Backslash,
        InputKey::Colon => Key::Colon,
        InputKey::Comma => Key::Comma,
        InputKey::Equals => Key::Equals,
        InputKey::Grave => Key::Backtick,
        InputKey::LBracket => Key::OpenBracket,
        InputKey::Minus => Key::Minus,
        InputKey::Period => Key::Period,
        InputKey::Plus => Key::Plus,
        InputKey::RBracket => Key::CloseBracket,
        InputKey::Semicolon => Key::Semicolon,
        InputKey::Slash => Key::Slash,
        _ => return None,
    })
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

impl From<&MouseButton> for egui::PointerButton {
    fn from(button: &MouseButton) -> Self {
        match button {
            MouseButton::Left => egui::PointerButton::Primary,
            MouseButton::Right => egui::PointerButton::Secondary,
            MouseButton::Middle => egui::PointerButton::Middle,
        }
    }
}
