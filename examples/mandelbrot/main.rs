
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Mandelbrot").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut camera = Camera::new_orthographic(&gl, vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                4.0, 4.0*height as f32/width as f32, 10.0);

    let indices = vec![
        0, 1, 2, 2, 3, 0
    ];
    let index_buffer = ElementBuffer::new_with_u32(&gl, &indices).unwrap();

    let positions = vec![
        -2.0, -2.0, 0.0,
        2.0, -2.0, 0.0,
        2.0, 2.0, 0.0,
        -2.0, 2.0, 0.0,
    ];
    let position_buffer = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();

    let program = Program::from_source(&gl,
                                       include_str!("../assets/shaders/mandelbrot.vert"),
                                       include_str!("../assets/shaders/mandelbrot.frag")).unwrap();

    // main loop
    let mut panning = false;
    window.render_loop(move |frame_input|
    {
        camera.set_aspect(frame_input.screen_width as f32 / frame_input.screen_height as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    panning = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if panning {
                        camera.pan(delta.0 as f32, delta.1 as f32);
                    }
                },
                Event::MouseWheel {delta} => {
                    camera.zoom(*delta as f32);
                },
                _ => {}
            }
        }

        Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.8, 0.8, 0.8, 1.0)), Some(1.0), || {
            program.use_attribute_vec3_float(&position_buffer, "position")?;

            program.add_uniform_mat4("modelMatrix", &Mat4::identity())?;
            program.use_uniform_block(camera.matrix_buffer(), "Camera");

            program.draw_elements(&index_buffer);
            Ok(())
        }).unwrap();

        #[cfg(target_arch = "x86_64")]
        if let Some(ref path) = screenshot_path {
            let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
            Saver::save_pixels(path, &pixels, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}