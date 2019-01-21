
mod scene_objects;
mod window_handler;

use crate::window_handler::WindowHandler;
use dust::*;
use glutin::*;

fn main() {
    let mut window_handler = WindowHandler::new_default("Hello, world!");
    let (width, height) = window_handler.size();
    let gl = window_handler.gl();

    // Renderer
    let renderer = pipeline::DeferredPipeline::new(&gl, width, height, true).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let (meshes, _materials) = tobj::load_obj(&std::path::PathBuf::from("../Dust/examples/assets/models/suzanne.obj")).unwrap();
    let mesh = meshes.first().unwrap();
    let mut shaded_mesh = objects::ShadedMesh::create(&gl, &mesh.mesh.indices, &att!["position" => (mesh.mesh.positions.clone(), 3),
                                                                    "normal" => (mesh.mesh.normals.clone(), 3)]).unwrap();

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
    let plane = crate::objects::ShadedMesh::create(&gl, &plane_indices, &att!["position" => (plane_positions, 3), "normal" => (plane_normals, 3)]).unwrap();

    let mut ambient_light = crate::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut directional_light = dust::light::DirectionalLight::new(vec3(1.0, -1.0, -1.0));
    directional_light.base.color = vec3(1.0, 0.0, 0.0);
    directional_light.enable_shadows(&gl, 2.0, 10.0).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    let mut i = 0;
    loop {
        window_handler.handle_events( |event| {
            WindowHandler::handle_window_close_events(event);
            WindowHandler::handle_camera_events(event, &mut camera_handler, &mut camera);
        });

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

        renderer.copy_to_screen().unwrap();

        window_handler.swap_buffers();
    };
}
