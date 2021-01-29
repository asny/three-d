
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new("Hello, world!", 1024, 512).unwrap();
    let (width, height) = window.framebuffer_size();
    let viewport = Viewport::new(width, height);
    let gl = window.gl();

    // Camera
    let mut camera = Camera::new_perspective(&gl, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                             degrees(45.0), window.aspect(), 0.1, 10.0);

    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0,// bottom left
        0.0,  0.5, 0.0 // top
    ];
    let position_buffer = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();
    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0,   // bottom right
        0.0, 1.0, 0.0,   // bottom left
        0.0, 0.0, 1.0    // top
    ];
    let color_buffer = VertexBuffer::new_with_static_f32(&gl, &colors).unwrap();

    let program = Program::from_source(&gl,
                                       include_str!("../assets/shaders/color.vert"),
                                       include_str!("../assets/shaders/color.frag")).unwrap();

    // main loop
    let mut time = 0.0;
    window.render_loop(move |frame_input|
    {
        time += frame_input.elapsed_time as f32;
        camera.set_aspect(frame_input.aspect());

        Screen::write(&gl, Some(&vec4(0.8, 0.8, 0.8, 1.0)), Some(1.0), || {
            program.use_attribute_vec3_float(&position_buffer, "position")?;
            program.use_attribute_vec3_float(&color_buffer, "color")?;

            let world_view_projection = camera.get_projection() * camera.get_view() * Mat4::from_angle_y(radians(time * 0.005));
            program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection)?;

            program.draw_arrays(RenderStates::default(), viewport, 3);
            Ok(())
        }).unwrap();

        #[cfg(target_arch = "x86_64")]
        if let Some(ref path) = screenshot_path {
            let pixels = Screen::read_color(&gl, viewport).unwrap();
            Saver::save_pixels(path, &pixels, viewport.width, viewport.height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}