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

    let mut camera = Camera::new_2d(window.viewport());

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
    let image = std::sync::Arc::new(Texture2D::new(&context, &loaded.deserialize("").unwrap()));

    let mut gui = GUI::new(&context);

    // main loop
    let mut texture_transform_scale = 1.0;
    let mut texture_transform_x = 0.0;
    let mut texture_transform_y = 0.0;
    let mut tone_mapping = ToneMapping::default();
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::right("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
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
                    ui.label("Tone mapping");
                    ui.radio_value(&mut tone_mapping, ToneMapping::None, "None");
                    ui.radio_value(&mut tone_mapping, ToneMapping::Reinhard, "Reinhard");
                    ui.radio_value(&mut tone_mapping, ToneMapping::Aces, "Aces");
                    ui.radio_value(&mut tone_mapping, ToneMapping::Filmic, "Filmic");
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        let viewport = Viewport::new_at_origo(
            frame_input.viewport.width - (panel_width * frame_input.device_pixel_ratio) as u32,
            frame_input.viewport.height,
        );
        camera.set_viewport(viewport);

        let material = ColorMaterial {
            texture: Some(Texture2DRef {
                texture: image.clone(),
                transformation: Mat3::from_scale(texture_transform_scale)
                    * Mat3::from_translation(vec2(texture_transform_x, texture_transform_y)),
            }),
            ..Default::default()
        };

        let mut target = Texture2D::new_empty::<[f16; 4]>(
            &context,
            viewport.width,
            viewport.height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        camera.disable_tone_and_color_mapping();
        target
            .as_color_target(None)
            .clear(ClearState::default())
            .apply_screen_material(&material, &camera, &[]);

        camera.color_mapping = ColorMapping::default();
        camera.tone_mapping = tone_mapping;
        frame_input
            .screen()
            .clear(ClearState::default())
            .apply_screen_effect(
                &ScreenEffect::default(),
                &camera,
                &[],
                Some(ColorTexture::Single(&target)),
                None,
            )
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default()
    });
}
