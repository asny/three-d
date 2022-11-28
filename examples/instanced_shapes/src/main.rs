use three_d::*;

/// Function to calculate the translation and rotation of each cube based on time and cube count.
fn calculate_cube_transforms(count: usize, time: f32) -> Vec<(Vec3, Quaternion<f32>)> {
    let mut result = Vec::with_capacity(count);
    let cube_separation = 3.0; // distance between center of cubes.
    let grid_side = ((count as f32).powf(1.0 / 3.0)) as usize; // cube root of count.
    let mut x_index = 0;
    let mut y_index = 0;
    let mut z_index = 0;
    for _i in 0..count {
        // Calculate the actual position.
        let x = x_index as f32 * cube_separation;
        let y = y_index as f32 * cube_separation;
        let z = z_index as f32 * cube_separation;
        let translation = vec3(x, y, z);

        // Rotate cubes based on time, this makes it easier to see performance.
        let rotation = Mat3::from_angle_x(Rad(time)).into();
        result.push((translation, rotation));

        // Advance the row, column and layer indices.
        x_index += 1;
        if x_index >= grid_side {
            x_index = 0;
            y_index += 1;
        }
        if y_index >= grid_side {
            x_index = 0;
            y_index = 0;
            z_index += 1;
        }
    }
    result
}

/// Adjust the vector of normal cubes, and update their translations.
fn adjust_normal_cubes(
    context: &Context,
    time: f32,
    count: usize,
    cubes: &mut Vec<Gm<Mesh, PhysicalMaterial>>,
) {
    // Ensure we have the correct number of cubes, does no work if already correctly sized.
    cubes.resize_with(count, || {
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
    let mut cube_transforms = calculate_cube_transforms(count, time);
    for ((position, rotation), cube) in cube_transforms.drain(..).zip(cubes.iter_mut()) {
        cube.set_transformation(Mat4::from_translation(position) * Mat4::from(rotation));
    }
}

/// Adjust the instanced cubes with their new position and rotation.
fn adjust_instanced_cubes(
    _context: &Context,
    time: f32,
    count: usize,
    cubes: &mut Gm<InstancedMesh, PhysicalMaterial>,
) {
    // Allocate vectors to set the instanced transforms.
    let mut rotations = Vec::with_capacity(count);
    let mut translations = Vec::with_capacity(count);

    // Calculate the cube transforms and split these into rotation and translation.
    let mut cube_transforms = calculate_cube_transforms(count, time);
    for (position, rotation) in cube_transforms.drain(..) {
        translations.push(position);
        rotations.push(rotation);
    }

    // Create the new Instances object with these properties and assign it to the InstancedMesh.
    let new_instances = three_d::renderer::geometry::Instances {
        translations,
        rotations: Some(rotations),
        ..Default::default()
    };
    cubes.set_instances(&new_instances);
}

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
    let mut non_instanced_meshes: Vec<Gm<Mesh, PhysicalMaterial>> = vec![];

    // Instanced mesh object, initialise with empty instances.
    let instances: three_d::renderer::geometry::Instances = Default::default();
    let mut instanced_mesh: Gm<InstancedMesh, PhysicalMaterial> = Gm::new(
        InstancedMesh::new(&context, &instances, &CpuMesh::cube()),
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

    // Initial properties of the example, 10 cubes and non instanced.
    let mut cube_count = 10;
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
                    ui.add(Slider::new(&mut cube_count, 1..=10000).text("Number of cubes."));
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

        // Always update the transforms for both the normal cubes as well as the instanced versions.
        // This shows that the difference in frame rate is not because of updating the transforms
        // and shows that the performance difference is not related to how we update the cubes.

        adjust_normal_cubes(&context, time, cube_count, &mut non_instanced_meshes);
        adjust_instanced_cubes(&context, time, cube_count, &mut instanced_mesh);

        // Then, based on whether or not we render the instanced cubes, collect the renderable
        // objects.
        let render_objects: Vec<&dyn Object> = if is_instanced {
            instanced_mesh.into_iter().collect::<_>()
        } else {
            non_instanced_meshes
                .iter()
                .map(|x| x as &dyn Object)
                .collect::<_>()
        };

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, render_objects, &[&light0, &light1])
            .write(|| gui.render());

        FrameOutput::default()
    });
}
