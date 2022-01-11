use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Lights!".to_string(),
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
        &[
            "examples/assets/gltf/DamagedHelmet.glb",
            "examples/assets/chinese_garden_4k.hdr",
        ],
        move |context, mut loaded| {
            let environment_map = loaded.hdr_image("chinese").unwrap();
            let skybox = Skybox::new_from_equirectangular(&context, &environment_map).unwrap();

            let (mut cpu_meshes, cpu_materials) = loaded.gltf("DamagedHelmet.glb").unwrap();
            let mut material = PhysicalMaterial::new(&context, &cpu_materials[0]).unwrap();
            material.opaque_render_states.cull = Cull::Back;
            cpu_meshes[0].compute_tangents().unwrap();
            let mut model =
                Model::new_with_material(&context, &cpu_meshes[0], material.clone()).unwrap();
            model.set_transformation(Mat4::from_angle_x(degrees(90.0)));

            let mut directional = Vec::new();
            for _ in 0..0 {
                directional.push(
                    DirectionalLight::new(&context, 0.2, Color::RED, &vec3(0.0, -1.0, 0.0))
                        .unwrap(),
                );
            }

            let mut point = Vec::new();
            for _ in 0..30 {
                point.push(
                    PointLight::new(
                        &context,
                        0.2,
                        Color::RED,
                        &vec3(0.0, -1.0, 0.0),
                        0.5,
                        0.05,
                        0.005,
                    )
                    .unwrap(),
                );
            }

            let lights = Lights {
                ambient: Some(AmbientLight {
                    environment: Some(Environment::new(&context, skybox.texture())?),
                    ..Default::default()
                }),
                lighting_model: LightingModel::Cook(
                    NormalDistributionFunction::TrowbridgeReitzGGX,
                    GeometryFunction::SmithSchlickGGX,
                ),
                directional: directional,
                point: point,
                ..Default::default()
            };
            Ok((model, skybox, lights))
        },
    );

    // main loop
    window
        .render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    if let Some(ref scene) = *scene.borrow() {
                        let (model, skybox, lights) = scene.as_ref().unwrap();
                        skybox.render(&camera)?;
                        model.render(&camera, lights)?;
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
