use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Environment!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 5.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let scene = Loading::new(
        &context,
        &["examples/assets/chinese_garden_4k.hdr"],
        move |context, mut loaded| {
            let environment_map = loaded.hdr_image("").unwrap();
            let skybox = Skybox::new_from_equirectangular(&context, &environment_map).unwrap();
            let lights = Lights {
                environment: Some(EnvironmentLight::new(&context, skybox.texture())?),
                lighting_model: LightingModel::Cook(
                    NormalDistributionFunction::TrowbridgeReitzGGX,
                    GeometryFunction::SmithSchlickGGX,
                ),
                ..Default::default()
            };
            Ok((skybox, lights))
        },
    );

    let mut model = Model::new_with_material(
        &context,
        &CPUMesh::sphere(16),
        PhysicalMaterial {
            albedo: Color::WHITE,
            roughness: 0.5,
            metallic: 0.8,
            ..Default::default()
        },
    )
    .unwrap();
    let mut gui = three_d::GUI::new(&context).unwrap();

    // main loop
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = 0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut model.material.metallic, 0.0..=1.0).text("Metallic"));
                    ui.add(Slider::new(&mut model.material.roughness, 0.0..=1.0).text("Roughness"));
                });
                panel_width = gui_context.used_size().x as u32;
            })
            .unwrap();

            let viewport = Viewport {
                x: panel_width as i32,
                y: 0,
                width: frame_input.viewport.width - panel_width,
                height: frame_input.viewport.height,
            };
            camera.set_viewport(viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    if let Some(ref scene) = *scene.borrow() {
                        let (skybox, lights) = scene.as_ref().unwrap();
                        skybox.render(&camera)?;
                        model.render(&camera, lights)?;
                    }
                    gui.render()?;
                    Ok(())
                },
            )
            .unwrap();

            if args.len() > 1 {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(args[1].clone().into()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
