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

    let mut model = Gm::new(
        Mesh::new(&context, &cpu_mesh).unwrap(),
        ColorMaterial::default(),
    );

    let mut gui = three_d::GUI::new(&context).unwrap();
    let mut viewport_zoom = 1.0;
    let mut scissor_zoom = 1.0;
    window
        .render_loop(move |mut frame_input: FrameInput| {
            model.set_transformation(Mat4::from_angle_y(radians(
                (frame_input.accumulated_time * 0.005) as f32,
            )));

            let mut panel_width = 0.0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut viewport_zoom, 0.01..=1.0).text("Viewport"));
                    ui.add(Slider::new(&mut scissor_zoom, 0.01..=1.0).text("Scissor"));
                });
                panel_width = gui_context.used_size().x as f64;
            })
            .unwrap();

            let viewport = Viewport {
                x: (panel_width * frame_input.device_pixel_ratio) as i32,
                y: 0,
                width: frame_input.viewport.width
                    - (panel_width * frame_input.device_pixel_ratio) as u32,
                height: frame_input.viewport.height,
            };

            // Main view
            let viewport_zoomed = zoom(viewport_zoom, viewport);
            let scissor_box_zoomed = zoom(scissor_zoom, viewport).into();

            camera.set_viewport(viewport_zoomed).unwrap();
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .unwrap()
                .clear_partially(
                    if viewport_zoom < scissor_zoom {
                        scissor_box_zoomed
                    } else {
                        viewport_zoomed.into()
                    },
                    ClearState::color(0.8, 0.8, 0.8, 1.0),
                )
                .unwrap()
                .clear_partially(
                    if viewport_zoom > scissor_zoom {
                        scissor_box_zoomed
                    } else {
                        viewport_zoomed.into()
                    },
                    ClearState::color(0.5, 0.5, 0.5, 1.0),
                )
                .unwrap()
                .render_partially(scissor_box_zoomed, &camera, &[&model], &[])
                .unwrap()
                .write(|| gui.render())
                .unwrap();

            // Secondary view
            let secondary_viewport = Viewport {
                x: viewport.x,
                y: viewport.y,
                width: 200,
                height: 200,
            };
            camera.set_viewport(secondary_viewport).unwrap();
            frame_input
                .screen()
                .clear_partially(
                    secondary_viewport.into(),
                    ClearState::color(0.3, 0.3, 0.3, 1.0),
                )
                .unwrap()
                .render_partially(secondary_viewport.into(), &camera, &[&model], &[])
                .unwrap();

            // Returns default frame output to end the frame
            FrameOutput::default()
        })
        .unwrap();
}

fn zoom(zoom: f32, viewport: Viewport) -> Viewport {
    let width = (viewport.width as f32 * zoom) as u32;
    let height = (viewport.height as f32 * zoom) as u32;
    Viewport {
        x: ((viewport.width - width) / 2 + viewport.x as u32) as i32,
        y: ((viewport.height - height) / 2 + viewport.y as u32) as i32,
        width,
        height,
    }
}
