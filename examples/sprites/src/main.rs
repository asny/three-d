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
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 15.0, 15.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.1);

    let axes = Axes::new(&context, 0.1, 1.0);

    let img = three_d_asset::io::load_async(&["examples/assets/test_texture.jpg"])
        .await
        .unwrap()
        .deserialize("")
        .unwrap();
    let material = ColorMaterial {
        color: Color::WHITE,
        texture: Some(std::sync::Arc::new(Texture2D::new(&context, &img)).into()),
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
    );

    let sprites_up = Sprites::new(
        &context,
        &[
            vec3(5.0, 0.0, -5.0),
            vec3(0.0, 0.0, -10.0),
            vec3(-5.0, 0.0, -5.0),
        ],
        Some(vec3(0.0, 1.0, 0.0)),
    );

    let sprites = Sprites::new(
        &context,
        &[
            vec3(20.0, 0.0, -5.0),
            vec3(15.0, 0.0, -10.0),
            vec3(10.0, 0.0, -5.0),
        ],
        Some(vec3(1.0, 1.0, 0.0).normalize()),
    );

    let ambient = AmbientLight::new(&context, 1.0, Color::WHITE);

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                axes.into_iter()
                    .chain(&Gm {
                        geometry: &billboards,
                        material: &material,
                    })
                    .chain(&Gm {
                        geometry: &sprites_up,
                        material: &material,
                    })
                    .chain(&Gm {
                        geometry: &sprites,
                        material: &material,
                    }),
                &[&ambient],
            );

        FrameOutput::default()
    });
}
