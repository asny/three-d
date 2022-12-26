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
        vec3(120.00, 97.0, 122.0),   // camera position
        vec3(115.000, 93.25, 118.5), // camera target
        vec3(0.0, 1.0, 0.0),         // camera up
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(1.0);

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

    // Initial properties of the example, 2 cubes per side and non instanced.
    let mut side_count = 2;
    let mut is_instanced = false;

    let mut gui = three_d::GUI::new(&context);
    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);

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

        // Camera control must be after the gui update.
        control.handle_events(&mut camera, &mut frame_input.events);

        // Time to move the cubes.
        let time = (frame_input.accumulated_time * 0.001) as f32;
        let count = side_count * side_count * side_count;

        // Always update the transforms for both the normal cubes as well as the instanced versions.
        // This shows that the difference in frame rate is not because of updating the transforms
        // and shows that the performance difference is not related to how we update the cubes.

        // Ensure we have the correct number of cubes, does no work if already correctly sized.
        non_instanced_meshes.resize_with(count, || {
            Gm::new(
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
            )
        });

        // Finally, calculate the cube transforms and update them.
        let rotation = Mat4::from_angle_x(Rad(time));
        let mut transformations = Vec::new();
        for (i, mesh) in non_instanced_meshes.iter_mut().enumerate() {
            let x = (i % side_count) as f32;
            let y = ((i as f32 / side_count as f32).floor() as usize % side_count) as f32;
            let z = (i as f32 / side_count.pow(2) as f32).floor();
            let transformation = Mat4::from_translation(3.0 * vec3(x, y, z)) * rotation;
            mesh.set_transformation(transformation);
            transformations.push(transformation);
        }
        instanced_mesh.set_instances(&Instances {
            transformations,
            ..Default::default()
        });

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
