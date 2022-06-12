use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes 2D!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut rectangle = Rectangle::new_with_material(
        &context,
        vec2(200.0, 200.0),
        degrees(45.0),
        100.0,
        200.0,
        ColorMaterial {
            color: Color::RED,
            ..Default::default()
        },
    )
    .unwrap();
    let mut circle = Circle::new_with_material(
        &context,
        vec2(500.0, 500.0),
        200.0,
        ColorMaterial {
            color: Color::BLUE,
            ..Default::default()
        },
    );
    let mut line = Line::new_with_material(
        &context,
        vec2(0.0, 0.0),
        vec2(
            window.viewport().unwrap().width as f32,
            window.viewport().unwrap().height as f32,
        ),
        5.0,
        ColorMaterial {
            color: Color::GREEN,
            ..Default::default()
        },
    );

    window
        .render_loop(move |frame_input: FrameInput| {
            for event in frame_input.events.iter() {
                match event {
                    Event::MousePress {
                        button,
                        position,
                        modifiers,
                        ..
                    } => {
                        let pos = vec2(
                            (frame_input.device_pixel_ratio * position.0) as f32,
                            (frame_input.device_pixel_ratio * position.1) as f32,
                        );
                        if *button == MouseButton::Left && !modifiers.ctrl {
                            rectangle.set_center(pos);
                        }
                        if *button == MouseButton::Right && !modifiers.ctrl {
                            circle.set_center(pos);
                        }
                        if *button == MouseButton::Left && modifiers.ctrl {
                            line.set_endpoints(pos, line.end_point1());
                        }
                        if *button == MouseButton::Right && modifiers.ctrl {
                            line.set_endpoints(line.end_point0(), pos);
                        }
                    }
                    _ => {}
                }
            }
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .write(|| {
                    line.render(frame_input.viewport);
                    rectangle.render(frame_input.viewport);
                    circle.render(frame_input.viewport);
                });

            FrameOutput::default()
        })
        .unwrap();
}
