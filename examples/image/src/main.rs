// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::core::*;
use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Image!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/syferfontein_18d_clear_4k.hdr"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/syferfontein_18d_clear_4k.hdr",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };
    let image = Texture2D::new(&context, &loaded.deserialize("").unwrap());

    let mut gui = GUI::new(&context);

    let mut image_viewer = Gm::new(
        ScreenQuad::new(&context),
        ImageMaterial {
            image,
            tone_mapping: 1.0,
        },
    );

    // main loop
    let mut texture_transform_scale = 1.0;
    let mut texture_transform_x = 0.0;
    let mut texture_transform_y = 0.0;
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(
                        Slider::new(&mut image_viewer.material.tone_mapping, 0.0..=50.0)
                            .text("Tone mapping"),
                    );
                    ui.add(
                        Slider::new(&mut texture_transform_scale, 0.0..=10.0)
                            .text("Texture transform scale"),
                    );
                    ui.add(
                        Slider::new(&mut texture_transform_x, 0.0..=1.0)
                            .text("Texture transform x"),
                    );
                    ui.add(
                        Slider::new(&mut texture_transform_y, 0.0..=1.0)
                            .text("Texture transform y"),
                    );
                });
                panel_width = gui_context.used_size().x as f64;
            },
        );

        image_viewer.geometry.set_texture_transform(
            Mat3::from_scale(texture_transform_scale)
                * Mat3::from_translation(vec2(texture_transform_x, texture_transform_y)),
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };

        frame_input
            .screen()
            .clear(ClearState::default())
            .render(&camera2d(viewport), &image_viewer, &[])
            .write(|| {
                gui.render(frame_input.viewport);
            });

        FrameOutput::default()
    });
}

struct ImageMaterial {
    pub image: Texture2D,
    pub tone_mapping: f32,
}

impl Material for ImageMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("shader.frag").to_owned()
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_texture("image", &self.image);
        program.use_uniform("parameter", self.tone_mapping);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
