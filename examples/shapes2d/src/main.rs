use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings::default()).unwrap();
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

    window.render_loop(move |frame_input| {
        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button,
                position,
                modifiers,
                ..
            } = event
            {
                if *button == MouseButton::Left && !modifiers.ctrl {
                    rectangle.set_center(position);
                }
                if *button == MouseButton::Right && !modifiers.ctrl {
                    circle.set_center(position);
                }
                if *button == MouseButton::Left && modifiers.ctrl {
                    let ep = line.end_point1();
                    line.set_endpoints(position, ep);
                }
                if *button == MouseButton::Right && modifiers.ctrl {
                    let ep = line.end_point0();
                    line.set_endpoints(ep, position);
                }
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
