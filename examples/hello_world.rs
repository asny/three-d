extern crate sdl2;
extern crate dust;

mod color_material;

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

    let width = 900;
    let height = 700;
    let window = video_ctx
        .window("Dust", width, height)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_ctx.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Scene
    let mut scene = scene::Scene::create().unwrap();

    // Camera
    let camera = camera::Camera::create(&gl, glm::vec3(0.0, 0.0, 2.0), glm::vec3(0.0, 0.0, 0.0), width, height).unwrap();

    let positions: Vec<glm::Vec3> = vec![
        glm::vec3(0.5, -0.5, 0.0), // bottom right
        glm::vec3(-0.5, -0.5, 0.0),// bottom left
        glm::vec3(0.0,  0.5, 0.0) // top
    ];
    let colors: Vec<glm::Vec3> = vec![
        glm::vec3(1.0, 0.0, 0.0),   // bottom right
        glm::vec3(0.0, 1.0, 0.0),   // bottom left
        glm::vec3(0.0, 0.0, 1.0)    // top
    ];
    let mut mesh = mesh::Mesh::create(&vec![0, 1, 2], positions).unwrap();
    mesh.add_custom_attribute("Color", colors).unwrap();
    let material = color_material::ColorMaterial::create(&gl).unwrap();
    scene.add_model(&gl, mesh, material).unwrap();

    unsafe {
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

    // main loop
    let main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                _ => {}
            }
        }

        // draw
        camera.draw(&scene).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
