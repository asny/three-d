// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Environment!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/chinese_garden_4k.hdr", "../assets/suzanne.obj"])
            .await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/chinese_garden_4k.hdr", "https://asny.github.io/three-d/assets/suzanne.obj"
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let skybox = Skybox::new_from_equirectangular(
        &context,
        &loaded.deserialize("chinese_garden_4k").unwrap(),
    );

    let mut options = EnvironmentOptions::default();
    let mut current_options = EnvironmentOptions::default();

    let mut light = AmbientLight::new(&context, 1.0, Srgba::WHITE);
    light.environment = Some(Environment::new_with_options(
        &context,
        skybox.texture(),
        options,
    ));

    let cpu_mesh: CpuMesh = loaded.deserialize("suzanne.obj").unwrap();
    let mut model = Gm::new(
        Mesh::new(&context, &cpu_mesh),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                roughness: 0.2,
                metallic: 0.8,
                ..Default::default()
            },
        ),
    );
    let mut gui = three_d::GUI::new(&context);

    // main loop
    let mut color = [1.0; 4];
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ui| {
                use three_d::egui::*;
                Panel::left("side_panel").show_inside(ui, |ui| {
                    ui.heading("Debug Panel");

                    ui.label("Material");
                    ui.add(Slider::new(&mut model.material.metallic, 0.0..=1.0).text("Metallic"));
                    ui.add(Slider::new(&mut model.material.roughness, 0.0..=1.0).text("Roughness"));
                    ui.color_edit_button_rgba_unmultiplied(&mut color);

                    ui.label("Lighting model");
                    ui.radio_value(
                        &mut model.material.lighting_model,
                        LightingModel::Phong,
                        "Phong",
                    );
                    ui.radio_value(
                        &mut model.material.lighting_model,
                        LightingModel::Blinn,
                        "Blinn",
                    );
                    ui.radio_value(
                        &mut model.material.lighting_model,
                        LightingModel::Cook(
                            NormalDistributionFunction::Blinn,
                            GeometryFunction::SmithSchlickGGX,
                        ),
                        "Cook (Blinn)",
                    );
                    ui.radio_value(
                        &mut model.material.lighting_model,
                        LightingModel::Cook(
                            NormalDistributionFunction::Beckmann,
                            GeometryFunction::SmithSchlickGGX,
                        ),
                        "Cook (Beckmann)",
                    );
                    ui.radio_value(
                        &mut model.material.lighting_model,
                        LightingModel::Cook(
                            NormalDistributionFunction::TrowbridgeReitzGGX,
                            GeometryFunction::SmithSchlickGGX,
                        ),
                        "Cook (Trowbridge-Reitz GGX)",
                    );

                    ui.label("BRDF");
                    ui.add(
                        Slider::new(&mut options.brdf_sample_count, 1..=1024)
                            .text("BRDF sample count"),
                    );
                    ui.add(Slider::new(&mut options.brdf_map_size, 1..=1024).text("BRDF map size"));

                    ui.label("Irradiance");
                    ui.add(
                        Slider::new(&mut options.irradiance_sample_count, 1..=1024)
                            .text("Irradiance sample count"),
                    );
                    ui.add(
                        Slider::new(&mut options.irradiance_map_size, 1..=128)
                            .text("Irradiance map size"),
                    );

                    ui.label("Prefilter");
                    ui.add(
                        Slider::new(&mut options.prefilter_sample_count, 1..=1024)
                            .text("Prefilter sample count"),
                    );
                    ui.add(
                        Slider::new(&mut options.prefilter_map_size, 1..=512)
                            .text("Prefilter map size"),
                    );
                    ui.add(
                        Slider::new(&mut options.prefilter_map_max_mip_levels, 1..=10)
                            .text("Prefilter max mip levels"),
                    );
                });
                panel_width = frame_input.window_width as f32 - ui.available_width();
            },
        );
        model.material.albedo = Srgba::from(color);

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        if options != current_options {
            light.environment = Some(Environment::new_with_options(
                &context,
                skybox.texture(),
                options,
            ));
            current_options = options;
        }
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, skybox.into_iter().chain(&model), &[&light])
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default()
    });
}
