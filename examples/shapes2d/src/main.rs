use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes 2D!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut rectangle = Gm::new(
        Rectangle::new(&context, vec2(200.0, 200.0), degrees(45.0), 100.0, 200.0),
        ColorMaterial {
            color: Color::RED,
            ..Default::default()
        },
    );
    let mut circle = Gm::new(
        Circle::new(&context, vec2(500.0, 500.0), 200.0),
        ColorMaterial {
            color: Color::BLUE,
            ..Default::default()
        },
    );
    let mut line = Gm::new(
        Line::new(
            &context,
            vec2(0.0, 0.0),
            vec2(
                window.viewport().width as f32,
                window.viewport().height as f32,
            ),
            5.0,
        ),
        ColorMaterial {
            color: Color::GREEN,
            ..Default::default()
        },
    );

    window.render_loop(move |frame_input: FrameInput| {
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
                        let ep = line.end_point1();
                        line.set_endpoints(pos, ep);
                    }
                    if *button == MouseButton::Right && modifiers.ctrl {
                        let ep = line.end_point0();
                        line.set_endpoints(ep, pos);
                    }
                }
                _ => {}
            }
        }
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera2d(frame_input.viewport),
                line.into_iter().chain(&rectangle).chain(&circle),
                &[],
            );

        FrameOutput::default()
    });
}
