
use window::{event::*, Window};
use dust::*;

fn main() {
    let mut window = Window::new_default("Texture").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.0, 0.0, 0.0, 1.0)).unwrap();

    let monkey = Mesh::new_from_obj_source(&gl, include_str!("../assets/models/suzanne.obj").to_string()).unwrap();
    let mut mesh_shader = MeshShader::new(&gl).unwrap();
    mesh_shader.color = vec3(0.5, 1.0, 0.5);

    renderer.directional_light(0).unwrap().set_direction(&vec3(0.0, -1.0, 0.0));
    renderer.directional_light(0).unwrap().set_intensity(1.0);

    let mut fog_effect = effects::FogEffect::new(&gl).unwrap();
    fog_effect.color = vec3(0.9, 0.8, 0.8);
    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    let mut time = 0.0;
    window.render_loop(move |events, elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut renderer.camera);
            match event {
                Event::Key { state, kind } => {
                    if kind == "R" && *state == State::Pressed
                    {
                        debug_effect.change_type();
                    }
                },
                _ => {}
            }
        }
        time += elapsed_time;

        // draw
        // Geometry pass
        renderer.geometry_pass(&|camera| {
            let transformation = Mat4::identity();
            mesh_shader.render(&monkey, &transformation, camera);
        }).unwrap();

        // Light pass
        renderer.light_pass().unwrap();

        // Effect
        fog_effect.apply(renderer.full_screen(), time as f32, &renderer.camera, renderer.geometry_pass_texture(), renderer.geometry_pass_depth_texture()).unwrap();
        debug_effect.apply(renderer.full_screen(), renderer.geometry_pass_texture(), renderer.geometry_pass_depth_texture()).unwrap();
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