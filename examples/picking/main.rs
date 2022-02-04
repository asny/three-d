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

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(0.05));
    let mut pick_mesh = Model::new_with_material(
        &context,
        &sphere,
        PhysicalMaterial {
            albedo: Color::RED,
            ..Default::default()
        },
    )
    .unwrap();

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();

    let monkey = Loading::new(
        &context,
        &["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"],
        move |context, mut loaded| {
            let (meshes, materials) = loaded.obj("examples/assets/suzanne.obj").unwrap();
            let mut monkey = Model::new_with_material(
                &context,
                &meshes[0],
                PhysicalMaterial::new(&context, &materials[0]).unwrap(),
            )
            .unwrap();
            monkey.material.render_states.cull = Cull::Back;
            Ok(monkey)
        },
    );

    // main loop
    let mut loaded = false;
    window
        .render_loop(move |mut frame_input| {
            let mut change = frame_input.first_frame;
            if !loaded && monkey.is_loaded() {
                change = true;
                loaded = true;
            }
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
                            if let Some(ref monkey) = *monkey.borrow() {
                                let monkey = monkey.as_ref().unwrap();
                                if let Some(pick) =
                                    pick(&context, &camera, pixel, &[monkey]).unwrap()
                                {
                                    pick_mesh.set_transformation(Mat4::from_translation(pick));
                                    change = true;
                                }
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
                    || {
                        if let Some(ref monkey) = *monkey.borrow() {
                            let monkey = monkey.as_ref().unwrap();
                            render_pass(&camera, &[monkey, &pick_mesh], &[&ambient, &directional])?;
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
                    swap_buffers: change,
                    ..Default::default()
                }
            }
        })
        .unwrap();
}
