#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Text!".to_string(),
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

    let font = Font::default();
    let effect = TextEffect {
        text: 'a',
        size: 40.,
    };

    let texture2d = font.rasterize(effect, &context);

    let material = ColorMaterial {
        color: Color::WHITE,
        texture: Some(std::sync::Arc::new(texture2d).into()),
        ..Default::default()
    };

    let billboards = Sprites::new(&context, &[vec3(-20.0, 0.0, -5.0)], None);

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                axes.into_iter().chain(&Gm {
                    geometry: &billboards,
                    material: &material,
                }),
                &[],
            );

        FrameOutput::default()
    });
}
