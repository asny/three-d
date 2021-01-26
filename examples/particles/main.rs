
use three_d::*;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new("Particles", 800, 800).unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    let mut camera = Camera::new_perspective(&gl, vec3(0.0, 50.0, 170.0), vec3(0.0, 50.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let mut rng = rand::thread_rng();

    let explosion_speed = 12.0;
    let explosion_time = 3.0;
    let mut particles = Particles::new(&gl, &include_str!("../assets/shaders/particles.frag"), &CPUMesh::square(), &vec3(0.0, -9.82, 0.0)).unwrap();

    // main loop
    let mut time = explosion_time + 100.0;
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
        let elapsed_time = (frame_input.elapsed_time * 0.001) as f32;
        time += elapsed_time;
        if time > explosion_time {
            time = 0.0;
            let start_position = vec3(20.0 * rng.gen::<f32>() - 10.0, 40.0 + 20.0 * rng.gen::<f32>(), 20.0 * rng.gen::<f32>() - 10.0);
            let start_direction = vec3(rng.gen::<f32>() - 0.5, 0.5 + 0.5 * rng.gen::<f32>(), rng.gen::<f32>() - 0.5).normalize();
            let tangent = start_direction.cross(vec3(1.0, 0.0, 0.0));
            let cotangent = start_direction.cross(tangent);
            let mut data = Vec::new();
            for _ in 0..1000 {
                let explosion_direction = ((1.2 * rng.gen::<f32>() - 0.2) * start_direction
                    + (rng.gen::<f32>() - 0.5) * tangent
                    + (rng.gen::<f32>() - 0.5) * cotangent).normalize();
                data.push(ParticleData {
                    start_position,
                    start_velocity: (rng.gen::<f32>() + 0.5) * explosion_speed * explosion_direction
                });
            }
            particles.update(&data);
        }

        Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 0.0)), Some(1.0), || {
            let render_states = RenderStates {cull: CullType::Back,
                blend: Some(BlendParameters::new(BlendEquationType::Add, BlendMultiplierType::One, BlendMultiplierType::One)),
                ..Default::default()};
            let fade = (1.0 - time/explosion_time).max(0.0);
            particles.program().add_uniform_vec4("color", &vec4(fade, fade * 0.2, fade * 0.1, 1.0))?;
            particles.render(render_states, &Mat4::identity(), &camera, time)?;
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