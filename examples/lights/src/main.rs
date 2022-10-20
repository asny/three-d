// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use rand::prelude::*;
use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Lights!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(10.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        5000.0,
    );
    let mut control = FlyControl::new(0.01);
    let mut gui = three_d::GUI::new(&context);

    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/sponza/Sponza.gltf"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&["https://asny.github.io/three-d/assets/sponza/Sponza.gltf"])
            .await
            .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let model = loaded.deserialize("Sponza.gltf").unwrap();
    let model = Model::<DeferredPhysicalMaterial>::new(&context, &model).unwrap();

    let mut aabb = AxisAlignedBoundingBox::EMPTY;
    for m in model.iter() {
        aabb.expand_with_aabb(&m.aabb());
    }

    let size = aabb.size();
    let min = aabb.min() + vec3(size.x * 0.1, size.y * 0.1, size.z * 0.4);
    let max = aabb.max() - vec3(size.x * 0.1, size.y * 0.3, size.z * 0.4);
    let light_box = AxisAlignedBoundingBox::new_with_positions(&[min, max]);
    let mut lights = Vec::new();

    // main loop
    let mut intensity = 1.0;
    let mut constant = 0.0;
    let mut linear = 0.5;
    let mut quadratic = 0.5;
    let mut light_count = 20;
    let mut color = [1.0; 4];
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(Slider::new::<usize>(&mut light_count, 0..=50).text("Light count"));
                    ui.add(Slider::new::<f32>(&mut intensity, 0.0..=10.0).text("Light intensity"));
                    ui.add(
                        Slider::new::<f32>(&mut constant, 0.0..=10.0).text("Attenuation constant"),
                    );
                    ui.add(Slider::new::<f32>(&mut linear, 0.01..=1.0).text("Attenuation linear"));
                    ui.add(
                        Slider::new::<f32>(&mut quadratic, 0.0001..=1.0)
                            .text("Attenuation quadratic"),
                    );
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });
                panel_width = gui_context.used_rect().width() as f64;
            },
        );
        while lights.len() < light_count {
            lights.push(Glow::new(&context, light_box));
        }
        while lights.len() > light_count {
            lights.pop();
        }

        for light in lights.iter_mut() {
            light.set_light(
                intensity,
                Color::from_rgba_slice(&color),
                Attenuation {
                    constant,
                    linear,
                    quadratic,
                },
            );
            light.update(0.00005 * size.magnitude() * frame_input.elapsed_time as f32);
        }
        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);

        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.2, 0.2, 0.8, 1.0, 1.0))
            .render(
                &camera,
                lights.iter().map(|l| l.object()).chain(&model),
                &lights.iter().map(|l| l.light()).collect::<Vec<_>>(),
            )
            .write(|| {
                gui.render();
            });

        FrameOutput::default()
    });
}

struct Glow {
    light: PointLight,
    velocity: Vec3,
    aabb: AxisAlignedBoundingBox,
    sphere: Gm<Mesh, PhysicalMaterial>,
}

impl Glow {
    pub fn new(context: &Context, aabb: AxisAlignedBoundingBox) -> Self {
        let mut rng = rand::thread_rng();
        let pos = vec3(
            aabb.min().x + rng.gen::<f32>() * aabb.size().x,
            aabb.min().y + rng.gen::<f32>() * aabb.size().y,
            aabb.min().z + rng.gen::<f32>() * aabb.size().z,
        );
        Self {
            aabb,
            light: PointLight::new(&context, 1.0, Color::WHITE, &pos, Attenuation::default()),
            velocity: vec3(
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            )
            .normalize(),
            sphere: Gm::new(
                Mesh::new(context, &CpuMesh::sphere(16)),
                PhysicalMaterial::default(),
            ),
        }
    }

    pub fn set_light(&mut self, intensity: f32, color: Color, attenuation: Attenuation) {
        self.light.color = color;
        self.light.intensity = intensity;
        self.light.attenuation = attenuation;
        let c = color.to_vec4() * intensity;
        self.sphere.material.emissive = Color::from_rgba_slice(&[c.x, c.y, c.z, c.w]);
    }

    pub fn update(&mut self, delta: f32) {
        let mut rng = rand::thread_rng();
        let min = self.aabb.min();
        let max = self.aabb.max();
        let p = self.light.position;
        self.velocity.x +=
            (min.x - p.x).max(0.0) - (p.x - max.x).max(0.0) + rng.gen::<f32>() * 0.1 - 0.05;
        self.velocity.y +=
            (min.y - p.y).max(0.0) - (p.y - max.y).max(0.0) + rng.gen::<f32>() * 0.1 - 0.05;
        self.velocity.z +=
            (min.z - p.z).max(0.0) - (p.z - max.z).max(0.0) + rng.gen::<f32>() * 0.1 - 0.05;
        self.velocity = self.velocity.normalize();
        self.light.position += self.velocity * delta;
        self.sphere.set_transformation(
            Mat4::from_translation(self.light.position) * Mat4::from_scale(0.02),
        );
    }

    pub fn object(&self) -> &dyn Object {
        &self.sphere
    }

    pub fn light(&self) -> &dyn Light {
        &self.light
    }
}
