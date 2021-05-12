use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Mandelbrot!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    // Renderer
    let mut camera = CameraControl::new(
        Camera::new_orthographic(
            &context,
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            1.2,
            1.2 * window.viewport().unwrap().aspect(),
            10.0,
        )
        .unwrap(),
    );

    let indices = vec![0, 1, 2, 2, 3, 0];
    let positions = vec![
        -2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0, 2.0, 0.0, -2.0, 2.0, 0.0,
    ];
    let mut mesh = Mesh::new(
        &context,
        &CPUMesh {
            indices: Some(indices),
            positions,
            ..Default::default()
        },
    )
    .unwrap();
    mesh.cull = CullType::Back;
    mesh.transformation = Mat4::from_scale(10.0);
    let program =
        MeshProgram::new(&context, include_str!("../assets/shaders/mandelbrot.frag")).unwrap();

    // main loop
    let mut panning = false;
    let mut pick: Option<((f64, f64), Vec3)> = None;
    window
        .render_loop(move |frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_aspect(frame_input.viewport.aspect()).unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        panning = *button == MouseButton::Left && *state == State::Pressed;
                    }
                    Event::MouseMotion {
                        delta, position, ..
                    } => {
                        if panning {
                            let speed = 0.003 * camera.position().z.abs();
                            camera
                                .pan(speed * delta.0 as f32, speed * delta.1 as f32)
                                .unwrap();
                            redraw = true;
                        }
                        if let Some((p, _)) = pick {
                            if (p.0 - position.0).abs() > 2.0 || (p.1 - position.1).abs() > 2.0 {
                                pick = None;
                            }
                        }
                    }
                    Event::MouseWheel {
                        delta, position, ..
                    } => {
                        if pick.is_none() {
                            let p = camera
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
                                    10.0,
                                    &[&mesh],
                                )
                                .unwrap();
                            pick = p.map(|pos| (*position, pos));
                        };
                        if let Some((_, pos)) = pick {
                            let distance = pos.distance(*camera.position());
                            camera
                                .zoom_towards(&pos, distance * 0.05 * delta.1 as f32, 0.00001, 10.0)
                                .unwrap();
                            redraw = true;
                        }
                    }
                    _ => {}
                }
            }

            if redraw {
                Screen::write(&context, ClearState::color(0.0, 1.0, 1.0, 1.0), || {
                    mesh.render(
                        &program,
                        RenderStates {
                            write_mask: WriteMask::COLOR,
                            depth_test: DepthTestType::Always,
                            ..Default::default()
                        },
                        frame_input.viewport,
                        &camera,
                    )
                    .unwrap();
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
}
