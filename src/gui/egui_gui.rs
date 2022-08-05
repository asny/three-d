use crate::control::*;
use crate::core::*;
#[doc(hidden)]
pub use egui;
use egui_glow::Painter;

///
/// Integration of [egui](https://crates.io/crates/egui), an immediate mode GUI.
///
pub struct GUI {
    painter: Painter,
    egui_context: egui::Context,
    width: u32,
    height: u32,
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
        #[allow(unsafe_code)] // Temporary until egui takes Arc
        let context = unsafe { std::rc::Rc::from_raw(std::sync::Arc::into_raw(context)) };
        GUI {
            egui_context: egui::Context::default(),
            painter: Painter::new(context, None, "").unwrap(),
            width: 0,
            height: 0,
        }
    }

    ///
    /// Initialises a new frame of the GUI and handles events.
    /// Construct the GUI (Add panels, widgets etc.) using the [egui::Context] in the callback function.
    /// This function returns whether or not the GUI has changed, ie. if it consumes any events, and therefore needs to be rendered again.
    ///
    pub fn update<F: FnOnce(&egui::Context)>(
        &mut self,
        width: u32,
        height: u32,
        device_pixel_ratio: f64,
        accumulated_time: f64,
        events: &mut [Event],
        callback: F,
    ) -> bool {
        self.width = width;
        self.height = height;
        let input_state =
            construct_input_state(width, height, device_pixel_ratio, accumulated_time, events);
        self.egui_context.begin_frame(input_state);
        callback(&self.egui_context);

        let mut change = false;
        for event in events.iter_mut() {
            if self.egui_context.wants_pointer_input() {
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
                    _ => {}
                }
                change = true;
            }

            if self.egui_context.wants_keyboard_input() {
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
                change = true;
            }
        }
        change
    }

    ///
    /// Render the GUI defined in the [update](Self::update) function.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(&mut self) {
        let output = self.egui_context.end_frame();
        let clipped_meshes = self.egui_context.tessellate(output.shapes);
        let scale = self.egui_context.pixels_per_point();
        self.painter.paint_and_update_textures(
            [
                (self.width as f32 * scale).round() as u32,
                (self.height as f32 * scale).round() as u32,
            ],
            scale,
            &clipped_meshes,
            &output.textures_delta,
        );
        #[allow(unsafe_code)]
        unsafe {
            use glow::HasContext as _;
            self.painter.gl().disable(glow::FRAMEBUFFER_SRGB);
        }
    }
}

fn construct_input_state(
    width: u32,
    height: u32,
    device_pixel_ratio: f64,
    accumulated_time: f64,
    events: &[Event],
) -> egui::RawInput {
    let mut egui_modifiers = egui::Modifiers::default();
    let mut egui_events = Vec::new();
    for event in events.iter() {
        match event {
            Event::KeyPress {
                kind,
                modifiers,
                handled,
            } => {
                if !handled {
                    egui_events.push(egui::Event::Key {
                        key: translate_to_egui_key_code(kind),
                        pressed: true,
                        modifiers: map_modifiers(modifiers),
                    });
                }
            }
            Event::KeyRelease {
                kind,
                modifiers,
                handled,
            } => {
                if !handled {
                    egui_events.push(egui::Event::Key {
                        key: translate_to_egui_key_code(kind),
                        pressed: false,
                        modifiers: map_modifiers(modifiers),
                    });
                }
            }
            Event::MousePress {
                button,
                position,
                modifiers,
                handled,
            } => {
                if !handled {
                    egui_events.push(egui::Event::PointerButton {
                        pos: egui::Pos2 {
                            x: position.0 as f32,
                            y: position.1 as f32,
                        },
                        button: match button {
                            MouseButton::Left => egui::PointerButton::Primary,
                            MouseButton::Right => egui::PointerButton::Secondary,
                            MouseButton::Middle => egui::PointerButton::Middle,
                        },
                        pressed: true,
                        modifiers: map_modifiers(modifiers),
                    });
                }
            }
            Event::MouseRelease {
                button,
                position,
                modifiers,
                handled,
            } => {
                if !handled {
                    egui_events.push(egui::Event::PointerButton {
                        pos: egui::Pos2 {
                            x: position.0 as f32,
                            y: position.1 as f32,
                        },
                        button: match button {
                            MouseButton::Left => egui::PointerButton::Primary,
                            MouseButton::Right => egui::PointerButton::Secondary,
                            MouseButton::Middle => egui::PointerButton::Middle,
                        },
                        pressed: false,
                        modifiers: map_modifiers(modifiers),
                    });
                }
            }
            Event::MouseMotion {
                position, handled, ..
            } => {
                if !handled {
                    egui_events.push(egui::Event::PointerMoved(egui::Pos2 {
                        x: position.0 as f32,
                        y: position.1 as f32,
                    }));
                }
            }
            Event::Text(text) => {
                egui_events.push(egui::Event::Text(text.clone()));
            }
            Event::MouseLeave => {
                egui_events.push(egui::Event::PointerGone);
            }
            Event::MouseWheel { delta, handled, .. } => {
                if !handled {
                    egui_events.push(egui::Event::Scroll(egui::Vec2::new(
                        delta.0 as f32,
                        delta.1 as f32,
                    )));
                }
            }
            Event::ModifiersChange { modifiers } => egui_modifiers = map_modifiers(modifiers),
            _ => (),
        }
    }

    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            Default::default(),
            egui::Vec2 {
                x: width as f32,
                y: height as f32,
            },
        )),
        pixels_per_point: Some(device_pixel_ratio as f32),
        time: Some(accumulated_time * 0.001),
        modifiers: egui_modifiers,
        events: egui_events,
        ..Default::default()
    }
}

fn translate_to_egui_key_code(key: &crate::Key) -> egui::Key {
    use crate::Key::*;
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

fn map_modifiers(modifiers: &Modifiers) -> egui::Modifiers {
    egui::Modifiers {
        alt: modifiers.alt,
        ctrl: modifiers.ctrl,
        shift: modifiers.shift,
        command: modifiers.command,
        mac_cmd: cfg!(target_os = "macos") && modifiers.command,
    }
}
