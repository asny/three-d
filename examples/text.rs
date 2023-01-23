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
        text: "Your mama".to_owned(),
        size: 40.,
    };

    let texture2d = font.rasterize(effect, &context);
    let ratio = texture2d.width as f32 / texture2d.height as f32;
    let material = ColorMaterial::new_transparent(
        &context,
        &CpuMaterial {
            albedo_texture: Some(texture2d),
            ..Default::default()
        },
    );

    let mut billboards = Sprites::new(&context, &[vec3(5., 0.0, 0.)], None);
    billboards.set_transformation(Mat4::from_nonuniform_scale(ratio, 1.0, 1.0));

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
