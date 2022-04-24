use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Screen!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 1.3),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    )
    .unwrap();

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(vec![
            vec3(0.5, -0.5, 0.0),  // bottom right
            vec3(-0.5, -0.5, 0.0), // bottom left
            vec3(0.0, 0.5, 0.0),   // top
        ]),
        colors: Some(vec![
            Color::new(255, 0, 0, 255), // bottom right
            Color::new(0, 255, 0, 255), // bottom left
            Color::new(0, 0, 255, 255), // top
        ]),
        ..Default::default()
    };

    let mut model = Model::new(&context, &cpu_mesh).unwrap();

    let mut gui = three_d::GUI::new(&context).unwrap();
    let mut viewport_zoom = 1.0;
    let mut scissor_zoom = 1.0;
    window
        .render_loop(move |mut frame_input: FrameInput| {
            model.set_transformation(Mat4::from_angle_y(radians(
                (frame_input.accumulated_time * 0.005) as f32,
            )));

            let mut panel_width = 0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut viewport_zoom, 0.01..=1.0).text("Viewport"));
                    ui.add(Slider::new(&mut scissor_zoom, 0.01..=1.0).text("Scissor"));
                });
                panel_width = gui_context.used_size().x as u32;
            })
            .unwrap();

            let scissor_width =
                ((frame_input.viewport.width - panel_width) as f32 * scissor_zoom) as u32;
            let scissor_height = ((frame_input.viewport.height) as f32 * scissor_zoom) as u32;
            let scissor_box = ScissorBox {
                x: ((frame_input.viewport.width - panel_width - scissor_width) / 2 + panel_width)
                    as i32,
                y: ((frame_input.viewport.height - scissor_height) / 2) as i32,
                width: scissor_width,
                height: scissor_height,
            };

            let viewport_width =
                ((frame_input.viewport.width - panel_width) as f32 * viewport_zoom) as u32;
            let viewport_height = ((frame_input.viewport.height) as f32 * viewport_zoom) as u32;
            let viewport = Viewport {
                x: ((frame_input.viewport.width - panel_width - viewport_width) / 2 + panel_width)
                    as i32,
                y: ((frame_input.viewport.height - viewport_height) / 2) as i32,
                width: viewport_width,
                height: viewport_height,
            };
            camera.set_viewport(viewport).unwrap();

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .unwrap()
                .clear_partially(
                    if viewport_zoom < scissor_zoom {
                        scissor_box
                    } else {
                        viewport.into()
                    },
                    ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                )
                .unwrap()
                .clear_partially(
                    if viewport_zoom > scissor_zoom {
                        scissor_box
                    } else {
                        viewport.into()
                    },
                    ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                )
                .unwrap()
                .render_in_viewport(scissor_box, &camera, &[&model], &[])
                .unwrap()
                .write(|| gui.render())
                .unwrap();

            // Returns default frame output to end the frame
            FrameOutput::default()
        })
        .unwrap();
}
