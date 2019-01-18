
mod scene_objects;
mod window_handler;

use dust::*;

fn main() {

    let mut window_handler = window_handler::WindowHandler::new_default("Hello, world!");
    let (width, height) = window_handler.size();

    let gl = gl::Gl::load_with(|s| window_handler.get_proc_address(s));

    // Screen
    let screen = screen::Screen {width, height};

    // Renderer
    let renderer = pipeline::ForwardPipeline::create(&gl, &screen).unwrap();

    // Camera
    let camera = camera::PerspectiveCamera::new(vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), screen.aspect(), 0.1, 10.0);

    let model = scene_objects::triangle::Triangle::create(&gl);

    // main loop
    loop {
        window_handler.handle_events();

        // draw
        renderer.render_pass_begin();

        model.render(&camera);

        window_handler.swap_buffers();
    };
}
