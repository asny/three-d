use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Fog!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let target = vec3(0.0, 0.0, 0.0);
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(4.0, 4.0, 5.0),
        target,
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();

    Loader::load(
        &[
            "examples/assets/suzanne.obj",
            "examples/assets/suzanne.mtl",
            "examples/assets/skybox_evening/back.jpg",
            "examples/assets/skybox_evening/front.jpg",
            "examples/assets/skybox_evening/top.jpg",
            "examples/assets/skybox_evening/left.jpg",
            "examples/assets/skybox_evening/right.jpg",
        ],
        move |mut loaded| {
            let (meshes, mut materials) = loaded.obj("examples/assets/suzanne.obj").unwrap();
            materials[0].color = Some((0.5, 1.0, 0.5, 1.0));
            let mut monkey = Mesh::new_with_material(
                &context,
                &meshes[0],
                &Material::new(&context, &materials[0]).unwrap(),
            )
            .unwrap();
            monkey.cull = CullType::Back;

            let ambient_light = AmbientLight {
                intensity: 0.4,
                color: vec3(1.0, 1.0, 1.0),
            };
            let directional_light =
                DirectionalLight::new(&context, 2.0, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0))
                    .unwrap();

            // Fog
            let mut fog_effect = FogEffect::new(&context).unwrap();
            fog_effect.color = vec3(0.8, 0.8, 0.8);
            let mut fog_enabled = true;

            // Skybox
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

            // main loop
            let mut depth_texture = None;
            window
                .render_loop(move |frame_input| {
                    let mut change = frame_input.first_frame;
                    change |= camera.set_viewport(frame_input.viewport).unwrap();
                    if change {
                        depth_texture = Some(
                            DepthTargetTexture2D::new(
                                &context,
                                frame_input.viewport.width,
                                frame_input.viewport.height,
                                Wrapping::ClampToEdge,
                                Wrapping::ClampToEdge,
                                DepthFormat::Depth32F,
                            )
                            .unwrap(),
                        );
                    }

                    for event in frame_input.events.iter() {
                        match event {
                            Event::MouseMotion { delta, button, .. } => {
                                if *button == Some(MouseButton::Left) {
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
                                camera
                                    .zoom_towards(&target, 0.02 * delta.1 as f32, 5.0, 100.0)
                                    .unwrap();
                                change = true;
                            }
                            Event::KeyPress { kind, .. } => {
                                if *kind == Key::F {
                                    fog_enabled = !fog_enabled;
                                    change = true;
                                    println!("Fog: {:?}", fog_enabled);
                                }
                            }
                            _ => {}
                        }
                    }

                    // draw
                    if change {
                        depth_texture
                            .as_ref()
                            .unwrap()
                            .write(Some(1.0), &|| {
                                monkey.render_depth(RenderStates::default(), &camera)?;
                                Ok(())
                            })
                            .unwrap();
                    }

                    Screen::write(&context, ClearState::default(), || {
                        monkey.render_with_lighting(
                            RenderStates {
                                depth_test: DepthTestType::LessOrEqual,
                                ..Default::default()
                            },
                            &camera,
                            Some(&ambient_light),
                            &[&directional_light],
                            &[],
                            &[],
                        )?;
                        skybox.render(&camera)?;
                        if fog_enabled {
                            fog_effect.apply(
                                &camera,
                                depth_texture.as_ref().unwrap(),
                                frame_input.accumulated_time as f32,
                            )?;
                        }
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
        },
    );
}
