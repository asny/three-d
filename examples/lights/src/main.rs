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
    let context = window.gl().unwrap();

    let mut pipeline = DeferredPipeline::new(&context).unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(10.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        5000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(0.01);
    let mut gui = three_d::GUI::new(&context).unwrap();

    let mut loaded = three_d_asset::io::load_async(&[
        "examples/assets/sponza/Sponza.gltf",
        "examples/assets/sponza/Sponza.bin",
        "examples/assets/sponza/10381718147657362067.jpeg",
        "examples/assets/sponza/10388182081421875623.jpeg",
        "examples/assets/sponza/11474523244911310074.jpeg",
        "examples/assets/sponza/11490520546946913238.jpeg",
        "examples/assets/sponza/11872827283454512094.jpeg",
        "examples/assets/sponza/11968150294050148237.jpeg",
        "examples/assets/sponza/1219024358953944284.jpeg",
        "examples/assets/sponza/12501374198249454378.jpeg",
        "examples/assets/sponza/13196865903111448057.jpeg",
        "examples/assets/sponza/13824894030729245199.jpeg",
        "examples/assets/sponza/13982482287905699490.jpeg",
        "examples/assets/sponza/14118779221266351425.jpeg",
        "examples/assets/sponza/14170708867020035030.jpeg",
        "examples/assets/sponza/14267839433702832875.jpeg",
        "examples/assets/sponza/14650633544276105767.jpeg",
        "examples/assets/sponza/15295713303328085182.jpeg",
        "examples/assets/sponza/15722799267630235092.jpeg",
        "examples/assets/sponza/16275776544635328252.png",
        "examples/assets/sponza/16299174074766089871.jpeg",
        "examples/assets/sponza/16885566240357350108.jpeg",
        "examples/assets/sponza/17556969131407844942.jpeg",
        "examples/assets/sponza/17876391417123941155.jpeg",
        "examples/assets/sponza/2051777328469649772.jpeg",
        "examples/assets/sponza/2185409758123873465.jpeg",
        "examples/assets/sponza/2299742237651021498.jpeg",
        "examples/assets/sponza/2374361008830720677.jpeg",
        "examples/assets/sponza/2411100444841994089.jpeg",
        "examples/assets/sponza/2775690330959970771.jpeg",
        "examples/assets/sponza/2969916736137545357.jpeg",
        "examples/assets/sponza/332936164838540657.jpeg",
        "examples/assets/sponza/3371964815757888145.jpeg",
        "examples/assets/sponza/3455394979645218238.jpeg",
        "examples/assets/sponza/3628158980083700836.jpeg",
        "examples/assets/sponza/3827035219084910048.jpeg",
        "examples/assets/sponza/4477655471536070370.jpeg",
        "examples/assets/sponza/4601176305987539675.jpeg",
        "examples/assets/sponza/466164707995436622.jpeg",
        "examples/assets/sponza/4675343432951571524.jpeg",
        "examples/assets/sponza/4871783166746854860.jpeg",
        "examples/assets/sponza/4910669866631290573.jpeg",
        "examples/assets/sponza/4975155472559461469.jpeg",
        "examples/assets/sponza/5061699253647017043.png",
        "examples/assets/sponza/5792855332885324923.jpeg",
        "examples/assets/sponza/5823059166183034438.jpeg",
        "examples/assets/sponza/6047387724914829168.jpeg",
        "examples/assets/sponza/6151467286084645207.jpeg",
        "examples/assets/sponza/6593109234861095314.jpeg",
        "examples/assets/sponza/6667038893015345571.jpeg",
        "examples/assets/sponza/6772804448157695701.jpeg",
        "examples/assets/sponza/7056944414013900257.jpeg",
        "examples/assets/sponza/715093869573992647.jpeg",
        "examples/assets/sponza/7268504077753552595.jpeg",
        "examples/assets/sponza/7441062115984513793.jpeg",
        "examples/assets/sponza/755318871556304029.jpeg",
        "examples/assets/sponza/759203620573749278.jpeg",
        "examples/assets/sponza/7645212358685992005.jpeg",
        "examples/assets/sponza/7815564343179553343.jpeg",
        "examples/assets/sponza/8006627369776289000.png",
        "examples/assets/sponza/8051790464816141987.jpeg",
        "examples/assets/sponza/8114461559286000061.jpeg",
        "examples/assets/sponza/8481240838833932244.jpeg",
        "examples/assets/sponza/8503262930880235456.jpeg",
        "examples/assets/sponza/8747919177698443163.jpeg",
        "examples/assets/sponza/8750083169368950601.jpeg",
        "examples/assets/sponza/8773302468495022225.jpeg",
        "examples/assets/sponza/8783994986360286082.jpeg",
        "examples/assets/sponza/9288698199695299068.jpeg",
        "examples/assets/sponza/9916269861720640319.jpeg",
        "examples/assets/sponza/white.png",
    ])
    .await
    .unwrap();

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
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = 0.0;
            gui.update(&mut frame_input, |gui_context| {
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
                panel_width = gui_context.used_size().x as f64;
            })
            .unwrap();
            while lights.len() < light_count {
                lights.push(Glow::new(&context, light_box).unwrap());
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
            camera.set_viewport(viewport).unwrap();

            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            pipeline
                .render_pass(
                    &camera,
                    &model.iter().map(|m| (m, &m.material)).collect::<Vec<_>>(),
                )
                .unwrap();

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.2, 0.2, 0.8, 1.0, 1.0))
                .unwrap()
                .write(|| {
                    for light in lights.iter() {
                        light.render(&camera)?;
                    }
                    pipeline.lighting_pass(
                        &camera,
                        &lights
                            .iter()
                            .map(|l| &l.light as &dyn Light)
                            .collect::<Vec<_>>(),
                    )?;
                    gui.render()?;
                    Ok(())
                })
                .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}

struct Glow {
    light: PointLight,
    velocity: Vec3,
    aabb: AxisAlignedBoundingBox,
    sphere: Gm<Mesh, PhysicalMaterial>,
}

impl Glow {
    pub fn new(context: &Context, aabb: AxisAlignedBoundingBox) -> ThreeDResult<Self> {
        let mut rng = rand::thread_rng();
        let pos = vec3(
            aabb.min().x + rng.gen::<f32>() * aabb.size().x,
            aabb.min().y + rng.gen::<f32>() * aabb.size().y,
            aabb.min().z + rng.gen::<f32>() * aabb.size().z,
        );
        Ok(Self {
            aabb,
            light: PointLight::new(&context, 1.0, Color::WHITE, &pos, Attenuation::default())?,
            velocity: vec3(
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            )
            .normalize(),
            sphere: Gm::new(
                Mesh::new(context, &CpuMesh::sphere(16))?,
                PhysicalMaterial::default(),
            ),
        })
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

    pub fn render(&self, camera: &Camera) -> ThreeDResult<()> {
        self.sphere.render(camera, &[])
    }
}
