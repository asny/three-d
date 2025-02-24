use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Text!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();
    let mut camera = Camera::new_2d(window.viewport());
    let mut control = Control2D::new(0.5, 100.0);

    let text_generator = TextGenerator::new(include_bytes!("font0.ttf"), 0, 30.0).unwrap();
    let text_mesh0 = text_generator.generate("Hello, World!", TextLayoutOptions::default());
    let text_mesh1 = text_generator.generate("Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Vivamus rutrum, augue vitae interdum dapibus, risus velit interdum dui, sit amet condimentum wisi sem vel odio. Nam lorem. Sed et leo sed est vehicula suscipit. Nunc volutpat, sapien non laoreet cursus, ipsum ipsum varius velit, sit amet lacinia nulla enim quis erat. Curabitur sagittis. Donec quis nulla et wisi molestie consequat. Nulla vel neque. Proin dignissim volutpat leo. 
	Suspendisse ac libero sit amet leo bibendum aliquam. Pellentesque nisl. Etiam sed sem et purus convallis mattis. Sed fringilla eros id risus. 
	Aliquam fermentum mattis lectus. Nunc luctus. Integer accumsan pede quis risus. Vestibulum et ante. 
	Morbi dolor. In nisl. Curabitur malesuada. 
	Morbi tincidunt semper tortor. Maecenas hendrerit. Vivamus fermentum ante ut wisi. Nunc mattis. Praesent nunc. Suspendisse potenti. Morbi sapien. 
	Quisque sapien libero, ornare eget, tincidunt semper, convallis vel, sem. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; ", TextLayoutOptions { line_height: 1.1 });

    let text_generator = TextGenerator::new(include_bytes!("font1.ttf"), 0, 100.0).unwrap();
    let text_mesh2 = text_generator.generate("Hi!\nHow are you?", TextLayoutOptions::default());

    // Create models
    let mut text0 = Gm::new(
        Mesh::new(&context, &text_mesh0),
        ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
    );
    text0.set_transformation(
        Mat4::from_translation(vec3(
            camera.viewport().width as f32 * 0.5,
            camera.viewport().height as f32 * 0.5,
            0.0,
        )) * Mat4::from_scale(10.0),
    );

    let mut text1 = Gm::new(
        Mesh::new(&context, &text_mesh1),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    text1.set_transformation(Mat4::from_translation(vec3(
        50.0,
        camera.viewport().height as f32 - 50.0,
        0.0,
    )));

    let mut text2 = Gm::new(
        Mesh::new(&context, &text_mesh2),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    text2.set_transformation(Mat4::from_translation(vec3(
        50.0,
        camera.viewport().height as f32 - 450.0,
        0.0,
    )));

    // Render loop
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(
            &mut camera,
            &mut frame_input.events,
            frame_input.device_pixel_ratio,
        );

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .render(&camera, &[&text0, &text1, &text2], &[]);
        FrameOutput::default()
    });
}
