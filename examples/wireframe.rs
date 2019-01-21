
mod scene_objects;
mod window_handler;

use crate::window_handler::WindowHandler;
use dust::*;

fn main() {
    let mut window_handler = WindowHandler::new_default("Wireframe");
    let (width, height) = window_handler.size();
    let gl = window_handler.gl();

    // Renderer
    let renderer = pipeline::DeferredPipeline::new(&gl, width, height, false).unwrap();
    let mirror_renderer = pipeline::DeferredPipeline::new(&gl, width/2, height/2, true).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 1.0, 0.0),
                                                    vec3(0.0, 1.0, 0.0),degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    // Objects
    let (meshes, _materials) = tobj::load_obj(&std::path::PathBuf::from("../Dust/examples/assets/models/suzanne.obj")).unwrap();
    let mesh = meshes.first().unwrap();
    let mut positions = mesh.mesh.positions.clone();
    for i in 0..positions.len() {
        if i%3 == 1 {
            positions[i] += 2.0;
        }
    }

    let mut wireframe = crate::objects::Wireframe::create(&gl, &mesh.mesh.indices, &positions, 0.015);
    wireframe.set_parameters(0.8, 0.2, 5.0);

    let mut model = objects::ShadedMesh::create(&gl, &mesh.mesh.indices, &att!["position" => (positions, 3),
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
    let mut plane = crate::objects::ShadedMesh::create(&gl, &plane_indices, &att!["position" => (plane_positions, 3), "normal" => (plane_normals, 3)]).unwrap();
    plane.diffuse_intensity = 0.2;
    plane.specular_intensity = 0.4;
    plane.specular_power = 20.0;

    let mut ambient_light = crate::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut light1 = dust::light::SpotLight::new(vec3(5.0, 5.0, 5.0), vec3(-1.0, -1.0, -1.0));
    light1.enable_shadows(&gl, 20.0).unwrap();
    light1.base.intensity = 0.5;

    let mut light2 = dust::light::SpotLight::new(vec3(-5.0, 5.0, 5.0), vec3(1.0, -1.0, -1.0));
    light2.enable_shadows(&gl, 20.0).unwrap();
    light2.base.intensity = 0.5;

    let mut light3 = dust::light::SpotLight::new(vec3(-5.0, 5.0, -5.0), vec3(1.0, -1.0, 1.0));
    light3.enable_shadows(&gl, 20.0).unwrap();
    light3.base.intensity = 0.5;

    let mut light4 = dust::light::SpotLight::new(vec3(5.0, 5.0, -5.0), vec3(-1.0, -1.0, 1.0));
    light4.enable_shadows(&gl, 20.0).unwrap();
    light4.base.intensity = 0.5;

    // Mirror
    let mirror_program = program::Program::from_resource(&gl, "../Dust/examples/assets/shaders/copy",
                                                                 "../Dust/examples/assets/shaders/mirror").unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    loop {
        window_handler.handle_events( |event| {
            WindowHandler::handle_window_close_events(event);
            WindowHandler::handle_camera_events(event, &mut camera_handler, &mut camera);
        });

        // Draw
        let render_scene = |camera: &Camera| {
            //model.render(&Mat4::identity(), camera);
            wireframe.render(camera);
        };

        // Shadow pass
        light1.shadow_cast_begin().unwrap();
        render_scene(light1.shadow_camera().unwrap());

        light2.shadow_cast_begin().unwrap();
        render_scene(light2.shadow_camera().unwrap());

        light3.shadow_cast_begin().unwrap();
        render_scene(light3.shadow_camera().unwrap());

        light4.shadow_cast_begin().unwrap();
        render_scene(light4.shadow_camera().unwrap());

        // Mirror pass
        camera.mirror_in_xz_plane();

        // Mirror pass (Geometry pass)
        mirror_renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);

        // Mirror pass (Light pass)
        mirror_renderer.light_pass_begin(&camera).unwrap();
        mirror_renderer.shine_ambient_light(&ambient_light).unwrap();
        mirror_renderer.shine_spot_light(&light1).unwrap();
        mirror_renderer.shine_spot_light(&light2).unwrap();
        mirror_renderer.shine_spot_light(&light3).unwrap();
        mirror_renderer.shine_spot_light(&light4).unwrap();

        camera.mirror_in_xz_plane();

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);
        plane.render(&Mat4::from_scale(100.0), &camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_spot_light(&light1).unwrap();
        renderer.shine_spot_light(&light2).unwrap();
        renderer.shine_spot_light(&light3).unwrap();
        renderer.shine_spot_light(&light4).unwrap();

        // Blend with the result of the mirror pass
        state::blend(&gl,state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);
        state::depth_write(&gl,false);
        state::depth_test(&gl, state::DepthTestType::NONE);
        state::cull(&gl,state::CullType::BACK);

        mirror_renderer.light_pass_color_texture().unwrap().bind(0);
        mirror_program.add_uniform_int("colorMap", &0).unwrap();
        full_screen_quad::render(&gl, &mirror_program);

        window_handler.swap_buffers();
    };
}
