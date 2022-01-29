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
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let scene = Loading::new(
        &context,
        &["examples/assets/chinese_garden_4k.hdr"], // Source: https://polyhaven.com/
        move |context, mut loaded| {
            let skybox = Skybox::new_with_texture(
                &context,
                TextureCubeMap::<f16>::new_from_equirectangular(
                    &context,
                    &loaded.hdr_image("chinese_garden_4k")?,
                )?,
            )
            .unwrap();
            let light = AmbientLight {
                environment: Some(Environment::new(&context, skybox.texture())?),
                ..Default::default()
            };
            Ok((skybox, light))
        },
    );

    let mut model = Model::new_with_material(
        &context,
        &CPUMesh::sphere(32),
        PhysicalMaterial {
            roughness: 0.2,
            metallic: 0.8,
            ..Default::default()
        },
    )
    .unwrap();
    let mut gui = three_d::GUI::new(&context).unwrap();

    // main loop
    let mut color = [1.0; 4];
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = 0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut model.material.metallic, 0.0..=1.0).text("Metallic"));
                    ui.add(Slider::new(&mut model.material.roughness, 0.0..=1.0).text("Roughness"));
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });
                panel_width = gui_context.used_size().x as u32;
            })
            .unwrap();
            model.material.albedo = Color::from_rgba_slice(&color);

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
                        let (skybox, light) = scene.as_ref().unwrap();
                        skybox.render(&camera)?;
                        model.render(&camera, &[light])?;
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
