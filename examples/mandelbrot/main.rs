use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new("Mandelbrot", Some((1280, 720))).unwrap();
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
    let mesh = Mesh::new(
        &context,
        &CPUMesh {
            indices: Some(indices),
            positions,
            ..Default::default()
        },
    )
    .unwrap();
    let program =
        MeshProgram::new(&context, include_str!("../assets/shaders/mandelbrot.frag")).unwrap();

    // main loop
    let mut panning = false;
    window
        .render_loop(move |frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_aspect(frame_input.viewport.aspect()).unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        panning = *button == MouseButton::Left && *state == State::Pressed;
                    }
                    Event::MouseMotion { delta, .. } => {
                        if panning {
                            camera
                                .pan(0.2 * delta.0 as f32, 0.2 * delta.1 as f32)
                                .unwrap();
                            redraw = true;
                        }
                    }
                    Event::MouseWheel { delta, .. } => {
                        camera.zoom(0.05 * delta.1 as f32).unwrap();
                        redraw = true;
                    }
                    _ => {}
                }
            }

            if redraw {
                Screen::write(&context, &ClearState::color(0.0, 1.0, 1.0, 1.0), || {
                    mesh.render(
                        &program,
                        RenderStates {
                            cull: CullType::Back,
                            write_mask: WriteMask::COLOR,
                            depth_test: DepthTestType::Always,
                            ..Default::default()
                        },
                        frame_input.viewport,
                        &Mat4::identity(),
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
