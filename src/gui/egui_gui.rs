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
            painter: RefCell::new(Painter::new(context, "", None).unwrap()),
            output: RefCell::new(None),
            viewport: Viewport::new_at_origo(1, 1),
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
        events: &mut [Event],
        accumulated_time_in_ms: f64,
        viewport: Viewport,
        device_pixel_ratio: f64,
        callback: impl FnOnce(&egui::Context),
    ) -> bool {
        self.viewport = viewport;
        let egui_input = egui::RawInput {
            screen_rect: Some(egui::Rect {
                min: egui::Pos2 {
                    x: viewport.x as f32 / device_pixel_ratio as f32,
                    y: viewport.y as f32 / device_pixel_ratio as f32,
                },
                max: egui::Pos2 {
                    x: viewport.x as f32 / device_pixel_ratio as f32
                        + viewport.width as f32 / device_pixel_ratio as f32,
                    y: viewport.y as f32 / device_pixel_ratio as f32
                        + viewport.height as f32 / device_pixel_ratio as f32,
                },
            }),
            pixels_per_point: Some(device_pixel_ratio as f32),
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
                .collect::<Vec<_>>(),
            ..Default::default()
        };

        self.egui_context.begin_frame(egui_input);
        callback(&self.egui_context);
        *self.output.borrow_mut() = Some(self.egui_context.end_frame());

        for event in events.iter_mut() {
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
    pub fn render(&self) {
        let output = self
            .output
            .borrow_mut()
            .take()
            .expect("need to call GUI::update before GUI::render");
        let clipped_meshes = self.egui_context.tessellate(output.shapes);
        let scale = self.egui_context.pixels_per_point();
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
