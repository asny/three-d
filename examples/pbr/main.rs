use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "PBR!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let target = vec3(0.0, 0.0, 0.0);
    let mut camera = CameraControl::new(
        Camera::new_perspective(
            &context,
            vec3(2.0, 2.0, 5.0),
            target,
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            window.viewport().unwrap().aspect(),
            0.1,
            1000.0,
        )
        .unwrap(),
    );

    Loader::load(&["examples/assets/gltf/DamagedHelmet.glb"], move |loaded| {
        let (cpu_meshes, cpu_materials) = loaded
            .gltf("examples/assets/gltf/DamagedHelmet.glb")
            .unwrap();
        let mut model = PhongMesh::new(
            &context,
            &cpu_meshes[0],
            &PhongMaterial::new(&context, &cpu_materials[0]).unwrap(),
        )
        .unwrap();
        model.cull = CullType::Back;

        let plane = PhongMesh::new(
            &context,
            &CPUMesh {
                positions: vec![
                    -10000.0, -1.3, 10000.0, 10000.0, -1.3, 10000.0, 0.0, -1.3, -10000.0,
                ],
                normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                ..Default::default()
            },
            &PhongMaterial {
                color_source: ColorSource::Color(vec4(0.5, 0.7, 0.3, 1.0)),
                diffuse_intensity: 0.7,
                specular_intensity: 0.8,
                specular_power: 20.0,
                ..Default::default()
            },
        )
        .unwrap();

        let ambient_light = AmbientLight {
            color: vec3(1.0, 1.0, 1.0),
            intensity: 0.2,
        };
        let mut directional_light0 =
            DirectionalLight::new(&context, 0.3, &vec3(1.0, 1.0, 1.0), &vec3(0.0, -1.0, 0.0))
                .unwrap();
        let mut directional_light1 =
            DirectionalLight::new(&context, 0.3, &vec3(1.0, 1.0, 1.0), &vec3(0.0, -1.0, 0.0))
                .unwrap();
        let mut spot_light = SpotLight::new(
            &context,
            0.3,
            &vec3(1.0, 1.0, 1.0),
            &vec3(0.0, 0.0, 0.0),
            &vec3(0.0, -1.0, 0.0),
            20.0,
            0.1,
            0.001,
            0.0001,
        )
        .unwrap();

        // main loop
        let mut rotating = false;
        window
            .render_loop(move |frame_input| {
                camera.set_aspect(frame_input.viewport.aspect()).unwrap();

                for event in frame_input.events.iter() {
                    match event {
                        Event::MouseClick {
                            state,
                            button,
                            handled,
                            ..
                        } => {
                            if !handled {
                                rotating = *button == MouseButton::Left && *state == State::Pressed;
                            }
                        }
                        Event::MouseMotion { delta, handled, .. } => {
                            if !handled && rotating {
                                camera
                                    .rotate_around_with_fixed_up(
                                        &target,
                                        0.1 * delta.0 as f32,
                                        0.1 * delta.1 as f32,
                                    )
                                    .unwrap();
                            }
                        }
                        Event::MouseWheel { delta, handled, .. } => {
                            if !handled {
                                camera
                                    .zoom_towards(&target, 0.02 * delta.1 as f32, 5.0, 100.0)
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                let time = 0.001 * frame_input.accumulated_time;
                let c = time.cos() as f32;
                let s = time.sin() as f32;
                directional_light0.set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
                directional_light1.set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));
                spot_light.set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
                spot_light.set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));

                // Draw
                directional_light0
                    .generate_shadow_map(
                        &vec3(0.0, 0.0, 0.0),
                        2.0,
                        2.0,
                        20.0,
                        1024,
                        1024,
                        &[&model],
                    )
                    .unwrap();
                directional_light1
                    .generate_shadow_map(
                        &vec3(0.0, 0.0, 0.0),
                        2.0,
                        2.0,
                        20.0,
                        1024,
                        1024,
                        &[&model],
                    )
                    .unwrap();
                spot_light
                    .generate_shadow_map(15.0, 1024, &[&model])
                    .unwrap();
                Screen::write(&context, ClearState::default(), || {
                    plane.render_with_lighting(
                        RenderStates::default(),
                        frame_input.viewport,
                        &camera,
                        Some(&ambient_light),
                        &[&directional_light0, &directional_light1],
                        &[&spot_light],
                        &[],
                    )?;

                    model.render_with_lighting(
                        RenderStates::default(),
                        frame_input.viewport,
                        &camera,
                        Some(&ambient_light),
                        &[&directional_light0, &directional_light1],
                        &[&spot_light],
                        &[],
                    )?;
                    Ok(())
                })
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
    });
}
