use rand::prelude::*;
use three_d::core::*;
use three_d::window::*;
use three_d::*;

#[derive(Clone)]
struct FireworksMaterial {
    pub color: Vec3,
    pub fade: f32,
}

impl Material for FireworksMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("particles.frag").to_string()
    }
    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform(
            "color",
            vec4(
                self.color.x * self.fade,
                self.color.y * self.fade,
                self.color.z * self.fade,
                1.0,
            ),
        );
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
            depth_test: DepthTest::Always,
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
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        window.viewport().unwrap(),
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
        vec3(1.0, 1.0, 0.7),
        vec3(1.0, 0.2, 0.1),
        vec3(0.2, 0.4, 0.2),
        vec3(0.5, 0.5, 0.8),
        vec3(0.85, 0.09, 0.51),
        vec3(0.98, 0.93, 0.15),
        vec3(0.3, 0.93, 0.15),
        vec3(0.16, 0.07, 0.87),
    ];
    let mut square = CpuMesh::square();
    square.transform(&Mat4::from_scale(0.6)).unwrap();
    let mut particles = Particles::new(&context, &square).unwrap();
    let mut fireworks_material = FireworksMaterial {
        color: vec3(0.0, 0.0, 0.0),
        fade: 0.0,
    };

    // main loop
    particles.time = explosion_time + 100.0;
    let mut color_index = 0;
    window
        .render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport);

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
                particles.update(&data).unwrap();
            }

            frame_input
                .screen()
                .clear(ClearState::color(0.0, 0.0, 0.0, 1.0))
                .write(|| {
                    let f = particles.time / explosion_time.max(0.0);
                    fireworks_material.fade = 1.0 - f * f * f * f;
                    fireworks_material.color = colors[color_index];
                    particles.render_with_material(&fireworks_material, &camera, &[])?;
                    Ok(())
                })
                .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}
