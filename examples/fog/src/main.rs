// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Fog!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.05);

    let mut loaded = three_d_asset::io::load_async(&[
        "examples/assets/suzanne.obj",
        "examples/assets/suzanne.mtl",
    ])
    .await
    .unwrap();

    let mut monkey =
        Model::<PhysicalMaterial>::new(&context, &loaded.deserialize("suzanne.obj").unwrap())
            .unwrap();
    monkey
        .iter_mut()
        .for_each(|m| m.material.render_states.cull = Cull::Back);

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));

    // Fog
    let fog_effect = FogEffect::new(&context, Color::new_opaque(200, 200, 200), 0.2, 0.1);
    let mut fog_enabled = true;

    // main loop
    let mut depth_texture = None;
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
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
            depth_texture = Some(DepthTargetTexture2D::new(
                &context,
                frame_input.viewport.width,
                frame_input.viewport.height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            ));
            depth_texture.as_mut().map(|dt| {
                dt.as_depth_target()
                    .clear(ClearState::default())
                    .render_with_material(
                        &DepthMaterial::default(),
                        &camera,
                        &monkey.to_geometries(),
                        &[],
                    );
            });
        }

        frame_input
            .screen()
            .clear(ClearState::default())
            .render(&camera, &monkey.to_objects(), &[&ambient, &directional])
            .write(|| {
                if fog_enabled {
                    if let Some(ref depth_texture) = depth_texture {
                        fog_effect.apply(
                            &camera,
                            depth_texture,
                            frame_input.accumulated_time as f32,
                        );
                    }
                }
            });

        FrameOutput::default()
    });
}
