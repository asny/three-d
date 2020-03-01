
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Lighting!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(2.0, 2.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let mut monkey = CPUMesh::new(include_bytes!("../assets/models/suzanne.3d")).unwrap().to_mesh(&gl).unwrap();
    monkey.diffuse_intensity = 0.3;
    monkey.specular_intensity = 0.8;
    monkey.specular_power = 20.0;

    let mut plane_mesh = tri_mesh::MeshBuilder::new().plane().build().unwrap();
    plane_mesh.scale(10.0);
    plane_mesh.translate(tri_mesh::prelude::vec3(0.0, -1.0, 0.0));
    let mut plane = Mesh::new(&gl, &plane_mesh.indices_buffer(), &plane_mesh.positions_buffer_f32(), &plane_mesh.normals_buffer_f32()).unwrap();
    plane.diffuse_intensity = 0.3;
    plane.specular_intensity = 0.8;
    plane.specular_power = 20.0;

    renderer.ambient_light().set_intensity(0.1);

    let mut directional_light = DirectionalLight::new(&gl).unwrap();
    directional_light.set_intensity(0.3);

    /*directional_light = renderer.directional_light(1).unwrap();
    directional_light.set_intensity(0.3);
    directional_light.enable_shadows();

    let mut point_light = renderer.point_light(0).unwrap();
    point_light.set_intensity(0.5);
    point_light.set_color(&vec3(0.0, 1.0, 0.0));

    point_light = renderer.point_light(1).unwrap();
    point_light.set_intensity(0.5);
    point_light.set_color(&vec3(1.0, 0.0, 0.0));

    let spot_light = renderer.spot_light(0).unwrap();
    spot_light.set_intensity(0.8);
    spot_light.set_color(&vec3(0.0, 0.0, 1.0));
    spot_light.set_cutoff(0.1*std::f32::consts::PI);
    spot_light.set_attenuation(0.1, 0.001, 0.0001);
    spot_light.enable_shadows();*/

    // main loop
    let mut time = 0.0;
    let mut rotating = false;
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        time += (0.001 * frame_input.elapsed_time) % 1000.0;
        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    rotating = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if rotating {
                        camera.rotate(delta.0 as f32, delta.1 as f32);
                    }
                },
                Event::MouseWheel {delta} => {
                    camera.zoom(*delta as f32);
                },
                Event::Key { ref state, ref kind } => {
                    if kind == "R" && *state == State::Pressed
                    {

                    }
                }
            }
            //handle_ambient_light_parameters(event, &mut ambient_light);
            //handle_directional_light_parameters(event, &mut directional_light);
            handle_surface_parameters(&event, &mut plane);
            handle_surface_parameters(&event, &mut monkey);
        }
        let c = time.cos() as f32;
        let s = time.sin() as f32;
        directional_light.set_direction(&vec3(-1.0-c, -1.0, 1.0+s));
        /*renderer.directional_light(1).unwrap().set_direction(&vec3(1.0+c, -1.0, -1.0-s));
        renderer.spot_light(0).unwrap().set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
        renderer.spot_light(0).unwrap().set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));
        renderer.point_light(0).unwrap().set_position(&vec3(-5.0 * c, 5.0, -5.0 * s));
        renderer.point_light(1).unwrap().set_position(&vec3(5.0 * c, 5.0, 5.0 * s));*/

        // Draw
        let render_scene = |camera: &Camera| {
            monkey.render(&Mat4::identity(), camera);
        };
        directional_light.generate_shadow_map(vec3(0.0, 0.0, 0.0), 4.0, 4.0, 20.0, 1024, 1024, &render_scene);
        /*renderer.directional_light(1).unwrap().generate_shadow_map(&render_scene);
        renderer.spot_light(0).unwrap().generate_shadow_map(&render_scene);*/

        // Geometry pass
        renderer.geometry_pass(&||
            {
                render_scene(&camera);
                plane.render(&Mat4::identity(), &camera);
            }).unwrap();

        // Light pass

        ScreenRendertarget::write(&gl, width, height);
        ScreenRendertarget::clear_color_and_depth(&gl, &vec4(0.0, 0.0, 0.0, 0.0));
        renderer.light_pass_render_to_rendertarget(&camera, &directional_light).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, width, height).unwrap();
            std::process::exit(1);
        }

    }).unwrap();
}

/*fn handle_ambient_light_parameters(event: &Event, light: &mut light::AmbientLight)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::X => {
                            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
                            println!("Ambient light intensity: {}", light.base.intensity);
                        },
                        VirtualKeyCode::Z => {
                            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
                            println!("Ambient light intensity: {}", light.base.intensity);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}

fn handle_directional_light_parameters(event: &Event, light: &mut light::DirectionalLight)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::V => {
                            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
                            println!("Directional light intensity: {}", light.base.intensity);
                        },
                        VirtualKeyCode::C => {
                            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
                            println!("Directional light intensity: {}", light.base.intensity);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}*/

fn handle_surface_parameters(event: &Event, surface: &mut Mesh)
{
    match event {
        Event::Key { state, kind } => {
            if kind == "S" && *state == State::Pressed {
                surface.diffuse_intensity = (surface.diffuse_intensity + 0.1).min(1.0);
                println!("Diffuse intensity: {}", surface.diffuse_intensity);
            }
            if kind == "A" && *state == State::Pressed {
                surface.diffuse_intensity = (surface.diffuse_intensity - 0.1).max(0.0);
                println!("Diffuse intensity: {}", surface.diffuse_intensity);
            }
            if kind == "F" && *state == State::Pressed {
                surface.specular_intensity = (surface.specular_intensity + 0.1).min(1.0);
                println!("Specular intensity: {}", surface.specular_intensity);
            }
            if kind == "D" && *state == State::Pressed {
                surface.specular_intensity = (surface.specular_intensity - 0.1).max(0.0);
                println!("Specular intensity: {}", surface.specular_intensity);
            }
            if kind == "H" && *state == State::Pressed {
                surface.specular_power = (surface.specular_power + 2.0).min(30.0);
                println!("Specular power: {}", surface.specular_power);
            }
            if kind == "G" && *state == State::Pressed {
                surface.specular_power = (surface.specular_power - 2.0).max(2.0);
                println!("Specular power: {}", surface.specular_power);
            }
        },
        _ => {}
    }
}