use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let sphere = Sphere::new(
        &context,
        vec3(0.0, 2.0, 1.0),
        1.0,
        PhysicalMaterial {
            albedo: Color::RED,
            ..Default::default()
        },
    )
    .unwrap();
    let axes = Axes::new(&context, 0.1, 1.0).unwrap();

    window
        .render_loop(move |mut frame_input: FrameInput| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            Screen::write(
                &context,
                ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                || {
                    let lights = Lights {
                        directional: vec![
                            DirectionalLight::new(
                                &context,
                                1.0,
                                Color::WHITE,
                                &vec3(0.0, -0.5, -0.5),
                            )?,
                            DirectionalLight::new(
                                &context,
                                1.0,
                                Color::WHITE,
                                &vec3(0.0, 0.5, 0.5),
                            )?,
                        ],
                        ..Default::default()
                    };
                    sphere.render(&camera, &lights)?;
                    axes.render(&camera, &lights)?;
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
