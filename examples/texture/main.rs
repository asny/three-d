use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Texture!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    // Renderer
    let mut pipeline = PhongDeferredPipeline::new(&context).unwrap();
    let mut camera = CameraControl::new(
        Camera::new_perspective(
            &context,
            vec3(4.0, 1.5, 4.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            window.viewport().unwrap().aspect(),
            0.1,
            1000.0,
        )
        .unwrap(),
    );

    Loader::load(
        &[
            "examples/assets/PenguinBaseMesh.obj",
            "examples/assets/PenguinBaseMesh.mtl",
            "examples/assets/penguin.png",
            "examples/assets/test_texture.jpg",
            "examples/assets/skybox_evening/back.jpg",
            "examples/assets/skybox_evening/front.jpg",
            "examples/assets/skybox_evening/top.jpg",
            "examples/assets/skybox_evening/left.jpg",
            "examples/assets/skybox_evening/right.jpg",
        ],
        move |loaded| {
            let mut box_cpu_mesh = CPUMesh {
                positions: cube_positions(),
                uvs: Some(cube_uvs()),
                ..Default::default()
            };
            box_cpu_mesh.compute_normals();
            let box_texture = Texture2D::new_with_u8(
                &context,
                &loaded.image("examples/assets/test_texture.jpg").unwrap(),
            )
            .unwrap();
            let box_material = PhongMaterial {
                color_source: ColorSource::Texture(std::rc::Rc::new(box_texture)),
                ..Default::default()
            };
            let mut box_mesh = PhongMesh::new(&context, &box_cpu_mesh, &box_material).unwrap();
            box_mesh.cull = CullType::Back;

            let skybox = Skybox::new(
                &context,
                &mut loaded
                    .cube_image(
                        "examples/assets/skybox_evening/right.jpg",
                        "examples/assets/skybox_evening/left.jpg",
                        "examples/assets/skybox_evening/top.jpg",
                        "examples/assets/skybox_evening/top.jpg",
                        "examples/assets/skybox_evening/front.jpg",
                        "examples/assets/skybox_evening/back.jpg",
                    )
                    .unwrap(),
            )
            .unwrap();

            let (penguin_cpu_meshes, penguin_cpu_materials) =
                loaded.obj("examples/assets/PenguinBaseMesh.obj").unwrap();
            let penguin_cpu_material =
                PhongMaterial::new(&context, &penguin_cpu_materials[0]).unwrap();
            let mut penguin_deferred =
                PhongMesh::new(&context, &penguin_cpu_meshes[0], &penguin_cpu_material).unwrap();
            penguin_deferred.cull = CullType::Back;
            penguin_deferred.transformation = Mat4::from_translation(vec3(-0.5, 1.0, 0.0));
            let mut penguin_forward =
                PhongMesh::new(&context, &penguin_cpu_meshes[0], &penguin_cpu_material).unwrap();
            penguin_forward.cull = CullType::Back;
            penguin_forward.transformation = Mat4::from_translation(vec3(0.5, 1.0, 0.0));

            let ambient_light = AmbientLight {
                intensity: 0.4,
                color: vec3(1.0, 1.0, 1.0),
            };
            let directional_light =
                DirectionalLight::new(&context, 1.0, &vec3(1.0, 1.0, 1.0), &vec3(0.0, -1.0, -1.0))
                    .unwrap();

            let axes = Axes::new(&context, 0.1, 3.0).unwrap();
            // main loop
            let mut rotating = false;
            window
                .render_loop(move |frame_input| {
                    let mut redraw = frame_input.first_frame;
                    redraw |= camera.set_aspect(frame_input.viewport.aspect()).unwrap();

                    for event in frame_input.events.iter() {
                        match event {
                            Event::MouseClick { state, button, .. } => {
                                rotating = *button == MouseButton::Left && *state == State::Pressed;
                            }
                            Event::MouseMotion { delta, .. } => {
                                if rotating {
                                    let target = *camera.target();
                                    camera
                                        .rotate_around_with_fixed_up(
                                            &target,
                                            0.1 * delta.0 as f32,
                                            0.1 * delta.1 as f32,
                                        )
                                        .unwrap();
                                    redraw = true;
                                }
                            }
                            Event::MouseWheel { delta, .. } => {
                                let target = *camera.target();
                                camera
                                    .zoom_towards(&target, 0.1 * delta.1 as f32, 3.0, 100.0)
                                    .unwrap();
                                redraw = true;
                            }
                            _ => {}
                        }
                    }

                    // draw
                    if redraw {
                        // Geometry pass
                        pipeline
                            .geometry_pass(
                                frame_input.viewport.width,
                                frame_input.viewport.height,
                                || {
                                    box_mesh.render_geometry(
                                        RenderStates::default(),
                                        frame_input.viewport,
                                        &camera,
                                    )?;
                                    penguin_deferred.render_geometry(
                                        RenderStates::default(),
                                        frame_input.viewport,
                                        &camera,
                                    )?;
                                    Ok(())
                                },
                            )
                            .unwrap();

                        Screen::write(&context, ClearState::default(), || {
                            pipeline.light_pass(
                                frame_input.viewport,
                                &camera,
                                Some(&ambient_light),
                                &[&directional_light],
                                &[],
                                &[],
                            )?;
                            penguin_forward.render_with_lighting(
                                RenderStates::default(),
                                frame_input.viewport,
                                &camera,
                                Some(&ambient_light),
                                &[&directional_light],
                                &[],
                                &[],
                            )?;
                            axes.render(frame_input.viewport, &camera)?;
                            skybox.render(frame_input.viewport, &camera)?;
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
                            swap_buffers: redraw,
                            wait_next_event: true,
                            ..Default::default()
                        }
                    }
                })
                .unwrap();
        },
    );
}

fn cube_positions() -> Vec<f32> {
    vec![
        1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0,
        1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
    ]
}

fn cube_uvs() -> Vec<f32> {
    vec![
        1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0,
    ]
}
