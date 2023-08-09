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

    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));

    // Fog
    let mut fog_effect = FogEffect {
        color: Srgba::new_opaque(200, 200, 200),
        density: 0.1,
        animation: 0.1,
        ..Default::default()
    };
    let mut fog_enabled = true;

    // main loop
    let mut color_texture = Texture2D::new_empty::<[f16; 4]>(
        &context,
        camera.viewport().width,
        camera.viewport().height,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    let mut depth_texture = DepthTexture2D::new::<f32>(
        &context,
        camera.viewport().width,
        camera.viewport().height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
        change |= control.handle_events(&mut camera, &mut frame_input.events);

        for event in frame_input.events.iter() {
            if let Event::KeyPress { kind, .. } = event {
                if *kind == Key::F {
                    fog_enabled = !fog_enabled;
                    change = true;
                    println!("Fog: {:?}", fog_enabled);
                }
            }
        }

        if change {
            // Draw the scene to a render target if a change has occured
            if camera.viewport().width != color_texture.width()
                || camera.viewport().height != color_texture.height()
            {
                color_texture = Texture2D::new_empty::<[f16; 4]>(
                    &context,
                    camera.viewport().width,
                    camera.viewport().height,
                    Interpolation::Nearest,
                    Interpolation::Nearest,
                    None,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                depth_texture = DepthTexture2D::new::<f32>(
                    &context,
                    camera.viewport().width,
                    camera.viewport().height,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
            }
            camera.disable_tone_and_color_mapping();
            RenderTarget::new(
                color_texture.as_color_target(None),
                depth_texture.as_depth_target(),
            )
            .clear(ClearState::default())
            .render(&camera, &monkey, &[&ambient, &directional]);
        }

        change |= fog_enabled; // Always render if fog is enabled since it contain animation.

        if change {
            camera.set_default_tone_and_color_mapping();
            if fog_enabled {
                fog_effect.time = frame_input.accumulated_time as f32;
                frame_input.screen().apply_screen_effect(
                    &fog_effect,
                    &camera,
                    &[],
                    Some(ColorTexture::Single(&color_texture)),
                    Some(DepthTexture::Single(&depth_texture)),
                );
            } else {
                frame_input.screen().apply_screen_effect(
                    &ScreenEffect::default(),
                    &camera,
                    &[],
                    Some(ColorTexture::Single(&color_texture)),
                    Some(DepthTexture::Single(&depth_texture)),
                );
            }
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
