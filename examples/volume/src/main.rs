// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Volume!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.25, -0.5, -2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Source: https://web.cs.ucdavis.edu/~okreylos/PhDStudies/Spring2000/ECS277/DataSets.html
    let cpu_volume = three_d_asset::io::load_async(&["examples/assets/Skull.vol"])
        .await
        .unwrap()
        .vol("")
        .unwrap();
    let mut volume = Model::new_with_material(
        &context,
        &CpuMesh::cube(),
        IsourfaceMaterial {
            voxels: std::rc::Rc::new(Texture3D::new(&context, &cpu_volume.voxels).unwrap()),
            lighting_model: LightingModel::Blinn,
            size: cpu_volume.size,
            threshold: 0.15,
            color: Color::WHITE,
            roughness: 1.0,
            metallic: 0.0,
        },
    )
    .unwrap();
    volume.set_transformation(Mat4::from_nonuniform_scale(
        0.5 * cpu_volume.size.x,
        0.5 * cpu_volume.size.y,
        0.5 * cpu_volume.size.z,
    ));

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional1 =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();
    let directional2 =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0)).unwrap();

    // main loop
    let mut gui = three_d::GUI::new(&context).unwrap();
    let mut color = [1.0; 4];
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = 0.0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(
                        Slider::new(&mut volume.material.threshold, 0.0..=1.0).text("Threshold"),
                    );
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });
                panel_width = gui_context.used_size().x as f64;
            })
            .unwrap();
            volume.material.color = Color::from_rgba_slice(&color);

            let viewport = Viewport {
                x: (panel_width * frame_input.device_pixel_ratio) as i32,
                y: 0,
                width: frame_input.viewport.width
                    - (panel_width * frame_input.device_pixel_ratio) as u32,
                height: frame_input.viewport.height,
            };
            camera.set_viewport(viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            // draw
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
                .unwrap()
                .render(
                    &camera,
                    &[&volume],
                    &[&ambient, &directional1, &directional2],
                )
                .unwrap()
                .write(|| gui.render())
                .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}
