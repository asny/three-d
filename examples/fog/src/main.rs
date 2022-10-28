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

    let mut loaded = three_d_asset::io::load_async(&["examples/assets/suzanne.obj"])
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
    let fog_effect = FogEffect {
        color: Color::new_opaque(200, 200, 200),
        density: 0.2,
        animation: 0.1,
    };
    let mut fog_enabled = true;

    // main loop
    let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
        &context,
        1,
        1,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    let mut depth_texture =
        DepthTexture2D::new::<f32>(&context, 1, 1, Wrapping::ClampToEdge, Wrapping::ClampToEdge);
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
        change |= control.handle_events(&mut camera, &mut frame_input.events);

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

        if change {
            // Draw the scene to a render target if a change has occured
            color_texture = Texture2D::new_empty::<[u8; 4]>(
                &context,
                frame_input.viewport.width,
                frame_input.viewport.height,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            depth_texture = DepthTexture2D::new::<f32>(
                &context,
                frame_input.viewport.width,
                frame_input.viewport.height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            RenderTarget::new(
                color_texture.as_color_target(None),
                depth_texture.as_depth_target(),
            )
            .clear(ClearState::default())
            .render(&camera, &monkey, &[&ambient, &directional]);
        }

        if fog_enabled {
            // Apply fog nomatter if a change has occured since it contain animation.
            frame_input
                .screen()
                .copy_from(
                    ColorTexture::Single(&color_texture),
                    DepthTexture::Single(&depth_texture),
                    frame_input.viewport,
                    WriteMask::default(),
                )
                .write(|| {
                    fog_effect.apply(
                        &context,
                        frame_input.accumulated_time,
                        &camera,
                        DepthTexture::Single(&depth_texture),
                    )
                });
        } else if change {
            // If a change has happened and no fog is applied, copy the result to the screen
            frame_input.screen().copy_from_color(
                ColorTexture::Single(&color_texture),
                frame_input.viewport,
                WriteMask::default(),
            );
        }

        FrameOutput {
            swap_buffers: change || fog_enabled,
            ..Default::default()
        }
    });
}
