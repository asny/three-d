use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Text!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_orthographic(
        window.viewport(),
        vec3(window.viewport().width as f32 * 0.5, 0.0, 2.0),
        vec3(window.viewport().width as f32 * 0.5, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        window.viewport().height as f32,
        0.1,
        10.0,
    );

    let font_data = include_bytes!("GrapeNuts-Regular.ttf");
    let font = FontRef::from_index(font_data, 0).expect("Failed to load font");
    let text_generator = TextGenerator::new(font);

    let mut mesh = Gm::new(
        Mesh::new(&context, &text_generator.generate("Hello, World!")),
        ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
    );
    mesh.set_transformation(Mat4::from_scale(0.25));

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        for event in frame_input.events.iter() {
            match *event {
                Event::MouseMotion { delta, button, .. } => {
                    if button == Some(MouseButton::Left) {
                        let speed = 1.3;
                        let right = camera.right_direction();
                        let up = right.cross(camera.view_direction());
                        let delta = -right * speed * delta.0 + up * speed * delta.1;
                        camera.translate(&delta);
                    }
                }
                _ => {}
            }
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &mesh, &[]);
        FrameOutput::default()
    });
}
