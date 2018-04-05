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

    #[cfg(not(target_os = "emscripten"))]
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

    // set up shader program
    let mut shader_program = program::Program::from_resource(
        &gl, "assets/shaders/triangle"
        ).unwrap();

    shader_program.set_used();

    // set up vertex buffer object
    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0,   1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,   0.0, 0.0, 1.0    // top
    ];

    // set up vertex array object
    let attribute = attribute::Attribute::create(&gl).unwrap();
    attribute.populate(vertices);

    let model = model::Model::create(&gl).unwrap();

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, attribute.id());
        use std::ffi::{CString};
        let pos_location = gl.GetAttribLocation(shader_program.id(), CString::new("Position").unwrap().as_ptr()) as gl::types::GLuint;
        gl.EnableVertexAttribArray(pos_location);
        gl.VertexAttribPointer(
            pos_location, // index of the generic vertex attribute
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        let color_location = gl.GetAttribLocation(shader_program.id(), CString::new("Color").unwrap().as_ptr()) as gl::types::GLuint;
        gl.EnableVertexAttribArray(color_location);
        gl.VertexAttribPointer(
            color_location, // index of the generic vertex attribute
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

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
                    shader_program = program::Program::from_resource(
                        &gl, "assets/shaders/triangle"
                        ).unwrap();

                    shader_program.set_used();
                },
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle
        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(model.id());
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

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
