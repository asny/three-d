
use crate::gui::painter::*;
use crate::*;


struct MyRepaintSignal {}

impl epi::RepaintSignal for MyRepaintSignal {
    fn request_repaint(&self) {}
}

pub struct GUI {
    context: Context,
    egui_context: egui::CtxRef,
    painter: Painter
}

impl GUI {
    pub fn new(context: &Context) -> Result<Self, Error> {

        /*if app.warm_up_enabled() {
            // let warm_up_start = Instant::now();
            input_state.raw.time = Some(0.0);
            input_state.raw.screen_rect = Some(Rect::from_min_size(
                Default::default(),
                screen_size_in_pixels(&display) / input_state.raw.pixels_per_point.unwrap(),
            ));
            ctx.begin_frame(input_state.raw.take());
            let mut app_output = epi::backend::AppOutput::default();
            let mut frame = epi::backend::FrameBuilder {
                info: integration_info(&display, None),
                tex_allocator: Some(&mut painter),
                #[cfg(feature = "http")]
                http: http.clone(),
                output: &mut app_output,
                repaint_signal: repaint_signal.clone(),
            }.build();

            let saved_memory = ctx.memory().clone();
            ctx.memory().set_everything_is_visible(true);
            app.update(&ctx, &mut frame);
            *ctx.memory() = saved_memory; // We don't want to remember that windows were huge.
            ctx.clear_animations();

            let (egui_output, _shapes) = ctx.end_frame();
            handle_output(egui_output, &display, clipboard.as_mut());
            // TODO: handle app_output
            // eprintln!("Warmed up in {} ms", warm_up_start.elapsed().as_millis())
        }*/


        let mut egui_context = egui::CtxRef::default();
        //app.setup(&egui_context);
        Ok(GUI {
            egui_context,
            context: context.clone(),
            painter: Painter::new(context)?
        })
    }

    pub fn render<F: FnOnce(&egui::CtxRef)>(&mut self, frame_input: &FrameInput, callback: F) -> Result<(), Error> {

        /*TODO:for event in frame_input.events {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => {
                    input_to_egui(event, clipboard.as_mut(), &mut input_state, control_flow);
                    display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
                }
                _ => (),
            }
        };*/

        let mut input_state = egui::RawInput {
            scroll_delta: egui::Vec2::ZERO,
            screen_size: Default::default(),
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(),
                egui::Vec2 {x: frame_input.window_width as f32, y: frame_input.window_height as f32},
            )),
            pixels_per_point: None,
            time: None,
            predicted_dt: 1.0 / 60.0,
            modifiers: egui::Modifiers::default(),
            events: vec![],
        };
        self.egui_context.begin_frame(input_state);

        /*let mut app_output = epi::backend::AppOutput::default();
        let repaint_signal = std::sync::Arc::new(MyRepaintSignal {});
        let mut frame = epi::backend::FrameBuilder {
            info: epi::IntegrationInfo {
                web_info: None,
                cpu_usage: Some(frame_input.elapsed_time as f32),
                seconds_since_midnight: None,
                native_pixels_per_point: Some(1.0),
            },
            tex_allocator: None,
            output: &mut app_output,
            repaint_signal: repaint_signal.clone(),
        }.build();
        self.app.update(&self.egui_context, &mut frame);*/
        callback(&self.egui_context);

        let (egui_output, shapes) = self.egui_context.end_frame();
        let clipped_meshes = self.egui_context.tessellate(shapes);
        self.painter.paint_meshes(
            frame_input.window_width,
            frame_input.window_height,
            self.egui_context.pixels_per_point(),
            clipped_meshes,
            &self.egui_context.texture(),
        )?;
        Ok(())
    }
}

