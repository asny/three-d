
use window::{event::*, Window};
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Texture").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.0, 0.0, 0.0, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);
    camera.enable_matrix_buffer(&gl);

    let mut monkey = Mesh::new_from_obj_source(&gl, include_str!("../assets/models/suzanne.obj").to_string()).unwrap().pop().unwrap();
    monkey.color = vec3(0.5, 1.0, 0.5);

    renderer.directional_light(0).unwrap().set_direction(&vec3(0.0, -1.0, 0.0));
    renderer.directional_light(0).unwrap().set_intensity(1.0);

    let mut fog_effect = effects::FogEffect::new(&gl).unwrap();
    fog_effect.color = vec3(0.8, 0.8, 0.8);
    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    let mut time = 0.0;
    window.render_loop(move |frame_input|
    {
        camera.set_perspective_projection(degrees(45.0), frame_input.screen_width as f32 / frame_input.screen_height as f32, 0.1, 1000.0);

        for event in frame_input.events {
            handle_camera_events(&event, &mut camera_handler, &mut camera);
            match event {
                Event::Key { ref state, ref kind } => {
                    if kind == "R" && *state == State::Pressed
                    {
                        debug_effect.change_type();
                    }
                },
                _ => {}
            }
        }
        time += frame_input.elapsed_time;

        // draw
        // Geometry pass
        renderer.geometry_pass(&|| {
            let transformation = Mat4::identity();
            monkey.render(&transformation, &camera);
        }).unwrap();

        // Light pass
        renderer.light_pass(&camera).unwrap();

        // Effect
        fog_effect.apply(time as f32, &camera, renderer.geometry_pass_depth_texture()).unwrap();
        debug_effect.apply(&camera, renderer.geometry_pass_texture(), renderer.geometry_pass_depth_texture()).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, width, height).unwrap();
            std::process::exit(1);
        }
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