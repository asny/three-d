use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Picking!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    // Renderer
    let mut camera = CameraControl::new(
        Camera::new_perspective(
            &context,
            vec3(4.0, 4.0, 5.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            window.viewport().unwrap().aspect(),
            0.1,
            1000.0,
        )
        .unwrap(),
    );

    let mut pick_mesh = PhongMesh::new(
        &context,
        &CPUMesh::sphere(0.05),
        &PhongMaterial::new(
            &context,
            &CPUMaterial {
                color: Some((1.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
        )
        .unwrap(),
    )
    .unwrap();

    Loader::load(
        &["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"],
        move |loaded| {
            let (meshes, mut materials) = loaded.obj("examples/assets/suzanne.obj").unwrap();
            materials[0].color = Some((0.5, 1.0, 0.5, 1.0));
            let mut monkey = PhongMesh::new(
                &context,
                &meshes[0],
                &PhongMaterial::new(&context, &materials[0]).unwrap(),
            )
            .unwrap();
            monkey.cull = CullType::Back;

            let ambient_light = AmbientLight {
                intensity: 0.2,
                color: vec3(1.0, 1.0, 1.0),
            };
            let directional_light =
                DirectionalLight::new(&context, 0.5, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0))
                    .unwrap();

            // main loop
            let mut rotating = false;
            window
                .render_loop(move |frame_input| {
                    let mut change = frame_input.first_frame;
                    change |= camera.set_aspect(frame_input.viewport.aspect()).unwrap();

                    for event in frame_input.events.iter() {
                        match event {
                            Event::MouseClick {
                                state,
                                button,
                                position,
                                ..
                            } => {
                                rotating = *button == MouseButton::Left && *state == State::Pressed;
                                if *button == MouseButton::Left && *state == State::Pressed {
                                    if let Some(pick) = camera
                                        .pick(
                                            (
                                                ((position.0 * frame_input.device_pixel_ratio
                                                    - frame_input.viewport.x as f64)
                                                    / frame_input.viewport.width as f64)
                                                    as f32,
                                                ((position.1 * frame_input.device_pixel_ratio
                                                    - frame_input.viewport.y as f64)
                                                    / frame_input.viewport.height as f64)
                                                    as f32,
                                            ),
                                            100.0,
                                            &[&monkey],
                                        )
                                        .unwrap()
                                    {
                                        pick_mesh.transformation = Mat4::from_translation(pick);
                                        change = true;
                                    }
                                }
                            }
                            Event::MouseMotion { delta, .. } => {
                                if rotating {
                                    let target = *camera.target();
                                    camera
                                        .rotate_around(
                                            &target,
                                            0.1 * delta.0 as f32,
                                            0.1 * delta.1 as f32,
                                        )
                                        .unwrap();
                                    change = true;
                                }
                            }
                            Event::MouseWheel { delta, .. } => {
                                let target = *camera.target();
                                camera
                                    .zoom_towards(&target, 0.02 * delta.1 as f32, 5.0, 100.0)
                                    .unwrap();
                                change = true;
                            }
                            _ => {}
                        }
                    }

                    // draw
                    if change {
                        Screen::write(&context, ClearState::default(), || {
                            monkey.render_with_lighting(
                                RenderStates {
                                    depth_test: DepthTestType::LessOrEqual,
                                    ..Default::default()
                                },
                                frame_input.viewport,
                                &camera,
                                Some(&ambient_light),
                                &[&directional_light],
                                &[],
                                &[],
                            )?;
                            pick_mesh.render_with_lighting(
                                RenderStates::default(),
                                frame_input.viewport,
                                &camera,
                                Some(&ambient_light),
                                &[&directional_light],
                                &[],
                                &[],
                            )?;
                            Ok(())
                        })
                        .unwrap();
                    }

                    if args.len() > 1 {
                        // To automatically generate screenshots of the examples, can safely be ignored.
                        FrameOutput {
                            screenshot: Some(args[1].clone().into()),
                            exit: true,
                            ..Default::default()
                        }
                    } else {
                        FrameOutput {
                            swap_buffers: change,
                            ..Default::default()
                        }
                    }
                })
                .unwrap();
        },
    );
}
