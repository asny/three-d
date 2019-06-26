
use window::{event::*, Window};
use dust::*;

fn main() {
    let mut window = Window::new_default("Wireframe").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    let mut mirror_renderer = DeferredPipeline::new(&gl, width/2, height/2, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    mirror_renderer.camera.mirror_in_xz_plane();
    let light_pass_rendertarget = rendertarget::ColorRendertarget::new(&gl, width/2, height/2, 1, false).unwrap();

    // Objects
    let obj_file = include_str!("assets/models/suzanne.obj").to_string();
    let mut wireframe = objects::Wireframe::new_from_obj_source(&gl, obj_file.clone(), 0.015, &vec3(0.0, 2.0, 0.0));
    wireframe.set_parameters(0.8, 0.2, 5.0);

    let model = objects::ShadedMesh::new_from_obj_source(&gl, obj_file).unwrap();

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
    let mut plane = crate::objects::ShadedMesh::new(&gl, &plane_indices, &plane_positions, &plane_normals).unwrap();
    plane.diffuse_intensity = 0.2;
    plane.specular_intensity = 0.4;
    plane.specular_power = 20.0;

    let mut light = renderer.spot_light(0).unwrap();
    light.set_intensity(0.5);
    light.set_position(&vec3(5.0, 5.0, 5.0));
    light.set_direction(&vec3(-1.0, -1.0, -1.0));
    light.enable_shadows();

    light = renderer.spot_light(1).unwrap();
    light.set_intensity(0.5);
    light.set_position(&vec3(-5.0, 5.0, 5.0));
    light.set_direction(&vec3(1.0, -1.0, -1.0));
    light.enable_shadows();

    light = renderer.spot_light(2).unwrap();
    light.set_intensity(0.5);
    light.set_position(&vec3(-5.0, 5.0, -5.0));
    light.set_direction(&vec3(1.0, -1.0, 1.0));
    light.enable_shadows();

    light = renderer.spot_light(3).unwrap();
    light.set_intensity(0.5);
    light.set_position(&vec3(5.0, 5.0, -5.0));
    light.set_direction(&vec3(-1.0, -1.0, 1.0));
    light.enable_shadows();

    // Mirror
    let mirror_program = program::Program::from_source(&gl,
                                                    include_str!("assets/shaders/copy.vert"),
                                                    include_str!("assets/shaders/mirror.frag")).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut renderer.camera);
        }

        mirror_renderer.camera.set_view(*renderer.camera.position(), *renderer.camera.target(), *renderer.camera.up());
        mirror_renderer.camera.mirror_in_xz_plane();

        // Draw
        let render_scene = |camera: &Camera| {
            model.render(&Mat4::from_translation(vec3(0.0, 2.0, 0.0)), camera);
            wireframe.render(camera);
        };

        // Shadow pass
        renderer.shadow_pass(&render_scene);

        // Mirror pass (Geometry pass)
        mirror_renderer.geometry_pass(&render_scene).unwrap();

        // Mirror pass (Light pass)
        mirror_renderer.light_pass_render_to(&light_pass_rendertarget).unwrap();

        // Geometry pass
        renderer.geometry_pass(&|camera| {
            render_scene(camera);
            plane.render(&Mat4::from_scale(100.0), camera);
        }).unwrap();

        // Light pass
        renderer.light_pass().unwrap();

        // Blend with the result of the mirror pass
        state::blend(&gl,state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);
        state::depth_write(&gl,false);
        state::depth_test(&gl, state::DepthTestType::NONE);
        state::cull(&gl,state::CullType::BACK);

        mirror_program.use_texture(&light_pass_rendertarget.targets[0], "colorMap").unwrap();
        renderer.full_screen().render(&mirror_program);
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