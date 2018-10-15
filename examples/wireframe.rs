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
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),screen.aspect(), 0.1, 1000.0);

    let mut mesh = gust::loader::load_obj_as_dynamic_mesh("../Dust/examples/assets/models/box.obj").unwrap();
    mesh.update_vertex_normals();
    let model = ::objects::ShadedMesh::create(&gl, &mesh);

    let mut edges = ::objects::ShadedEdges::create(&gl, &mesh);
    edges.diffuse_intensity = 0.8;
    edges.specular_intensity = 0.2;
    edges.specular_power = 5.0;

    let mut vertices = ::objects::ShadedVertices::create(&gl, &mesh);
    vertices.diffuse_intensity = 0.8;
    vertices.specular_intensity = 0.2;
    vertices.specular_power = 5.0;
    vertices.scale = 0.1;

    let mut plane = ::objects::ShadedMesh::create(&gl, &mesh_generator::create_plane().unwrap());
    plane.diffuse_intensity = 0.1;
    plane.specular_intensity = 0.3;
    plane.specular_power = 40.0;

    let mut ambient_light = ::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut light1 = dust::light::DirectionalLight::new(vec3(-1.0, -1.0, -1.0));
    light1.enable_shadows(&gl, 4.0, 10.0).unwrap();
    light1.base.intensity = 0.5;

    let mut light2 = dust::light::DirectionalLight::new(vec3(1.0, -1.0, 1.0));
    light2.enable_shadows(&gl, 4.0, 10.0).unwrap();
    light2.base.intensity = 0.5;

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
        let render_scene = |camera: &Camera| {
            plane.render(&(Mat4::new_translation(&vec3(0.0, -1.0, 0.0)) * Mat4::new_scaling(10.0)), camera);
            //model.render(&Mat4::identity(), camera);
            edges.render(camera);
            vertices.render(camera);
        };

        // Shadow pass
        light1.shadow_cast_begin().unwrap();
        render_scene(light1.shadow_camera().unwrap());

        light2.shadow_cast_begin().unwrap();
        render_scene(light2.shadow_camera().unwrap());

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&light1).unwrap();
        renderer.shine_directional_light(&light2).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}
