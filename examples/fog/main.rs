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
    let mut control = FlyControl::new(0.05);

    let monkey = Loading::new(
        &context,
        &["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"],
        move |context, mut loaded| {
            let (meshes, materials) = loaded.obj("suzanne.obj")?;
            let mut monkey_material = PhysicalMaterial::new(&context, &materials[0])?;
            monkey_material.opaque_render_states.cull = Cull::Back;
            Model::new_with_material(&context, &meshes[0], monkey_material)
        },
    );

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();

    // Fog
    let mut fog_effect = FogEffect::new(&context).unwrap();
    fog_effect.color = vec3(0.8, 0.8, 0.8);
    let mut fog_enabled = true;

    // main loop
    let mut depth_texture = None;
    let mut loaded = false;
    window
        .render_loop(move |mut frame_input| {
            let mut change = frame_input.first_frame;
            change |= camera.set_viewport(frame_input.viewport).unwrap();
            change |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();
            if !loaded && monkey.is_loaded() {
                change = true;
                loaded = true;
            }

            for event in frame_input.events.iter() {
                match event {
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
            if change && fog_enabled {
                if let Some(ref monkey) = *monkey.borrow() {
                    depth_texture = Some(
                        pipeline
                            .depth_pass_texture(&camera, &[monkey.as_ref().unwrap()])
                            .unwrap(),
                    );
                }
            }

            Screen::write(&context, ClearState::default(), || {
                if let Some(ref monkey) = *monkey.borrow() {
                    monkey
                        .as_ref()
                        .unwrap()
                        .render(&camera, &[&ambient, &directional])?;
                }
                if fog_enabled {
                    if let Some(ref depth_texture) = depth_texture {
                        fog_effect.apply(
                            &camera,
                            depth_texture,
                            frame_input.accumulated_time as f32,
                        )?;
                    }
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
}
