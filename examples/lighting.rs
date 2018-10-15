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
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), screen.aspect(), 0.1, 1000.0);

    let mesh = gust::loader::load_obj_as_static_mesh("../Dust/examples/assets/models/suzanne.obj").unwrap();
    let mut monkey = objects::ShadedMesh::create(&gl, &mesh);

    let plane = ::objects::ShadedMesh::create(&gl, &mesh_generator::create_plane().unwrap());

    let mut ambient_light = ::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut directional_light = dust::light::DirectionalLight::new(vec3(0.0, -1.0, -1.0));
    directional_light.enable_shadows(&gl, 5.0, 1000.0).unwrap();

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
            handle_ambient_light_parameters(&event, &mut ambient_light);
            handle_directional_light_parameters(&event, &mut directional_light);
            handle_surface_parameters(&event, &mut monkey);
        }

        // Draw
        let render_scene = |camera: &Camera| {
            monkey.render(&Mat4::identity(), camera);
            plane.render(&(Mat4::new_translation(&vec3(0.0, -1.0, 0.0)) * Mat4::new_scaling(10.0)), camera);
        };

        // Shadow pass
        directional_light.shadow_cast_begin().unwrap();
        render_scene(directional_light.shadow_camera().unwrap());

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}

fn handle_ambient_light_parameters(event: &Event, light: &mut light::AmbientLight)
{
    match event {
        Event::KeyDown { keycode: Some(Keycode::X), .. } => {
            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
            println!("Ambient light intensity: {}", light.base.intensity);
        },
        Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
            println!("Ambient light intensity: {}", light.base.intensity);
        },
        _ => {}
    }
}

fn handle_directional_light_parameters(event: &Event, light: &mut light::DirectionalLight)
{
    match event {
        Event::KeyDown { keycode: Some(Keycode::V), .. } => {
            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
            println!("Directional light intensity: {}", light.base.intensity);
        },
        Event::KeyDown { keycode: Some(Keycode::C), .. } => {
            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
            println!("Directional light intensity: {}", light.base.intensity);
        },
        _ => {}
    }
}

fn handle_surface_parameters(event: &Event, surface: &mut ::objects::ShadedMesh)
{
    match event {
        Event::KeyDown { keycode: Some(Keycode::S), .. } => {
            surface.diffuse_intensity = (surface.diffuse_intensity + 0.1).min(1.0);
            println!("Diffuse intensity: {}", surface.diffuse_intensity);
        },
        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
            surface.diffuse_intensity = (surface.diffuse_intensity - 0.1).max(0.0);
            println!("Diffuse intensity: {}", surface.diffuse_intensity);
        },
        Event::KeyDown { keycode: Some(Keycode::F), .. } => {
            surface.specular_intensity = (surface.specular_intensity + 0.1).min(1.0);
            println!("Specular intensity: {}", surface.specular_intensity);
        },
        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
            surface.specular_intensity = (surface.specular_intensity - 0.1).max(0.0);
            println!("Specular intensity: {}", surface.specular_intensity);
        },
        Event::KeyDown { keycode: Some(Keycode::H), .. } => {
            surface.specular_power = surface.specular_power + 1.0;
            println!("Specular power: {}", surface.specular_power);
        },
        Event::KeyDown { keycode: Some(Keycode::G), .. } => {
            surface.specular_power = (surface.specular_power - 1.0).max(0.0);
            println!("Specular power: {}", surface.specular_power);
        },
        _ => {}
    }
}