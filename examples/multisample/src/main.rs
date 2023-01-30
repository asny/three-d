use three_d::*;

#[derive(PartialEq)]
enum RenderMethod {
    Direct,
    ToTexture,
    ToMultisampledTexture(u32),
}

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Multisample!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut render_steps = RenderMethod::ToTexture;

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        ColorMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    cube.set_transformation(Mat4::from_translation(vec3(-1.0, 0.0, 0.0)) * Mat4::from_scale(0.4));

    let mut thin_beams = vec![];

    let beam_rotation = Mat4::from_angle_z(degrees(90.0));
    let beam_scale = Mat4::from_nonuniform_scale(20.0, 0.001, 0.001);
    for i in 0..3 {
        let mut beam = Gm::new(
            Mesh::new(&context, &CpuMesh::cylinder(3)),
            ColorMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                    ..Default::default()
                },
            ),
        );
        beam.set_transformation(
            Mat4::from_translation(vec3(2.75 + (0.25 * i as f32), -10.0, 0.0))
                * beam_rotation
                * beam_scale,
        );
        thin_beams.push(beam);
    }

    let mut gui = three_d::GUI::new(&context);

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);

        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.radio_value(&mut render_steps, RenderMethod::Direct, "Direct (MSAA x4)");
                    ui.radio_value(
                        &mut render_steps,
                        RenderMethod::ToTexture,
                        "To texture (No MSAA)",
                    );
                    ui.radio_value(
                        &mut render_steps,
                        RenderMethod::ToMultisampledTexture(1),
                        "To texture (MSAA x1)",
                    );
                    ui.radio_value(
                        &mut render_steps,
                        RenderMethod::ToMultisampledTexture(2),
                        "To texture (MSAA x2)",
                    );
                    ui.radio_value(
                        &mut render_steps,
                        RenderMethod::ToMultisampledTexture(4),
                        "To texture (MSAA x4)",
                    );
                    ui.radio_value(
                        &mut render_steps,
                        RenderMethod::ToMultisampledTexture(8),
                        "To texture (MSAA x8)",
                    );
                });
                panel_width = gui_context.used_rect().width() as f64;
            },
        );

        // Set up rendering for this frame:

        // slowly rotate cube, to better show off aliasing
        cube.set_transformation(Mat4::from_angle_y(radians(
            (frame_input.accumulated_time * 0.0005) as f32,
        )));

        //  Consistent clear state and iterator of renderable things for each render method
        let clear_state = ClearState::color_and_depth(0.4, 0.4, 0.4, 1.0, 1.0);
        let renderable_things = cube
            .into_iter()
            .chain(thin_beams.iter().map(|x| x as &dyn Object));

        // Render according to the selected render steps
        match render_steps {
            RenderMethod::Direct => {
                // Render the shapes directly to the screen.
                frame_input
                    .screen()
                    .clear(clear_state)
                    .render(&camera, renderable_things, &[]);
            }

            RenderMethod::ToTexture => {
                // Render the shapes to a non-multisample texture, and copy the color texture to the screen.
                let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
                    &context,
                    frame_input.viewport.width,
                    frame_input.viewport.height,
                    Interpolation::Nearest,
                    Interpolation::Nearest,
                    None,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                let mut depth_texture = DepthTexture2D::new::<f32>(
                    &context,
                    frame_input.viewport.width,
                    frame_input.viewport.height,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );

                RenderTarget::new(
                    color_texture.as_color_target(None),
                    depth_texture.as_depth_target(),
                )
                .clear(clear_state)
                .render(&camera, renderable_things, &[]);

                frame_input.screen().copy_from_color(
                    ColorTexture::Single(&color_texture),
                    frame_input.viewport,
                    WriteMask::default(),
                );
            }

            RenderMethod::ToMultisampledTexture(sample_count) => {
                // Render the shapes to a multisampled render target, resolve that render target into a non-multisampled color texture,
                // and copy that color texture to the screen.
                let color_texture = RenderTargetMultisample::<[u8; 4], f32>::new(
                    &context,
                    frame_input.viewport.width,
                    frame_input.viewport.height,
                    sample_count,
                )
                .clear(clear_state)
                .render(&camera, renderable_things, &[])
                .resolve_color();

                frame_input.screen().clear(clear_state).copy_from_color(
                    ColorTexture::Single(&color_texture),
                    frame_input.viewport,
                    WriteMask::default(),
                );
            }
        };

        // Render GUI to screen
        frame_input.screen().write(|| gui.render());

        FrameOutput::default()
    });
}
