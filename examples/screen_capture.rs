
use window::{event::*, Window};
use dust::*;

fn main() {
    let mut window = Window::new_default("Screen capture").unwrap();
    let (width, height) = window.size();
    let gl = window.gl();

    // Renderer
    let renderer = pipeline::DeferredPipeline::new(&gl, width, height, false).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let shaded_mesh = objects::ShadedMesh::new_from_obj_source(&gl, include_str!("assets/models/suzanne.obj").to_string()).unwrap();

    let plane_positions: Vec<f32> = vec![
        -1.0, 0.0, -1.0,
        1.0, 0.0, -1.0,
        1.0, 0.0, 1.0,
        -1.0, 0.0, 1.0
    ];
    let plane_normals: Vec<f32> = vec![
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0
    ];
    let plane_indices: Vec<u32> = vec![
        0, 2, 1,
        0, 3, 2,
    ];
    let plane = crate::objects::ShadedMesh::new(&gl, &plane_indices, &att!["position" => (plane_positions, 3), "normal" => (plane_normals, 3)]).unwrap();

    let mut ambient_light = crate::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut directional_light = dust::light::DirectionalLight::new(vec3(1.0, -1.0, -1.0));
    directional_light.base.color = vec3(1.0, 0.0, 0.0);
    directional_light.enable_shadows(&gl, 2.0, 10.0).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    let mut i = 0;
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut camera);
        }

        // Draw
        let render_scene = |camera: &Camera| {
            shaded_mesh.render(&Mat4::identity(), camera);
        };

        // Shadow pass
        directional_light.shadow_cast_begin().unwrap();
        render_scene(directional_light.shadow_camera().unwrap());

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);
        plane.render(&(Mat4::from_translation(vec3(0.0, -1.0, 0.0)) * Mat4::from_scale(10.0)), &camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();

        renderer.save_screenshot(&format!("image{}.png", i)).unwrap();
        i = i+1;

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
        Event::MouseClick {state, button} => {
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