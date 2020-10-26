
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

    let monkey = Mesh::from_file(&gl, "./examples/assets/models/suzanne.3d");
    monkey.borrow_mut().color = vec3(0.5, 1.0, 0.5);

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
            let transformation = Mat4::identity();
            monkey.borrow().render(&transformation, &camera);
        }).unwrap();

        let render = || {
                renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[]).unwrap();
                if fog_enabled {
                    fog_effect.apply(time as f32, &camera, renderer.geometry_pass_depth_texture()).unwrap();
                }
            };

        if fxaa_enabled {
            let color_texture = Texture2D::new(&gl, width, height, Interpolation::Nearest,
                         Interpolation::Nearest, None, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::RGBA8).unwrap();
            RenderTarget::write_to_color(&gl,0, 0, width, height,Some(&vec4(0.0, 0.0, 0.0, 0.0)), Some(&color_texture), &render).unwrap();
            Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 1.0)), None, &|| {
                fxaa_effect.apply(&color_texture).unwrap();
            }).unwrap();
        } else {
            Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 1.0)), None, &render).unwrap();
        }

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            Screen::save_color(path, &gl, 0, 0, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}