extern crate sdl2;
extern crate gl;
extern crate glm;

use std::process;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

pub mod dust;
use dust::*;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    #[cfg(target_os = "macos")] // Use OpenGL 4.1 since that is the newest version supported on macOS
    {
        let gl_attr = video_ctx.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
    }

    let window = video_ctx
        .window("Dust", 900, 700)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_ctx.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // set up vertex buffer object
    let positions: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        0.0,  0.5, 0.0,   // top
    ];
    let mesh = mesh::Mesh::create(positions).unwrap();

    let colors: Vec<f32> = vec![
        // positions      // colors
        1.0, 0.0, 0.0,   // bottom right
        0.0, 1.0, 0.0,   // bottom left
        0.0, 0.0, 1.0    // top
    ];

    // set up shader program
    let shader_program = program::Program::from_resource(&gl, "assets/shaders/triangle").unwrap();

    let material = material::Material::create(&shader_program).unwrap();
    let model = model::Model::create(&gl, &material, &mesh).unwrap();

    model.add_attribute("Color", &colors).unwrap();

    let camera = camera::Camera::create(&gl, glm::vec3(0.0, 0.0, 10.0)).unwrap();

    // set up shared state for window
    unsafe {
        gl.Viewport(0, 0, 900, 700); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

    // main loop
    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {
                    //material = material::Material::create(&gl).unwrap();
                    //model = model::Model::create(&gl, &material).unwrap();
                },
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw
        model.draw();

        window.gl_swap_window();
    };

    #[cfg(target_os = "emscripten")]
    {
        use emscripten::{emscripten};
        emscripten::set_main_loop_callback(main_loop);
    }

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
