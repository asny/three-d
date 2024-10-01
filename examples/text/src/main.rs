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

    let text_generator = TextGenerator::new(include_bytes!("font0.ttf"), 30.0);
    let text_mesh0 = text_generator.generate("Hello, World!");
    let text_mesh1 = text_generator.generate("Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Vivamus rutrum, augue vitae interdum dapibus, risus velit interdum dui, sit amet condimentum wisi sem vel odio. Nam lorem. Sed et leo sed est vehicula suscipit. Nunc volutpat, sapien non laoreet cursus, ipsum ipsum varius velit, sit amet lacinia nulla enim quis erat. Curabitur sagittis. Donec quis nulla et wisi molestie consequat. Nulla vel neque. Proin dignissim volutpat leo. 
	Suspendisse ac libero sit amet leo bibendum aliquam. Pellentesque nisl. Etiam sed sem et purus convallis mattis. Sed fringilla eros id risus. 
	Aliquam fermentum mattis lectus. Nunc luctus. Integer accumsan pede quis risus. Vestibulum et ante. 
	Morbi dolor. In nisl. Curabitur malesuada. 
	Morbi tincidunt semper tortor. Maecenas hendrerit. Vivamus fermentum ante ut wisi. Nunc mattis. Praesent nunc. Suspendisse potenti. Morbi sapien. 
	Quisque sapien libero, ornare eget, tincidunt semper, convallis vel, sem. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; ");

    let text_generator = TextGenerator::new(include_bytes!("font1.ttf"), 100.0);
    let text_mesh2 = text_generator.generate("Hi!\nHow are you?");

    // Create models
    let mut text0 = Gm::new(
        Mesh::new(&context, &text_mesh0),
        ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
    );
    text0.set_transformation(Mat4::from_scale(10.0));

    let mut text1 = Gm::new(
        Mesh::new(&context, &text_mesh1),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    text1.set_transformation(Mat4::from_translation(vec3(50.0, 500.0, 0.0)));

    let mut text2 = Gm::new(
        Mesh::new(&context, &text_mesh2),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    text2.set_transformation(Mat4::from_translation(vec3(1000.0, -200.0, 0.0)));

    // Render loop
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
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .render(&camera, &[&text0, &text1, &text2], &[]);
        FrameOutput::default()
    });
}
