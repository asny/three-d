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
    let mut camera = Camera::new_orthographic(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        1.2,
        0.0,
        10.0,
    )
    .unwrap();

    let indices = vec![0u8, 1, 2, 2, 3, 0];
    let positions = vec![
        -2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0, 2.0, 0.0, -2.0, 2.0, 0.0,
    ];
    let mut mesh = Mesh::new(
        &context,
        &CPUMesh {
            indices: Some(Indices::U8(indices)),
            positions,
            ..Default::default()
        },
    )
    .unwrap();
    mesh.cull = CullType::Back;
    mesh.transformation = Mat4::from_scale(10.0);
    mesh.depth_test = DepthTestType::Always;
    let program =
        MeshProgram::new(&context, include_str!("../assets/shaders/mandelbrot.frag")).unwrap();

    // main loop
    let mut pick: Option<((f64, f64), Vec3)> = None;
    window
        .render_loop(move |frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseMotion {
                        delta,
                        button,
                        position,
                        ..
                    } => {
                        if *button == Some(MouseButton::Left) {
                            let speed = 0.003 * camera.position().z.abs();
                            let right = camera.right_direction();
                            let up = right.cross(camera.view_direction());
                            let delta =
                                -right * speed * delta.0 as f32 + up * speed * delta.1 as f32;
                            camera.translate(&delta).unwrap();
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
                            let pixel = (
                                (frame_input.device_pixel_ratio * position.0) as f32,
                                (frame_input.device_pixel_ratio * position.1) as f32,
                            );
                            let p = camera.pick(pixel, 10.0, &[&mesh]).unwrap();
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
                    mesh.render(&program, WriteMask::COLOR, None, &camera)?;
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
