
mod scene_objects;

use std::process;

use dust::*;
use glutin::dpi::*;
use glutin::GlContext;

fn main() {

    let width: usize = 900;
    let height: usize = 700;
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(LogicalSize::new(width as f64, height as f64));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    let gl = gl::Gl::load_with(|s| gl_window.get_proc_address(s) as *const std::os::raw::c_void);

    // Screen
    let screen = screen::Screen {width, height};

    // Renderer
    let renderer = pipeline::ForwardPipeline::create(&gl, &screen).unwrap();

    // Camera
    let camera = camera::PerspectiveCamera::new(vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), screen.aspect(), 0.1, 10.0);

    let model = scene_objects::triangle::Triangle::create(&gl);

    // main loop
    let main_loop = || {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => process::exit(1),
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        // draw
        renderer.render_pass_begin();

        model.render(&camera);

        gl_window.swap_buffers().unwrap();
    };

    renderer::set_main_loop(main_loop);
}
