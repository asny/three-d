use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Statues!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut primary_camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-200.0, 200.0, 100.0),
        vec3(0.0, 100.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    )
    .unwrap();
    // Static camera to view frustum culling in effect
    let mut secondary_camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-500.0, 700.0, 500.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(
        *primary_camera.target(),
        0.5 * primary_camera.target().distance(*primary_camera.position()),
        5.0 * primary_camera.target().distance(*primary_camera.position()),
    );

    // Models from http://texturedmesh.isti.cnr.it/
    Loader::load(
        &[
            "examples/assets/COLOMBE.obj",
            "examples/assets/COLOMBE.mtl",
            "examples/assets/COLOMBE.png",
            "examples/assets/pfboy.obj",
            "examples/assets/pfboy.mtl",
            "examples/assets/pfboy.png",
        ],
        move |mut loaded| {
            let (statue_cpu_meshes, statue_cpu_materials) =
                loaded.obj("examples/assets/COLOMBE.obj").unwrap();
            let mut statue_material =
                PhysicalMaterial::new(&context, &statue_cpu_materials[0]).unwrap();
            statue_material.opaque_render_states.cull = Cull::Back;
            let mut statue =
                Model::new_with_material(&context, &statue_cpu_meshes[0], statue_material).unwrap();

            let mut models = Vec::new();
            let scale = Mat4::from_scale(10.0);
            for i in 0..8 {
                let angle = i as f32 * 2.0 * std::f32::consts::PI / 8.0;
                let rotation = Mat4::from_angle_y(radians(0.8 * std::f32::consts::PI - angle));
                let dist = 300.0;
                let translation = Mat4::from_translation(vec3(
                    angle.cos() * dist,
                    (1.2 * std::f32::consts::PI - angle).cos() * 21.0 - 33.0,
                    angle.sin() * dist,
                ));
                statue.set_transformation(translation * scale * rotation);
                models.push(statue.clone());
            }

            let (fountain_cpu_meshes, fountain_cpu_materials) =
                loaded.obj("examples/assets/pfboy.obj").unwrap();
            let mut fountain_material =
                PhysicalMaterial::new(&context, &fountain_cpu_materials[0]).unwrap();
            fountain_material.opaque_render_states.cull = Cull::Back;
            let mut fountain =
                Model::new_with_material(&context, &fountain_cpu_meshes[0], fountain_material)
                    .unwrap();
            fountain.set_transformation(Mat4::from_angle_x(degrees(-90.0)));
            models.push(fountain);

            let mut lights = Lights {
                ambient: Some(AmbientLight {
                    intensity: 0.4,
                    color: Color::WHITE,
                }),
                directional: vec![DirectionalLight::new(
                    &context,
                    10.0,
                    Color::new_opaque(204, 178, 127),
                    &vec3(0.0, -1.0, -1.0),
                )
                .unwrap()],
                ..Default::default()
            };
            lights.directional[0]
                .generate_shadow_map(1000.0, 1024, 1024, &models)
                .unwrap();

            // main loop
            let mut is_primary_camera = true;
            window
                .render_loop(move |mut frame_input| {
                    let mut redraw = frame_input.first_frame;
                    redraw |= primary_camera.set_viewport(frame_input.viewport).unwrap();
                    redraw |= secondary_camera.set_viewport(frame_input.viewport).unwrap();
                    redraw |= control
                        .handle_events(&mut primary_camera, &mut frame_input.events)
                        .unwrap();

                    for event in frame_input.events.iter() {
                        match event {
                            Event::KeyPress { kind, .. } => {
                                if *kind == Key::C {
                                    is_primary_camera = !is_primary_camera;
                                    redraw = true;
                                }
                            }
                            _ => {}
                        }
                    }

                    // draw
                    if redraw {
                        Screen::write(
                            &context,
                            ClearState::color_and_depth(0.8, 0.8, 0.7, 1.0, 1.0),
                            || {
                                for model in models
                                    .iter()
                                    .filter(|o| primary_camera.in_frustum(&o.aabb()))
                                {
                                    model.render(
                                        if is_primary_camera {
                                            &primary_camera
                                        } else {
                                            &secondary_camera
                                        },
                                        &lights,
                                    )?;
                                }
                                Ok(())
                            },
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
