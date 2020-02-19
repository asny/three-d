
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Hello, world!").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();

    let gl = window.gl();

    // Camera
    let mut camera = Camera::new_perspective(&gl, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), screen_width as f32/screen_height as f32, 0.1, 10.0);

    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0,// bottom left
        0.0,  0.5, 0.0 // top
    ];
    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0,   // bottom right
        0.0, 1.0, 0.0,   // bottom left
        0.0, 0.0, 1.0    // top
    ];

    let buffer = VertexBuffer::new_with_two_static_attributes(&gl, &positions, &colors).unwrap();
    let program = Program::from_source(&gl,
                                       include_str!("../assets/shaders/color.vert"),
                                       include_str!("../assets/shaders/color.frag")).unwrap();

    // main loop
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);
        ScreenRendertarget::write(&gl, screen_width, screen_height);
        ScreenRendertarget::clear_color_and_depth(&gl, &vec4(0.8, 0.8, 0.8, 1.0));

        program.use_attribute_vec3_float(&buffer, "position", 0).unwrap();
        program.use_attribute_vec3_float(&buffer, "color", 1).unwrap();

        program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();

        program.draw_arrays(3);

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, screen_width, screen_height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}