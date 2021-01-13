use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Lighting!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = PhongDeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(2.0, 2.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    Loader::load(&["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"], move |loaded|
    {
        let (monkey_cpu_meshes, mut monkey_cpu_materials) = Obj::parse(loaded, "examples/assets/suzanne.obj").unwrap();
        monkey_cpu_materials[0].diffuse_intensity = Some(0.7);
        monkey_cpu_materials[0].specular_intensity = Some(0.8);
        monkey_cpu_materials[0].specular_power = Some(20.0);
        let mut monkey = renderer.new_meshes(&monkey_cpu_meshes, &monkey_cpu_materials).unwrap().remove(0);

        let mut plane = renderer.new_mesh(
            &CPUMesh {
                positions: vec!(-10000.0, -1.0, 10000.0, 10000.0, -1.0, 10000.0, 0.0, -1.0, -10000.0),
                normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                ..Default::default()},
            &PhongMaterial {color_source: ColorSource::Color(vec4(0.5, 0.7, 0.3, 1.0)),
                diffuse_intensity: 0.7,
                specular_intensity: 0.8,
                specular_power: 20.0, ..Default::default()}
        ).unwrap();

        let mut directional_light0 = DirectionalLight::new(&gl, 0.3, &vec3(1.0, 0.0, 0.0), &vec3(0.0, -1.0, 0.0)).unwrap();
        let mut directional_light1 = DirectionalLight::new(&gl, 0.3, &vec3(0.0, 1.0, 0.0), &vec3(0.0, -1.0, 0.0)).unwrap();
        let mut point_light0 = PointLight::new(&gl, 0.5, &vec3(0.0, 1.0, 0.0), &vec3(0.0, 0.0, 0.0), 0.5, 0.05, 0.005).unwrap();
        let mut point_light1 = PointLight::new(&gl, 0.5, &vec3(1.0, 0.0, 0.0), &vec3(0.0, 0.0, 0.0), 0.5, 0.05, 0.005).unwrap();
        let mut spot_light = SpotLight::new(&gl, 0.8, &vec3(0.0, 0.0, 1.0), &vec3(0.0, 0.0, 0.0), &vec3(0.0, -1.0, 0.0), 25.0, 0.1, 0.001, 0.0001).unwrap();

        // main loop
        let mut time = 0.0;
        let mut rotating = false;
        let mut shadows_enabled = true;
        window.render_loop(move |frame_input|
        {
            camera.set_aspect(frame_input.screen_width as f32 / frame_input.screen_height as f32);

            time += (0.001 * frame_input.elapsed_time) % 1000.0;
            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    },
                    Event::MouseMotion { delta } => {
                        if rotating {
                            camera.rotate_around_up(delta.0 as f32, delta.1 as f32);
                        }
                    },
                    Event::MouseWheel { delta } => {
                        camera.zoom(*delta as f32);
                    },
                    Event::Key { ref state, ref kind } => {
                        if kind == "T" && *state == State::Pressed
                        {
                            shadows_enabled = !shadows_enabled;
                            if !shadows_enabled {
                                spot_light.clear_shadow_map();
                                directional_light0.clear_shadow_map();
                                directional_light1.clear_shadow_map();
                            }
                        }
                        #[cfg(target_arch = "x86_64")]
                        if kind == "P" && *state == State::Pressed
                        {
                            let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
                            Saver::save_pixels("lighting.png", &pixels, width, height).unwrap();
                        }
                        if kind == "R" && *state == State::Pressed
                        {
                            renderer.next_debug_type();
                            println!("{:?}", renderer.debug_type());
                        }
                    }
                }
                handle_surface_parameters(&event, &mut plane.material);
                handle_surface_parameters(&event, &mut monkey.material);
            }
            let c = time.cos() as f32;
            let s = time.sin() as f32;
            directional_light0.set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
            directional_light1.set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));
            spot_light.set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
            spot_light.set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));
            point_light0.set_position(&vec3(-5.0 * c, 5.0, -5.0 * s));
            point_light1.set_position(&vec3(5.0 * c, 5.0, 5.0 * s));

            // Draw
            let render_scene_depth = |camera: &Camera| {
                monkey.render_depth(&Mat4::identity(), camera)?;
                Ok(())
            };
            if shadows_enabled {
                directional_light0.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 4.0, 4.0, 20.0, 1024, 1024, render_scene_depth);
                directional_light1.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 4.0, 4.0, 20.0, 1024, 1024, render_scene_depth);
                spot_light.generate_shadow_map(20.0, 1024, render_scene_depth);
            }

            // Geometry pass
            renderer.geometry_pass(width, height, &||
                {
                    monkey.render_geometry(&Mat4::identity(), &camera)?;
                    plane.render_geometry(&Mat4::identity(), &camera)?;
                    Ok(())
                }).unwrap();

            // Light pass
            renderer.render_to_screen(&camera, None, &[&directional_light0, &directional_light1],
                                    &[&spot_light], &[&point_light0, &point_light1], width, height).unwrap();

            #[cfg(target_arch = "x86_64")]
            if let Some(ref path) = screenshot_path {
                let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
                Saver::save_pixels(path, &pixels, width, height).unwrap();
                std::process::exit(1);
            }
        }).unwrap();
    });
}

fn handle_surface_parameters(event: &Event, surface: &mut PhongMaterial)
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