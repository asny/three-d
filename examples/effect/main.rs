
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Effect").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(4.0, 4.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    Loader::load(&["./examples/assets/suzanne.3d"], move |loaded| {
        let (meshes, mut materials) = ThreeD::parse(loaded, "./examples/assets/suzanne.3d").unwrap();
        materials[0].color = Some((0.5, 1.0, 0.5, 1.0));
        let monkey = renderer.new_meshes(&meshes, &materials).unwrap().remove(0);

        let ambient_light = AmbientLight::new(&gl, 0.2, &vec3(1.0, 1.0, 1.0)).unwrap();
        let directional_light = DirectionalLight::new(&gl, 0.5, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0)).unwrap();

        let mut fog_effect = effects::FogEffect::new(&gl).unwrap();
        fog_effect.color = vec3(0.8, 0.8, 0.8);
        let mut fog_enabled = true;
        let fxaa_effect = effects::FXAAEffect::new(&gl).unwrap();
        let mut fxaa_enabled = true;

        // main loop
        let mut time = 0.0;
        let mut rotating = false;
        window.render_loop(move |frame_input|
        {
            camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    },
                    Event::MouseMotion { delta } => {
                        if rotating {
                            camera.rotate(delta.0 as f32, delta.1 as f32);
                        }
                    },
                    Event::MouseWheel { delta } => {
                        camera.zoom(*delta as f32);
                    },
                    Event::Key { state, kind } => {
                        if kind == "R" && *state == State::Pressed
                        {
                            renderer.next_debug_type();
                            println!("{:?}", renderer.debug_type());
                        }
                        if kind == "F" && *state == State::Pressed
                        {
                            fog_enabled = !fog_enabled;
                            println!("Fog: {:?}", fog_enabled);
                        }
                        if kind == "X" && *state == State::Pressed
                        {
                            fxaa_enabled = !fxaa_enabled;
                            println!("FXAA: {:?}", fxaa_enabled);
                        }
                    }
                }
            }
            time += frame_input.elapsed_time;

            // draw
            renderer.geometry_pass(width, height, &|| {
                monkey.render_geometry(&Mat4::identity(), &camera)?;
                Ok(())
            }).unwrap();

            if fxaa_enabled {
                let color_texture = Texture2D::new(&gl, width, height, Interpolation::Nearest,
                                                   Interpolation::Nearest, None, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::RGBA8).unwrap();

                RenderTarget::write_to_color(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 0.0)), Some(&color_texture), || {
                    renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[])?;
                    if fog_enabled {
                        fog_effect.apply(time as f32, &camera, renderer.geometry_pass_depth_texture())?;
                    }
                    Ok(())
                }).unwrap();

                Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 1.0)), None, &|| {
                    fxaa_effect.apply(&color_texture)?;
                    Ok(())
                }).unwrap();
            } else {
                renderer.render_to_screen_with_forward_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[], width, height, || {
                    if fog_enabled {
                        fog_effect.apply(time as f32, &camera, renderer.geometry_pass_depth_texture())?;
                    }
                    Ok(())
                }).unwrap();
            }

            #[cfg(target_arch = "x86_64")]
            if let Some(ref path) = screenshot_path {
                let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
                Saver::save_pixels(path, &pixels, width, height).unwrap();
                std::process::exit(1);
            }
        }).unwrap();
    });
}