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
        vec3(1200.0, 100.0, 0.0),
        vec3(0.0, 100.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        50000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(1.0);
    let mut gui = three_d::GUI::new(&context).unwrap();

    let mut loaded = Loader::load_async(
        &[
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/Sponza.gltf",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/Sponza.bin",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/10381718147657362067.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/10388182081421875623.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/11474523244911310074.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/11490520546946913238.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/11872827283454512094.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/11968150294050148237.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/1219024358953944284.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/12501374198249454378.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/13196865903111448057.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/13824894030729245199.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/13982482287905699490.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/14118779221266351425.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/14170708867020035030.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/14267839433702832875.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/14650633544276105767.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/15295713303328085182.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/15722799267630235092.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/16275776544635328252.png",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/16299174074766089871.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/16885566240357350108.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/17556969131407844942.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/17876391417123941155.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2051777328469649772.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2185409758123873465.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2299742237651021498.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2374361008830720677.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2411100444841994089.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2775690330959970771.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/2969916736137545357.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/332936164838540657.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/3371964815757888145.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/3455394979645218238.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/3628158980083700836.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/3827035219084910048.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/4477655471536070370.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/4601176305987539675.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/466164707995436622.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/4675343432951571524.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/4871783166746854860.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/4910669866631290573.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/4975155472559461469.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/5061699253647017043.png",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/5792855332885324923.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/5823059166183034438.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/6047387724914829168.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/6151467286084645207.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/6593109234861095314.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/6667038893015345571.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/6772804448157695701.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/7056944414013900257.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/715093869573992647.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/7268504077753552595.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/7441062115984513793.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/755318871556304029.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/759203620573749278.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/7645212358685992005.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/7815564343179553343.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8006627369776289000.png",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8051790464816141987.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8114461559286000061.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8481240838833932244.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8503262930880235456.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8747919177698443163.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8750083169368950601.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8773302468495022225.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/8783994986360286082.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/9288698199695299068.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/9916269861720640319.jpg",
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Sponza/glTF/white.png",
        ]
    ).await.unwrap();

    let (cpu_meshes, cpu_materials) = loaded.gltf("Sponza.gltf").unwrap();

    let mut materials = Vec::new();
    for m in cpu_materials.iter() {
        materials.push(DeferredPhysicalMaterial::new(&context, &m).unwrap());
    }

    let mut models = Vec::new();
    let mut aabb = AxisAlignedBoundingBox::EMPTY;
    for m in cpu_meshes.iter() {
        let material = materials
            .iter()
            .find(|material| &material.name == m.material_name.as_ref().unwrap())
            .unwrap()
            .clone();
        let m = Model::new_with_material(&context, &m, material).unwrap();
        aabb.expand_with_aabb(&m.aabb());
        models.push(m);
    }

    let size = aabb.size();
    let min = aabb.min() + vec3(size.x * 0.1, size.y * 0.1, size.z * 0.4);
    let max = aabb.max() - vec3(size.x * 0.1, size.y * 0.1, size.z * 0.4);
    let light_box = AxisAlignedBoundingBox::new_with_positions(&[min, max]);
    let mut lights = Vec::new();
    for _ in 0..20 {
        lights.push(Glow::new(&context, light_box));
    }

    // main loop
    let mut intensity = 0.2;
    let mut constant = 0.0;
    let mut linear = 0.0025;
    let mut quadratic = 0.00001;
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = frame_input.viewport.width;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(Slider::new::<f32>(&mut intensity, 0.0..=10.0).text("Light intensity"));
                    ui.add(
                        Slider::new::<f32>(&mut constant, 0.0..=10.0).text("Attenuation constant"),
                    );
                    ui.add(Slider::new::<f32>(&mut linear, 0.0..=0.01).text("Attenuation linear"));
                    ui.add(
                        Slider::new::<f32>(&mut quadratic, 0.0..=0.00001)
                            .text("Attenuation quadratic"),
                    );
                });
                panel_width = gui_context.used_size().x as u32;
            })
            .unwrap();

            for light in lights.iter_mut() {
                light.light.intensity = intensity;
                light.light.attenuation = Attenuation {
                    constant,
                    linear,
                    quadratic,
                };
                light.update(frame_input.elapsed_time as f32);
            }

            camera
                .set_viewport(Viewport {
                    x: panel_width as i32,
                    y: 0,
                    width: frame_input.viewport.width - panel_width,
                    height: frame_input.viewport.height,
                })
                .unwrap();

            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            pipeline
                .render_pass(
                    &camera,
                    &models.iter().map(|m| (m, &m.material)).collect::<Vec<_>>(),
                )
                .unwrap();

            Screen::write(
                &context,
                ClearState::color_and_depth(0.2, 0.2, 0.8, 1.0, 1.0),
                || {
                    pipeline.lighting_pass(
                        &camera,
                        &lights
                            .iter()
                            .map(|l| &l.light as &dyn Light)
                            .collect::<Vec<_>>(),
                    )?;
                    gui.render()?;
                    Ok(())
                },
            )
            .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}

struct Glow {
    pub light: PointLight,
    velocity: Vec3,
    aabb: AxisAlignedBoundingBox,
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
            light: PointLight::new(&context, 1.0, Color::WHITE, &pos, Attenuation::default())
                .unwrap(),
            velocity: vec3(
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            )
            .normalize(),
        }
    }

    pub fn update(&mut self, time: f32) {
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
        self.light.position += 0.2 * self.velocity * time;
    }
}
