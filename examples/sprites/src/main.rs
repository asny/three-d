// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a)));
}

use three_d::*;

pub fn run(screenshot: Option<std::path::PathBuf>) {
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
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let axes = Axes::new(&context, 0.1, 1.0).unwrap();

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
                    render_pass(&camera, &[&axes], &[&ambient])?;
                    Ok(())
                },
            )
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
