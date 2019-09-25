
use window::*;
use core::*;

fn main() {

    let mut window = Window::new_default("Hello, world!").unwrap();
    let (width, height) = window.framebuffer_size();

    let gl = window.gl();
    let rendertarget = ColorRendertarget::default(&gl, width, height).unwrap();

    // Camera
    let camera = Camera::new_perspective(&gl, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 10.0);

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

    let buffer = StaticVertexBuffer::new_with_vec3_vec3(&gl, &positions, &colors).unwrap();
    let program = Program::from_source(&gl,
                                       include_str!("../assets/shaders/color.vert"),
                                       include_str!("../assets/shaders/color.frag")).unwrap();

    // main loop
    window.render_loop(move |_events, _elapsed_time|
    {
        rendertarget.bind();
        rendertarget.clear(&vec4(0.8, 0.8, 0.8, 1.0));

        program.use_attribute_vec3_float(&buffer, "position", 0).unwrap();
        program.use_attribute_vec3_float(&buffer, "color", 1).unwrap();

        program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();

        program.draw_arrays(3);
    }).unwrap();
}