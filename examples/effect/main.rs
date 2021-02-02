
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Effect").unwrap();
    let viewport = window.viewport();
    let gl = window.gl();

    // Renderer
    let mut pipeline = PhongForwardPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(4.0, 4.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), viewport.aspect(), 0.1, 1000.0);

    Loader::load(&["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl",
        "examples/assets/skybox_evening/back.jpg", "examples/assets/skybox_evening/front.jpg",
        "examples/assets/skybox_evening/top.jpg", "examples/assets/skybox_evening/left.jpg",
        "examples/assets/skybox_evening/right.jpg"], move |loaded|
    {
        let (meshes, mut materials) = Obj::parse(loaded, "examples/assets/suzanne.obj").unwrap();
        materials[0].color = Some((0.5, 1.0, 0.5, 1.0));
        let monkey = PhongForwardMesh::new(&gl, &meshes[0], &PhongMaterial::new(&gl, &materials[0]).unwrap()).unwrap();

        let ambient_light = AmbientLight{ intensity: 0.2, color: vec3(1.0, 1.0, 1.0) };
        let directional_light = DirectionalLight::new(&gl, 0.5, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0)).unwrap();

        // Fog
        let mut fog_effect = effects::FogEffect::new(&gl).unwrap();
        fog_effect.color = vec3(0.8, 0.8, 0.8);
        let mut fog_enabled = true;

        // FXAA
        let fxaa_effect = effects::FXAAEffect::new(&gl).unwrap();
        let mut fxaa_enabled = true;

        // Skybox
        let skybox = Skybox::new(&gl, &Loader::get_image(loaded, "examples/assets/skybox_evening/right.jpg").unwrap(),
                                 &Loader::get_image(loaded, "examples/assets/skybox_evening/left.jpg").unwrap(),
                                 &Loader::get_image(loaded, "examples/assets/skybox_evening/top.jpg").unwrap(),
                                 &Loader::get_image(loaded, "examples/assets/skybox_evening/front.jpg").unwrap(),
                                 &Loader::get_image(loaded, "examples/assets/skybox_evening/back.jpg").unwrap()).unwrap();

        // main loop
        let mut time = 0.0;
        let mut rotating = false;
        window.render_loop(move |frame_input|
        {
            camera.set_aspect(frame_input.viewport.aspect());

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
            if fog_enabled || fxaa_enabled {
                pipeline.depth_pass(viewport.width, viewport.height, &|| {
                    let render_states = RenderStates {cull: CullType::Back, ..Default::default()};
                    monkey.render_depth(render_states, viewport, &Mat4::identity(), &camera)?;
                    Ok(())
                }).unwrap();
            }

            if fxaa_enabled {
                let color_texture = Texture2D::new(&gl, viewport.width, viewport.height, Interpolation::Nearest,
                                                   Interpolation::Nearest, None, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::RGBA8).unwrap();

                RenderTarget::write(&gl, Some(&vec4(0.0, 0.0, 0.0, 0.0)), None, Some(&color_texture), Some(pipeline.depth_texture()), || {
                    let render_states = RenderStates {depth_test: DepthTestType::LessOrEqual, cull: CullType::Back, ..Default::default()};
                    monkey.render_with_ambient_and_directional(render_states, viewport, &Mat4::identity(), &camera, &ambient_light, &directional_light)?;
                    skybox.render(viewport, &camera)?;
                    if fog_enabled {
                        fog_effect.apply(viewport, &camera, pipeline.depth_texture(), time as f32)?;
                    }
                    Ok(())
                }).unwrap();

                Screen::write(&gl, Some(&vec4(0.0, 0.0, 0.0, 1.0)), Some(1.0), &|| {
                    fxaa_effect.apply(viewport, &color_texture)?;
                    Ok(())
                }).unwrap();
            } else {
                Screen::write(&gl, Some(&vec4(0.0, 0.0, 0.0, 1.0)), Some(1.0), &|| {
                    let render_states = RenderStates {depth_test: DepthTestType::LessOrEqual, cull: CullType::Back, ..Default::default()};
                    monkey.render_with_ambient_and_directional(render_states, viewport, &Mat4::identity(), &camera, &ambient_light, &directional_light)?;
                    skybox.render(viewport, &camera)?;
                    if fog_enabled {
                        fog_effect.apply(viewport, &camera, pipeline.depth_texture(), time as f32)?;
                    }
                    Ok(())
                }).unwrap();
            }

            #[cfg(target_arch = "x86_64")]
            if let Some(ref path) = screenshot_path {
                let pixels = Screen::read_color(&gl, viewport).unwrap();
                Saver::save_pixels(path, &pixels, viewport.width, viewport.height).unwrap();
                std::process::exit(1);
            }
        }).unwrap();
    });
}