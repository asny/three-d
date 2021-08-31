use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Lines!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut model = Line::new(
        &context,
        (0.0, 0.0),
        (
            window.viewport().unwrap().width as f32,
            window.viewport().unwrap().height as f32,
        ),
        5.0,
    )
    .unwrap();
    window
        .render_loop(move |frame_input: FrameInput| {
            for event in frame_input.events.iter() {
                match event {
                    Event::MousePress {
                        button, position, ..
                    } => {
                        let pos = (
                            (frame_input.device_pixel_ratio * position.0) as f32,
                            (frame_input.device_pixel_ratio * position.1) as f32,
                        );
                        if *button == MouseButton::Left {
                            model.set_endpoints(pos, model.end_point1());
                        }
                        if *button == MouseButton::Right {
                            model.set_endpoints(model.end_point0(), pos);
                        }
                    }
                    _ => {}
                }
            }
            Screen::write(
                &context,
                ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                || {
                    model.render_with_color(Color::RED, frame_input.viewport)?;
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
