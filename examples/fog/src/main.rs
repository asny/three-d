// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}

use three_d::*;

pub async fn run(screenshot: Option<std::path::PathBuf>) {
    let window = Window::new(WindowSettings {
        title: "Fog!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let pipeline = ForwardPipeline::new(&context).unwrap();
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(4.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(0.05);

    let mut loaded =
        Loader::load_async(&["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"])
            .await
            .unwrap();

    let (meshes, materials) = loaded.obj("suzanne.obj").unwrap();
    let mut monkey_material = PhysicalMaterial::new(&context, &materials[0]).unwrap();
    monkey_material.render_states.cull = Cull::Back;
    let monkey = Model::new_with_material(&context, &meshes[0], monkey_material);

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();

    // Fog
    let mut fog_effect = FogEffect::new(&context).unwrap();
    fog_effect.color = vec3(0.8, 0.8, 0.8);
    let mut fog_enabled = true;

    // main loop
    let mut depth_texture = None;
    window
        .render_loop(move |mut frame_input| {
            let mut change = frame_input.first_frame;
            change |= camera.set_viewport(frame_input.viewport).unwrap();
            change |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::KeyPress { kind, .. } => {
                        if *kind == Key::F {
                            fog_enabled = !fog_enabled;
                            change = true;
                            println!("Fog: {:?}", fog_enabled);
                        }
                    }
                    _ => {}
                }
            }

            // draw
            if change && fog_enabled {
                depth_texture = Some(
                    pipeline
                        .depth_pass_texture(&camera, &[monkey.as_ref().unwrap()])
                        .unwrap(),
                );
            }

            Screen::write(&context, ClearState::default(), || {
                monkey
                    .as_ref()
                    .unwrap()
                    .render(&camera, &[&ambient, &directional])?;
                if fog_enabled {
                    if let Some(ref depth_texture) = depth_texture {
                        fog_effect.apply(
                            &camera,
                            depth_texture,
                            frame_input.accumulated_time as f32,
                        )?;
                    }
                }
                Ok(())
            })
            .unwrap();

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
