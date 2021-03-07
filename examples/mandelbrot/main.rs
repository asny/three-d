
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new("Mandelbrot", Some((1280, 720))).unwrap();
    let context = window.gl();

    // Renderer
    let mut camera = CameraControl::new(Camera::new_orthographic(&context, vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                1.2, 1.2*window.viewport().aspect(), 10.0).unwrap());

    let indices = vec![
        0, 1, 2, 2, 3, 0
    ];
    let positions = vec![
        -2.0, -2.0, 0.0,
        2.0, -2.0, 0.0,
        2.0, 2.0, 0.0,
        -2.0, 2.0, 0.0,
    ];
    let mesh = Mesh::new(&context, &CPUMesh {indices: Some(indices), positions, ..Default::default() }).unwrap();
    let program = MeshProgram::new(&context, include_str!("../assets/shaders/mandelbrot.frag")).unwrap();

    // main loop
    let mut panning = false;
    window.render_loop(move |frame_input|
    {
        let mut frame_output = FrameOutput::new_from_input(&frame_input);
        camera.set_aspect(frame_input.viewport.aspect()).unwrap();

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    panning = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta, ..} => {
                    if panning {
                        camera.pan(0.2 * delta.0 as f32, 0.2 * delta.1 as f32).unwrap();
                        frame_output.redraw = true;
                    }
                },
                Event::MouseWheel {delta, ..} => {
                    camera.zoom(0.05 * delta.1 as f32).unwrap();
                    frame_output.redraw = true;
                },
                _ => {}
            }
        }

        if frame_output.redraw {
            Screen::write(&context, &ClearState::color(0.0, 1.0, 1.0, 1.0), || {
                mesh.render(&program, RenderStates {cull: CullType::Back, write_mask: WriteMask::COLOR, depth_test: DepthTestType::Always, ..Default::default()},
                            frame_input.viewport, &Mat4::identity(), &camera).unwrap();
                Ok(())
            }).unwrap();
        }

        // To automatically generate screenshots of the examples, can safely be ignored.
        if args.len() > 1 {
            frame_output.screenshot = Some(args[1].clone());
            frame_output.exit = true;
        }
        frame_output
    }).unwrap();
}