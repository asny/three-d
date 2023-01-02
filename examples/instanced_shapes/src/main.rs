use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Instanced Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(60.00, 50.0, 60.0), // camera position
        vec3(0.0, 0.0, 0.0),     // camera target
        vec3(0.0, 1.0, 0.0),     // camera up
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    // Container for non instanced meshes.
    let mut non_instanced_meshes = Vec::new();

    // Instanced mesh object, initialise with empty instances.
    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 128,
                    g: 128,
                    b: 128,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    instanced_mesh.set_animation(|time| Mat4::from_angle_x(Rad(time)));

    // Initial properties of the example, 2 cubes per side and non instanced.
    let mut side_count = 2;
    let mut is_instanced = false;

    let mut gui = three_d::GUI::new(&context);
    window.render_loop(move |mut frame_input: FrameInput| {
        // Gui panel to control the number of cubes and whether or not instancing is turned on.
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(
                        Slider::new(&mut side_count, 1..=25).text("Number of cubes at each side."),
                    );
                    ui.add(Checkbox::new(&mut is_instanced, "Use Instancing"));
                    ui.add(Label::new(
                        "Increase the cube count until the cubes don't rotate \
                                       smoothly anymore, then toggle on instancing. The rotations \
                                       should become smooth again.",
                    ));
                });
                panel_width = gui_context.used_rect().width() as f64;
            },
        );
        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);

        // Camera control must be after the gui update.
        control.handle_events(&mut camera, &mut frame_input.events);

        // Ensure we have the correct number of cubes, does no work if already correctly sized.
        let count = side_count * side_count * side_count;
        if non_instanced_meshes.len() != count {
            non_instanced_meshes.clear();
            for i in 0..count {
                let mut gm = Gm::new(
                    Mesh::new(&context, &CpuMesh::cube()),
                    PhysicalMaterial::new(
                        &context,
                        &CpuMaterial {
                            albedo: Color {
                                r: 128,
                                g: 128,
                                b: 128,
                                a: 255,
                            },
                            ..Default::default()
                        },
                    ),
                );
                let x = (i % side_count) as f32;
                let y = ((i as f32 / side_count as f32).floor() as usize % side_count) as f32;
                let z = (i as f32 / side_count.pow(2) as f32).floor();
                gm.set_transformation(Mat4::from_translation(
                    3.0 * vec3(x, y, z) - 1.5 * (side_count as f32) * vec3(1.0, 1.0, 1.0),
                ));
                gm.set_animation(|time| Mat4::from_angle_x(Rad(time)));
                non_instanced_meshes.push(gm);
            }
        }

        if instanced_mesh.instance_count() != count as u32 {
            instanced_mesh.set_instances(&Instances {
                transformations: (0..count)
                    .map(|i| {
                        let x = (i % side_count) as f32;
                        let y =
                            ((i as f32 / side_count as f32).floor() as usize % side_count) as f32;
                        let z = (i as f32 / side_count.pow(2) as f32).floor();
                        Mat4::from_translation(
                            3.0 * vec3(x, y, z) - 1.5 * (side_count as f32) * vec3(1.0, 1.0, 1.0),
                        )
                    })
                    .collect(),
                ..Default::default()
            });
        }

        // Always update the transforms for both the normal cubes as well as the instanced versions.
        // This shows that the difference in frame rate is not because of updating the transforms
        // and shows that the performance difference is not related to how we update the cubes.
        let time = (frame_input.accumulated_time * 0.001) as f32;
        instanced_mesh.animate(time);
        non_instanced_meshes
            .iter_mut()
            .for_each(|m| m.animate(time));

        // Then, based on whether or not we render the instanced cubes, collect the renderable
        // objects.
        let render_objects: Vec<&dyn Object> = if is_instanced {
            instanced_mesh.into_iter().collect()
        } else {
            non_instanced_meshes
                .iter()
                .map(|x| x as &dyn Object)
                .collect()
        };

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, render_objects, &[&light0, &light1])
            .write(|| gui.render());

        FrameOutput::default()
    });
}
