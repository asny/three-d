use rand::prelude::*;
use three_d::window::*;
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Fireworks!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 30.0, 150.0),
        vec3(0.0, 30.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(0.1);

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
    let fragment_shader_source = include_str!("../assets/shaders/particles.frag");
    let particles_program = ParticlesProgram::new(&context, &fragment_shader_source).unwrap();
    let mut square = CPUMesh::square();
    square.transform(&Mat4::from_scale(0.6));
    let mut particles = Particles::new(&context, &square).unwrap();

    // main loop
    particles.time = explosion_time + 100.0;
    let mut color_index = 0;
    window
        .render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport).unwrap();

            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();
            let elapsed_time = (frame_input.elapsed_time * 0.001) as f32;
            particles.time += elapsed_time;
            if particles.time > explosion_time {
                particles.time = 0.0;
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

            Screen::write(&context, ClearState::color(0.0, 0.0, 0.0, 1.0), || {
                let f = particles.time / explosion_time.max(0.0);
                let fade = 1.0 - f * f * f * f;
                let color = colors[color_index];
                particles_program.use_uniform_vec4(
                    "color",
                    &vec4(color.x * fade, color.y * fade, color.z * fade, 1.0),
                )?;
                let render_states = RenderStates {
                    cull: Cull::Back,
                    blend: Blend::Enabled {
                        rgb_equation: BlendEquationType::Add,
                        alpha_equation: BlendEquationType::Add,
                        source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
                        source_alpha_multiplier: BlendMultiplierType::Zero,
                        destination_rgb_multiplier: BlendMultiplierType::One,
                        destination_alpha_multiplier: BlendMultiplierType::One,
                    },
                    depth_test: DepthTest::Always,
                    write_mask: WriteMask::COLOR,
                    clip: Clip::Disabled,
                };
                particles.render(render_states, &particles_program, &camera, particles.time)?;
                Ok(())
            })
            .unwrap();

            if args.len() > 1 && particles.time > explosion_time * 0.5 {
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
