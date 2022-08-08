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
    pub fn update(
        &mut self,
        frame_input: &mut FrameInput,
        callback: impl FnOnce(&egui::Context),
    ) -> bool {
        self.width = frame_input.window_width;
        self.height = frame_input.window_height;
        self.egui_context
            .begin_frame(egui::RawInput::from(&*frame_input));
        callback(&self.egui_context);

        for event in frame_input.events.iter_mut() {
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
