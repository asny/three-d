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
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/chinese_garden_4k.hdr"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/chinese_garden_4k.hdr",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let skybox = Skybox::new_from_equirectangular(
        &context,
        &loaded.deserialize("chinese_garden_4k").unwrap(),
    );
    let light = AmbientLight::new_with_environment(&context, 1.0, Srgba::WHITE, skybox.texture());

    let mut model = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(32)),
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
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut model.material.metallic, 0.0..=1.0).text("Metallic"));
                    ui.add(Slider::new(&mut model.material.roughness, 0.0..=1.0).text("Roughness"));
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });
                panel_width = gui_context.used_rect().width();
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

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, skybox.into_iter().chain(&model), &[&light])
            .write(|| gui.render());

        FrameOutput::default()
    });
}
