use rand::prelude::*;
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new("Fireworks", Some((1280, 720))).unwrap();
    let context = window.gl().unwrap();

    let mut camera = CameraControl::new(
        Camera::new_perspective(
            &context,
            vec3(0.0, 30.0, 150.0),
            vec3(0.0, 30.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            window.viewport().aspect(),
            0.1,
            1000.0,
        )
        .unwrap(),
    );

    let mut rng = rand::thread_rng();

    let explosion_speed = 15.0;
    let explosion_time = 3.0;
    let colors = [
        vec3(1.0, 1.0, 0.7),
        vec3(1.0, 0.2, 0.1),
        vec3(0.2, 0.4, 0.2),
        vec3(0.5, 0.5, 0.8),
        vec3(0.85, 0.09, 0.51),
        vec3(0.98, 0.93, 0.15),
        vec3(0.3, 0.93, 0.15),
        vec3(0.16, 0.07, 0.87),
    ];
    let particles_program =
        ParticlesProgram::new(&context, &include_str!("../assets/shaders/particles.frag")).unwrap();
    let mut particles =
        Particles::new(&context, &CPUMesh::square(1.2), &vec3(0.0, -9.82, 0.0)).unwrap();

    // main loop
    let mut time = explosion_time + 100.0;
    let mut rotating = false;
    let mut color_index = 0;
    window
        .render_loop(move |frame_input| {
            camera.set_aspect(frame_input.viewport.aspect()).unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    }
                    Event::MouseMotion { delta, .. } => {
                        if rotating {
                            camera.rotate(delta.0 as f32, delta.1 as f32).unwrap();
                        }
                    }
                    Event::MouseWheel { delta, .. } => {
                        camera.zoom(delta.1 as f32).unwrap();
                    }
                    _ => {}
                }
            }
            let elapsed_time = (frame_input.elapsed_time * 0.001) as f32;
            time += elapsed_time;
            if time > explosion_time {
                time = 0.0;
                color_index = (color_index + 1) % colors.len();
                let start_position = vec3(
                    10.0 * rng.gen::<f32>() - 5.0,
                    40.0 + 10.0 * rng.gen::<f32>(),
                    10.0 * rng.gen::<f32>() - 5.0,
                );
                let mut data = Vec::new();
                for _ in 0..300 {
                    let theta = rng.gen::<f32>() * std::f32::consts::PI;
                    let phi = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
                    let explosion_direction = vec3(
                        theta.sin() * phi.cos(),
                        theta.sin() * phi.sin(),
                        theta.cos(),
                    );
                    data.push(ParticleData {
                        start_position,
                        start_velocity: (rng.gen::<f32>() * 0.2 + 0.9)
                            * explosion_speed
                            * explosion_direction,
                    });
                }
                particles.update(&data);
            }

            Screen::write(&context, &ClearState::color(0.0, 0.0, 0.0, 1.0), || {
                let render_states = RenderStates {
                    cull: CullType::Back,
                    blend: Some(BlendParameters {
                        rgb_equation: BlendEquationType::Add,
                        alpha_equation: BlendEquationType::Add,
                        source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
                        source_alpha_multiplier: BlendMultiplierType::Zero,
                        destination_rgb_multiplier: BlendMultiplierType::One,
                        destination_alpha_multiplier: BlendMultiplierType::One,
                    }),
                    write_mask: WriteMask::COLOR,
                    depth_test: DepthTestType::Always,
                    ..Default::default()
                };
                let f = time / explosion_time.max(0.0);
                let fade = 1.0 - f * f * f * f;
                let color = colors[color_index];
                particles_program.use_uniform_vec4(
                    "color",
                    &vec4(color.x * fade, color.y * fade, color.z * fade, 1.0),
                )?;
                particles.render(
                    &particles_program,
                    render_states,
                    frame_input.viewport,
                    &Mat4::identity(),
                    &camera,
                    time,
                )?;
                Ok(())
            })
            .unwrap();

            if args.len() > 1 && time > explosion_time * 0.5 {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(args[1].clone().into()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
