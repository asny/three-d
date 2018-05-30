extern crate sdl2;
extern crate dust;

mod scene_objects;

use std::process;
use std::rc::Rc;

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

    // Scene
    let mut scene = scene::Scene::create();

    // Camera
    let mut camera = camera::Camera::create(&gl, glm::vec3(5.0, 5.0, 5.0), glm::vec3(0.0, 0.0, 0.0), width, height).unwrap();

    // Add model
    let colors: Vec<glm::Vec3> = vec![
        glm::vec3(1.0, 0.0, 0.0),   // bottom right
        glm::vec3(0.0, 1.0, 0.0),   // bottom left
        glm::vec3(0.0, 0.0, 1.0),    // top
        glm::vec3(0.0, 1.0, 0.0),   // bottom left
        glm::vec3(1.0, 0.0, 0.0),   // bottom right
        glm::vec3(0.0, 1.0, 0.0),   // bottom left
        glm::vec3(1.0, 0.0, 0.0),   // bottom right
        glm::vec3(0.0, 0.0, 1.0),    // top
    ];

    let mut mesh = gust::loader::load_obj("/examples/assets/models/box.obj").unwrap();
    mesh.add_custom_vec3_attribute("Color", colors).unwrap();
    let material = scene_objects::texture_material::TextureMaterial::create(&gl).unwrap();
    let model = model::Model::create(&gl, mesh, material).unwrap();
    scene.add_model(model);

    let light = dust::light::DirectionalLight::create(&gl, glm::vec3(0.0, -1.0, 0.0)).unwrap();
    scene.add_light(Rc::from(light));

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

        // draw
        camera.draw(&scene).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
