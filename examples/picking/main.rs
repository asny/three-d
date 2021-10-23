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

    let pipeline = ForwardPipeline::new(&context).unwrap();
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(4.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut sphere = CPUMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(0.05));
    let mut pick_mesh = Glue {
        geometry: Model::new(&context, &sphere).unwrap(),
        material: PhysicalMaterial {
            albedo: Color::RED,
            ..Default::default()
        },
    };

    Loader::load(
        &["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"],
        move |mut loaded| {
            let (meshes, materials) = loaded.obj("examples/assets/suzanne.obj").unwrap();
            let mut monkey = Glue {
                geometry: Model::new(&context, &meshes[0]).unwrap(),
                material: PhysicalMaterial::new(&context, &materials[0]).unwrap(),
            };
            monkey.material.opaque_render_states.cull = Cull::Back;

            let lights = Lights {
                ambient: Some(AmbientLight {
                    intensity: 0.4,
                    color: Color::WHITE,
                }),
                directional: vec![DirectionalLight::new(
                    &context,
                    2.0,
                    Color::WHITE,
                    &vec3(-1.0, -1.0, -1.0),
                )
                .unwrap()],
                ..Default::default()
            };

            // main loop
            window
                .render_loop(move |mut frame_input| {
                    let mut change = frame_input.first_frame;
                    change |= camera.set_viewport(frame_input.viewport).unwrap();

                    for event in frame_input.events.iter() {
                        match event {
                            Event::MousePress {
                                button, position, ..
                            } => {
                                if *button == MouseButton::Left {
                                    let pixel = (
                                        (frame_input.device_pixel_ratio * position.0) as f32,
                                        (frame_input.device_pixel_ratio * position.1) as f32,
                                    );
                                    if let Some(pick) =
                                        pick(&context, &camera, pixel, &[&monkey]).unwrap()
                                    {
                                        pick_mesh
                                            .geometry
                                            .set_transformation(&Mat4::from_translation(pick));
                                        change = true;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    change |= control
                        .handle_events(&mut camera, &mut frame_input.events)
                        .unwrap();

                    // draw
                    if change {
                        Screen::write(
                            &context,
                            ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0),
                            || pipeline.render_pass(&camera, &[&monkey, &pick_mesh], &lights),
                        )
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
