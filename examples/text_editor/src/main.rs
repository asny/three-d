use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Text editor!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();
    let mut camera = Camera::new_2d(window.viewport());

    let text_generator = TextGenerator::new(include_bytes!("Roboto-Regular.ttf"), 0, 30.0).unwrap();
    let mut text_string = "Write something here:\n".to_string();

    let mut text = Gm::new(
        Mesh::new(
            &context,
            &text_generator.generate(&text_string, TextLayoutOptions::default()),
        ),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    text.set_transformation(Mat4::from_translation(vec3(
        50.0,
        camera.viewport().height as f32 - 50.0,
        0.0,
    )));

    // Render loop
    window.render_loop(move |frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);

        let mut text_changed = false;
        for event in frame_input.events.iter() {
            match event {
                Event::Text(t) => {
                    text_string.push_str(&t);
                    text_changed = true;
                }
                Event::KeyPress { kind, .. } => match *kind {
                    Key::Backspace => {
                        text_string.pop();
                        text_changed = true;
                    }
                    Key::Enter => {
                        text_string.push('\n');
                        text_changed = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if text_changed {
            let text_mesh = text_generator.generate(&text_string, TextLayoutOptions::default());
            text.geometry = Mesh::new(&context, &text_mesh);
            text.set_transformation(Mat4::from_translation(vec3(
                50.0,
                camera.viewport().height as f32 - 50.0,
                0.0,
            )));
        }
        change |= text_changed;

        if change {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(&camera, &text, &[]);
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
