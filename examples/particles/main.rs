
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Particles").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let pipeline = PhongForwardPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(4.0, 1.5, 4.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let material = PhongMaterial {
        color_source: ColorSource::Color(vec4(1.0, 0.0, 0.0, 1.0)),
        ..Default::default()
    };

    //let mut particles = pipeline.new_instanced_mesh(&[], &CPUMesh::circle(1.0, 16), &material).unwrap();
    let mut particles = Particles::new(&gl, &CPUMesh::circle(1.0, 16), &material).unwrap();

    let ambient_light = AmbientLight::new(&gl, 0.4, &vec3(1.0, 1.0, 1.0)).unwrap();

    // main loop
    let mut rotating = false;
    window.render_loop(move |frame_input|
    {
        camera.set_aspect(frame_input.screen_width as f32 / frame_input.screen_height as f32);

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
                _ => { }
            }
        }

        particles.update_positions(&[vec3(0.0, 0.0, 0.0)]);

        // draw
        pipeline.render_to_screen(width, height, || {
            state::cull(&gl, state::CullType::None);
            particles.render_with_ambient(&camera, &ambient_light)?;
            Ok(())
        }).unwrap();

        #[cfg(target_arch = "x86_64")]
        if let Some(ref path) = screenshot_path {
            let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
            Saver::save_pixels(path, &pixels, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}