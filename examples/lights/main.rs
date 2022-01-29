use rand::prelude::*;
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Lights!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 100.0, 0.0),
        vec3(0.0, 100.0, -1.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        50000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(1.0);

    let scene = Loading::new(
        &context,
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
            "examples/assets/chinese_garden_4k.hdr",
        ],
        move |context, mut loaded| {
            let environment_map = loaded.hdr_image("chinese").unwrap();
            let skybox = Skybox::new_with_texture(
                &context,
                TextureCubeMap::<f16>::new_from_equirectangular(&context, &environment_map)
                    .unwrap(),
            )
            .unwrap();

            let (cpu_meshes, cpu_materials) = loaded.gltf("Sponza.gltf").unwrap();

            let mut materials = Vec::new();
            for m in cpu_materials.iter() {
                materials.push(PhysicalMaterial::new(&context, &m).unwrap());
            }

            let mut models = Vec::new();
            for m in cpu_meshes.iter() {
                let material = materials.iter().find(|material| &material.name == m.material_name.as_ref().unwrap()).unwrap().clone();
                models.push(Model::new_with_material(&context, &m, material).unwrap());
            }
            Ok((models, skybox))
        },
    );

    let mut rng = rand::thread_rng();
    let mut directional = Vec::new();
    for _ in 0..0 {
        directional.push(
            DirectionalLight::new(
                &context,
                0.2,
                Color::GREEN,
                &vec3(rng.gen::<f32>(), -1.0, rng.gen::<f32>()),
            )
            .unwrap(),
        );
    }

    let mut spot = Vec::new();
    for _ in 0..0 {
        spot.push(
            SpotLight::new(
                &context,
                2.0,
                Color::BLUE,
                &vec3(
                    100.0 * rng.gen::<f32>(),
                    100.0 + rng.gen::<f32>(),
                    100.0 * rng.gen::<f32>(),
                ),
                &vec3(rng.gen::<f32>(), -1.0, rng.gen::<f32>()),
                degrees(25.0),
                0.001,
                0.00001,
                0.000001,
            )
            .unwrap(),
        );
    }

    let mut point = Vec::new();
    for _ in 0..20 {
        point.push(
            PointLight::new(
                &context,
                0.4,
                Color::WHITE,
                &vec3(
                    1000.0 * rng.gen::<f32>() - 500.0,
                    100.0,
                    1000.0 * rng.gen::<f32>() - 500.0,
                ),
                0.005,
                0.0005,
                0.00005,
            )
            .unwrap(),
        );
    }

    // main loop
    window
        .render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            let time = 0.001 * frame_input.accumulated_time;
            let c = time.cos() as f32;
            let s = time.sin() as f32;
            for light in directional.iter_mut() {
                light.direction += vec3(-1.0 - c, -1.0, 1.0 + s);
            }
            for light in spot.iter_mut() {
                light.position += vec3(3.0 + c, 0.0 + s, 3.0 - s);
                light.direction += -vec3(3.0 + c, 5.0 + s, 3.0 - s);
            }
            for light in point.iter_mut() {
                light.position += vec3(-5.0 * c, 0.0, -5.0 * s);
            }

            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    if let Some(ref scene) = *scene.borrow() {
                        let (models, skybox) = scene.as_ref().unwrap();
                        skybox.render(&camera)?;
                        for model in models {
                            model.render(
                                &camera,
                                &point
                                    .iter()
                                    .map(|l| l as &dyn Light)
                                    .chain(directional.iter().map(|l| l as &dyn Light))
                                    .chain(spot.iter().map(|l| l as &dyn Light))
                                    .collect::<Vec<_>>(),
                            )?;
                        }
                    }
                    Ok(())
                },
            )
            .unwrap();

            if args.len() > 1 {
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
