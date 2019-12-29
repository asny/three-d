
use window::{event::*, Window};
use dust::*;
use dust::objects::MeshShader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Lighting!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(vec3(2.0, 2.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);
    camera.enable_matrix_buffer(&gl);

    let loaded_objects: Vec<_> = Mesh::new_from_obj_source(&gl, include_str!("../assets/models/tree1.obj").to_string()).unwrap();
    for object in loaded_objects.iter() {
        println!("{}", object.name());
    }
    let objects: Vec<_> = loaded_objects.into_iter().filter(|object| object.name().starts_with("tree.001")).collect();

    let imposter = Imposter::new(&gl, &|camera: &Camera| {
            for object in objects.iter() {
                object.render(&Mat4::identity(), camera);
            }
        });

    let plane = Mesh::new_plane(&gl).unwrap();
    let mut mesh_shader = MeshShader::new(&gl).unwrap();
    mesh_shader.diffuse_intensity = 0.5;
    mesh_shader.specular_intensity = 0.8;

    renderer.ambient_light().set_intensity(0.1);

    let mut directional_light = renderer.directional_light(0).unwrap();
    directional_light.set_direction(&vec3(1.0, -1.0, -1.0));
    directional_light.set_color(&vec3(1.0, 0.0, 0.0));
    directional_light.set_intensity(0.5);
    directional_light.enable_shadows();

    directional_light = renderer.directional_light(1).unwrap();
    directional_light.set_direction(&vec3(-1.0, -1.0, 1.0));
    directional_light.set_color(&vec3(0.0, 1.0, 0.0));
    directional_light.set_intensity(0.5);
    directional_light.enable_shadows();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut camera);
        }

        // Draw
        let render_scene = |camera: &Camera| {
            for object in objects.iter() {
                object.render(&Mat4::identity(), camera);
            }
        };

        // Shadow pass
        renderer.shadow_pass(&render_scene);

        // Geometry pass
        renderer.geometry_pass(&||
            {
                render_scene(&camera);
                mesh_shader.render(&plane, &(Mat4::from_scale(10.0)), &camera);
                imposter.render(&Mat4::identity(), &camera);
            }).unwrap();

        // Light pass
        renderer.light_pass(&camera).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, renderer.screen_rendertarget()).unwrap();
            std::process::exit(1);
        }

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