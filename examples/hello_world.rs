use dust::*;
use dust::window::event::*;

fn main() {

    let mut window = window::Window::new_default("Hello, world!").unwrap();
    let (width, height) = window.framebuffer_size();

    let gl = window.gl();
    let renderer = ForwardPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
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

    let buffer = buffer::VertexBufferBuilder::new_with_vec3_vec3(&gl, positions, colors).unwrap();
    let program = program::Program::from_source(&gl,
                                                include_str!("assets/shaders/color.vert"),
                                                include_str!("assets/shaders/color.frag")).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            handle_camera_events(&event, &mut camera_handler, &mut camera);
        }

        renderer.render_pass_begin();
        program.set_used();

        buffer.bind();
        program.use_attribute_vec3_float(&buffer, "position", 0).unwrap();
        program.use_attribute_vec3_float(&buffer, "color", 1).unwrap();

        program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();

        program.draw_arrays(3);
    }).unwrap();
}

pub fn handle_camera_events(event: &Event, camera_handler: &mut dust::camerahandler::CameraHandler, camera: &mut Camera)
{
    match event {
        Event::Key {state, kind} => {
            if kind == "Tab" && *state == State::Pressed
            {
                camera_handler.next_state();
            }
        },
        Event::MouseClick {state, button, ..} => {
            if *button == MouseButton::Left
            {
                if *state == State::Pressed { camera_handler.start_rotation(); }
                else { camera_handler.end_rotation() }
            }
        },
        Event::MouseMotion {delta} => {
            camera_handler.rotate(camera, delta.0 as f32, delta.1 as f32);
        },
        Event::MouseWheel {delta} => {
            camera_handler.zoom(camera, *delta as f32);
        }
    }
}