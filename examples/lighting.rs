extern crate sdl2;
extern crate dust;

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
    let renderer = pipeline::DeferredPipeline::create(&gl, &screen, false).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), screen.aspect());

    let mesh = gust::loader::load_obj_as_static_mesh("../Dust/examples/assets/models/suzanne.obj").unwrap();
    let monkey = objects::ShadedColoredMesh::create(&gl, &mesh);

    let plane = ::objects::ShadedColoredMesh::create(&gl, &mesh_generator::create_plane().unwrap());

    let directional_light = dust::light::DirectionalLight::new_that_cast_shadow(&gl, vec3(0.0, -1.0, 0.0), 10.0);

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

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

        // Draw
        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        monkey.render(&Mat4::identity(), &camera);
        plane.render(&(Mat4::new_translation(&vec3(0.0, -1.0, 0.0)) * Mat4::new_scaling(10.0)), &camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
