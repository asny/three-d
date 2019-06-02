
use window::{event::*, Window};
use dust::*;

fn main() {
    let mut window = Window::new_default("Texture").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let renderer = DeferredPipeline::new(&gl, width, height, vec4(0.0, 0.0, 0.0, 1.0)).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), width as f32 / height as f32, 0.1, 100.0);

    let mut monkey = objects::ShadedMesh::new_from_obj_source(&gl, include_str!("assets/models/suzanne.obj").to_string()).unwrap();
    monkey.color = vec3(0.5, 1.0, 0.5);

    let ambient_light = crate::light::AmbientLight::new();
    let mut light = dust::light::DirectionalLight::new(vec3(0.0, -1.0, 0.0));
    light.base.intensity = 1.0;

    let mut fog_effect = effects::FogEffect::new(&gl).unwrap();
    fog_effect.color = vec3(0.9, 0.8, 0.8);
    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    let mut time = 0.0;
    window.render_loop(move |events, elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut camera);
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
        renderer.geometry_pass_begin().unwrap();
        let transformation = Mat4::identity();
        monkey.render(&transformation, &camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&light).unwrap();

        // Effect
        fog_effect.apply(renderer.full_screen(), time as f32, &camera, renderer.geometry_pass_position_texture(), renderer.geometry_pass_depth_texture()).unwrap();
        debug_effect.apply(renderer.full_screen(), renderer.geometry_pass_color_texture(), renderer.geometry_pass_position_texture(), renderer.geometry_pass_normal_texture(), renderer.geometry_pass_depth_texture()).unwrap();
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