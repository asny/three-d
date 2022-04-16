// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Sprites!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 15.0, 15.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(0.1);

    let axes = Axes::new(&context, 0.1, 1.0).unwrap();

    let img = Loader::load_async(&["examples/assets/test_texture.jpg"])
        .await
        .unwrap()
        .image("")
        .unwrap();
    let material = ColorMaterial {
        color: Color::WHITE,
        texture: Some(std::rc::Rc::new(Texture2D::new(&context, &img).unwrap())),
        ..Default::default()
    };

    let billboards = Sprites::new(
        &context,
        &[
            vec3(-20.0, 0.0, -5.0),
            vec3(-15.0, 0.0, -10.0),
            vec3(-10.0, 0.0, -5.0),
        ],
        None,
    )
    .unwrap();

    let sprites_up = Sprites::new(
        &context,
        &[
            vec3(5.0, 0.0, -5.0),
            vec3(0.0, 0.0, -10.0),
            vec3(-5.0, 0.0, -5.0),
        ],
        Some(vec3(0.0, 1.0, 0.0)),
    )
    .unwrap();

    let sprites = Sprites::new(
        &context,
        &[
            vec3(20.0, 0.0, -5.0),
            vec3(15.0, 0.0, -10.0),
            vec3(10.0, 0.0, -5.0),
        ],
        Some(vec3(1.0, 1.0, 0.0).normalize()),
    )
    .unwrap();

    let ambient = AmbientLight::new(&context, 1.0, Color::WHITE).unwrap();

    window
        .render_loop(move |mut frame_input: FrameInput| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            Screen::write(
                &context,
                ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                || {
                    render_pass(
                        &camera,
                        &[
                            &axes,
                            &Gm {
                                geometry: &billboards,
                                material: &material,
                            },
                            &Gm {
                                geometry: &sprites_up,
                                material: &material,
                            },
                            &Gm {
                                geometry: &sprites,
                                material: &material,
                            },
                        ],
                        &[&ambient],
                    )?;
                    Ok(())
                },
            )
            .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}
