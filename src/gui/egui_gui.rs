use crate::control::*;
use crate::core::*;
use egui_glow::Painter;

#[doc(hidden)]
pub use egui;

///
/// Integration of [egui](https://crates.io/crates/egui), an immediate mode GUI.
///
pub struct GUI {
    painter: Painter,
    egui_context: egui::Context,
    output: Option<egui::FullOutput>,
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
        #[allow(unsafe_code)] // Temporary until egui takes Arc
        let context = unsafe { std::rc::Rc::from_raw(std::sync::Arc::into_raw(context)) };
        GUI {
            egui_context: egui::Context::default(),
            painter: Painter::new(context, None, "").unwrap(),
            output: None,
            modifiers: Modifiers::default(),
        }
    }

    ///
    /// Initialises a new frame of the GUI and handles events.
    /// Construct the GUI (Add panels, widgets etc.) using the [egui::Context] in the callback function.
    /// This function returns whether or not the GUI has changed, ie. if it consumes any events, and therefore needs to be rendered again.
    ///
    pub fn update(
        &mut self,
        frame_input: &mut FrameInput,
        callback: impl FnOnce(&egui::Context),
    ) -> bool {
        self.egui_context
            .begin_frame(construct_egui_input(frame_input, &self.modifiers));
        callback(&self.egui_context);
        self.output = Some(self.egui_context.end_frame());

        for event in frame_input.events.iter_mut() {
            if let Event::ModifiersChange { modifiers } = event {
                self.modifiers = *modifiers;
            }
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
            }
        }
        self.egui_context.wants_pointer_input() || self.egui_context.wants_keyboard_input()
    }

    ///
    /// Render the GUI defined in the [update](Self::update) function.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(&mut self, viewport: Viewport) {
        let output = self
            .output
            .take()
            .expect("need to call GUI::update before GUI::render");
        let clipped_meshes = self.egui_context.tessellate(output.shapes);
        let scale = self.egui_context.pixels_per_point();
        self.painter.paint_and_update_textures(
            [viewport.width, viewport.height],
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

fn construct_egui_input(frame_input: &FrameInput, egui_modifiers: &Modifiers) -> egui::RawInput {
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
            _ => None,
        })
        .collect::<Vec<_>>();
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2 {
                x: frame_input.viewport.x as f32,
                y: frame_input.viewport.y as f32,
            },
            egui::Vec2 {
                x: frame_input.viewport.width as f32,
                y: frame_input.viewport.height as f32,
            },
        )),
        pixels_per_point: Some(frame_input.device_pixel_ratio as f32),
        time: Some(frame_input.accumulated_time * 0.001),
        modifiers: egui_modifiers.into(),
        events,
        ..Default::default()
    }
}

impl From<&Key> for egui::Key {
    fn from(key: &Key) -> Self {
        use crate::control::Key::*;
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
