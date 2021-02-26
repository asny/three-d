
use three_d::core::*;
use three_d::Mesh;
use three_d::MeshProgram;
use three_d::CPUMesh;
use three_d::window::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let window = Window::new("Mandelbrot", None).unwrap();
    let context = window.gl();

    // Renderer
    let mut camera = Camera::new_orthographic(&context, vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                1.2, 1.2*window.viewport().aspect(), 10.0);

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
        camera.set_aspect(frame_input.viewport.aspect());

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    panning = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta, ..} => {
                    if panning {
                        camera.pan(0.2 * delta.0 as f32, 0.2 * delta.1 as f32);
                    }
                },
                Event::MouseWheel {delta, ..} => {
                    camera.zoom(0.05 * *delta as f32);
                },
                _ => {}
            }
        }

        Screen::write(&context, Some(&vec4(0.0, 1.0, 1.0, 1.0)), None, || {
            mesh.render(&program, RenderStates {cull: CullType::Back, depth_mask: false, depth_test: DepthTestType::Always, ..Default::default()},
                        frame_input.viewport, &Mat4::identity(), &camera).unwrap();
            Ok(())
        }).unwrap();

        #[cfg(target_arch = "x86_64")]
        if let Some(ref path) = screenshot_path {
            use three_d::io::*;
            let pixels = Screen::read_color(&context, frame_input.viewport).unwrap();
            Saver::save_pixels(path, &pixels, frame_input.viewport.width, frame_input.viewport.height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}