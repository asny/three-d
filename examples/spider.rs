extern crate sdl2;
extern crate dust;

mod scene_objects;

use std::process;
use std::time::Instant;

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
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),screen.aspect(), 0.1, 1000.0);

    // Models
    let mut environment = scene_objects::environment::Environment::create(&gl);
    let mut spider = scene_objects::spider::Spider::create(&gl).unwrap();

    // Lights
    let directional_light = dust::light::DirectionalLight::new(vec3(0.0, -1.0, 0.0));

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

    let mut camerahandler = camerahandler::CameraHandler::create();
    let mut now = Instant::now();
    let mut time = 0.0;
    // main loop
    let main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    spider.is_moving_forward = true;
                },
                Event::KeyUp {keycode: Some(Keycode::W), ..} => {
                    spider.is_moving_forward = false;
                },
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    spider.is_rotating_right = true;
                },
                Event::KeyUp {keycode: Some(Keycode::D), ..} => {
                    spider.is_rotating_right = false;
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    spider.is_rotating_left = true;
                },
                Event::KeyUp {keycode: Some(Keycode::A), ..} => {
                    spider.is_rotating_left = false;
                },
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    spider.is_moving_backward = true;
                },
                Event::KeyUp {keycode: Some(Keycode::S), ..} => {
                    spider.is_moving_backward = false;
                },
                Event::MouseMotion {xrel, yrel, mousestate, .. } => {
                    if mousestate.left()
                    {
                        camerahandler.rotate(&mut camera, xrel, yrel);
                    }
                },
                Event::MouseWheel {y, .. } => {
                    camerahandler.zoom(&mut camera, y);
                },
                Event::KeyDown {keycode: Some(Keycode::Tab), ..} => {
                    camerahandler.next_state();
                },
                _ => {}
            }
        }

        let new_now = Instant::now();
        let elapsed_time = 0.000000001 * new_now.duration_since(now).subsec_nanos() as f32;
        now = new_now;
        time += elapsed_time;

        // Update
        spider.update(elapsed_time, &environment);
        let spider_pos = spider.get_position(&environment);
        camerahandler.translate(&mut camera, &spider_pos, &spider.get_view_direction(&environment));
        environment.set_position(&spider_pos);

        // Draw
        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        let transformation = Mat4::identity();
        environment.render_opague(&camera).unwrap();
        spider.render(&camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();
        renderer.copy_to_screen().unwrap();

        // After effects
        environment.render_transparent(time, &camera, &screen, renderer.geometry_pass_color_texture(), renderer.geometry_pass_position_texture()).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
