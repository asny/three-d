
mod scene_objects;

use std::process;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use dust::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    #[cfg(target_os = "macos")] // Use OpenGL 4.1 since that is the newest version supported on macOS
    {
        let gl_attr = video_ctx.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
    }

    let width: usize = 900;
    let height: usize = 700;
    let window = video_ctx
        .window("Dust", width as u32, height as u32)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_ctx.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Screen
    let screen = screen::Screen {width, height};

    // Renderer
    let renderer = pipeline::DeferredPipeline::create(&gl, &screen, true).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), screen.aspect(), 0.1, 1000.0);

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

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

    let mut i = 0;
    // main loop
    let main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::MouseMotion {xrel, yrel, mousestate, .. } => {
                    if mousestate.left()
                    {
                        eventhandler::rotate(&mut camera, xrel, yrel);
                    }
                },
                Event::MouseWheel {y, .. } => {
                    eventhandler::zoom(&mut camera, y);
                },
                _ => {}
            }
        }

        eventhandler::rotate(&mut camera, -1, 0);

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

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
