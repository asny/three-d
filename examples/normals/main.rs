use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Normals".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let model = Loading::new(
        &context,
        &["examples/assets/gltf/NormalTangentTest.glb"],
        move |context, mut loaded| {
            let (mut cpu_meshes, cpu_materials) = loaded.gltf("NormalTangentTest.glb").unwrap();
            let mut material = PhysicalMaterial::new(&context, &cpu_materials[0]).unwrap();
            material.opaque_render_states.cull = Cull::Back;
            cpu_meshes[0].compute_tangents();
            let mut model =
                Model::new_with_material(&context, &cpu_meshes[0], material.clone()).unwrap();
            Ok(model)
        },
    );

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(3.0, 1.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut lights = Lights {
        ambient: Some(AmbientLight {
            color: Color::WHITE,
            intensity: 0.4,
        }),
        directional: vec![
            DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap(),
            DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap(),
        ],
        lighting_model: LightingModel::Cook(
            NormalDistributionFunction::TrowbridgeReitzGGX,
            GeometryFunction::SmithSchlickGGX,
        ),
        ..Default::default()
    };

    // main loop
    window
        .render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            let time = 0.001 * frame_input.accumulated_time;
            let c = time.cos() as f32;
            let s = time.sin() as f32;
            lights.directional[0].set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
            lights.directional[1].set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));

            // Draw
            if let Some(Ok(ref model)) = *model.borrow() {
                lights.directional[0]
                    .generate_shadow_map(2.0, 1024, 1024, &[model])
                    .unwrap();
                lights.directional[1]
                    .generate_shadow_map(2.0, 1024, 1024, &[model])
                    .unwrap();
            }
            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    if let Some(Ok(ref model)) = *model.borrow() {
                        model.render_forward(
                            &NormalMaterial::from_physical_material(&model.material),
                            &camera,
                            &lights,
                        )?;
                    }
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
