use rand::prelude::*;
use three_d::core::*;
use three_d::window::*;
use three_d::*;

#[derive(Clone)]
struct FireworksMaterial {
    pub color: Color,
    pub fade: f32,
}

impl Material for FireworksMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("particles.frag").to_string()
    }
    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("color", self.color);
        program.use_uniform("fade", self.fade);
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            cull: Cull::Back,
            blend: Blend::Enabled {
                rgb_equation: BlendEquationType::Add,
                alpha_equation: BlendEquationType::Add,
                source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
                source_alpha_multiplier: BlendMultiplierType::Zero,
                destination_rgb_multiplier: BlendMultiplierType::One,
                destination_alpha_multiplier: BlendMultiplierType::One,
            },
            depth_test: DepthTest::LessOrEqual,
            write_mask: WriteMask::COLOR,
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}
// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run();
}

pub fn run() {
    let window = Window::new(WindowSettings {
        title: "Fireworks!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 30.0, 150.0),
        vec3(0.0, 30.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.1);

    let mut rng = rand::thread_rng();

    let explosion_speed = 15.0;
    let explosion_time = 3.0;
    let colors = [
        Color::new_opaque(255, 255, 178),
        Color::new_opaque(255, 51, 25),
        Color::new_opaque(51, 102, 51),
        Color::new_opaque(127, 127, 204),
        Color::new_opaque(217, 23, 51),
        Color::new_opaque(250, 237, 38),
        Color::new_opaque(76, 237, 38),
        Color::new_opaque(40, 178, 222),
    ];
    let mut square = CpuMesh::square();
    square.transform(&Mat4::from_scale(0.6)).unwrap();

    // A particle system is created with an acceleration of -9.82 in the y direction to simulate gravity.
    let particles = ParticleSystem::new(
        &context,
        &Particles::default(),
        vec3(0.0, -9.82, 0.0),
        &square,
    );
    let fireworks_material = FireworksMaterial {
        color: colors[0],
        fade: 0.0,
    };
    let mut fireworks = Gm::new(particles, fireworks_material);

    // main loop
    let mut time = explosion_time + 100.0; // Ensure initialisation on the first loop.
    let mut color_index = 0;
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);

        control.handle_events(&mut camera, &mut frame_input.events);
        let elapsed_time = (frame_input.elapsed_time * 0.001) as f32;

        // Update the time in the particlesystem; this automatically integrates the velocity and
        // the acceleration of each particle to calculate its new position.
        time += elapsed_time;

        // If the time exceeds the explosion duration, re-initialise the explosion.
        if time > explosion_time {
            color_index = (color_index + 1) % colors.len();
            fireworks.material.color = colors[color_index];
            time = 0.0;
            let start_position = vec3(
                10.0 * rng.gen::<f32>() - 5.0,
                40.0 + 10.0 * rng.gen::<f32>(),
                10.0 * rng.gen::<f32>() - 5.0,
            );
            let start_positions = (0..300).map(|_| start_position).collect();
            let colors = Some(
                (0..300)
                    .map(|_| {
                        Color::new_opaque(
                            (rng.gen::<f32>() * 100.0 - 50.0) as u8,
                            (rng.gen::<f32>() * 100.0 - 50.0) as u8,
                            (rng.gen::<f32>() * 100.0 - 50.0) as u8,
                        )
                    })
                    .collect(),
            );
            let mut start_velocities = Vec::new();
            for _ in 0..300 {
                let theta = rng.gen::<f32>() * 2.0 - 1.0;
                let phi = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
                let explosion_direction = vec3(
                    theta.acos().sin() * phi.cos(),
                    theta.acos().sin() * phi.sin(),
                    theta,
                );
                start_velocities
                    .push((rng.gen::<f32>() * 0.2 + 0.9) * explosion_speed * explosion_direction);
            }
            fireworks.set_particles(&Particles {
                start_positions,
                start_velocities,
                colors,
                ..Default::default()
            });
        }

        let f = time / explosion_time.max(0.0);
        fireworks.material.fade = 1.0 - f * f * f * f;
        // Since our geometry is a square, we always want to view it from the same direction, nomatter how we change the camera.
        fireworks.set_transformation(
            Mat4::from_cols(
                camera.view().x,
                camera.view().y,
                camera.view().z,
                vec4(0.0, 0.0, 0.0, 1.0),
            )
            .invert()
            .unwrap(),
        );
        fireworks.animate(time);
        frame_input
            .screen()
            .clear(ClearState::color(0.0, 0.0, 0.0, 1.0))
            .render(&camera, &fireworks, &[]);

        FrameOutput::default()
    });
}
