use cgmath::point3;
use three_d::*;
use three_d_asset::ProjectionType;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Camera View!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    );
    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 0.2, 1000.0);

    let mut debug_camera = Camera::new_orthographic(
        window.viewport(),
        vec3(10.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        2.0,
        -1000.0,
        1000.0,
    );

    let axes = Axes::new(&context, 0.1, 2.0);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, 0.5, 0.5));

    // Shapes to represent the main camera view input through the debug camera
    let mut marker_sphere = CpuMesh::sphere(16);
    marker_sphere.transform(Mat4::from_scale(0.15)).unwrap();
    let mut marker_arrow = CpuMesh::arrow(0.8, 0.6, 16);
    marker_arrow
        .transform(Mat4::from_nonuniform_scale(1.0, 0.15, 0.15))
        .unwrap();

    let mut position_marker = Gm::new(
        Mesh::new(&context, &marker_sphere),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(255, 255, 0),
                ..Default::default()
            },
        ),
    );
    let mut target_marker = Gm::new(
        Mesh::new(&context, &marker_sphere),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(0, 255, 255),
                ..Default::default()
            },
        ),
    );
    let mut up_marker = Gm::new(
        Mesh::new(&context, &marker_arrow),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::GREEN,
                ..Default::default()
            },
        ),
    );

    let mut view_position_marker = Gm::new(
        Mesh::new(&context, &marker_sphere),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(255, 127, 0),
                ..Default::default()
            },
        ),
    );
    let mut view_direction_marker = Gm::new(
        Mesh::new(&context, &marker_arrow),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::BLUE,
                ..Default::default()
            },
        ),
    );

    // Shapes to view the main camera view frustum (projection output) through the debug camera
    let mut marker_vertex = CpuMesh::sphere(16);
    marker_vertex.transform(Mat4::from_scale(0.05)).unwrap();
    let mut marker_edge = CpuMesh::cylinder(16);
    marker_edge
        .transform(Mat4::from_nonuniform_scale(1.0, 0.05, 0.05))
        .unwrap();

    let cube_points = [
        point3(-1.0, -1.0, -1.0),
        point3(-1.0, -1.0, 1.0),
        point3(-1.0, 1.0, 1.0),
        point3(-1.0, 1.0, -1.0),
        point3(1.0, 1.0, -1.0),
        point3(1.0, 1.0, 1.0),
        point3(1.0, -1.0, 1.0),
        point3(1.0, -1.0, -1.0),
    ];
    let xy_edges = [0, 3, 1, 2, 4, 7, 5, 6, 0, 7, 1, 6, 2, 5, 3, 4];
    let mut frustum_vertex_marker = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &marker_vertex),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(255, 0, 255),
                ..Default::default()
            },
        ),
    );
    let mut frustum_edge_marker = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &marker_edge),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(255, 0, 255),
                ..Default::default()
            },
        ),
    );

    // Initial properties of the example
    let mut camera_ratio = 0.5;
    let mut height = 2.0;
    let mut fov = if let ProjectionType::Perspective { field_of_view_y } = camera.projection_type()
    {
        (*field_of_view_y).into()
    } else {
        degrees(45.0f32)
    };
    let mut near_plane = camera.z_near();
    let mut far_plane = camera.z_far();
    let mut show_click_info = false;

    let mut gui = three_d::GUI::new(&context);
    window.render_loop(move |mut frame_input| {
        // Gui panel to control the projection used for the main camera and enable picking debug features
        let mut panel_width = 0.0;
        let mut camera_changed = false;
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
                    ComboBox::from_label("Camera mode")
                        .selected_text(match camera.projection_type() {
                            ProjectionType::Perspective { .. } => "Perspective",
                            ProjectionType::Orthographic { .. } => "Orthographic",
                            ProjectionType::Planar { .. } => "Planar",
                        })
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(
                                    matches!(
                                        camera.projection_type(),
                                        ProjectionType::Orthographic { .. }
                                    ),
                                    "Orthographic",
                                )
                                .clicked()
                            {
                                camera.set_orthographic_projection(height, near_plane, far_plane);
                            };
                            if ui
                                .selectable_label(
                                    matches!(
                                        camera.projection_type(),
                                        ProjectionType::Perspective { .. }
                                    ),
                                    "Perspective",
                                )
                                .clicked()
                            {
                                camera.set_perspective_projection(fov, near_plane, far_plane)
                            }
                            if ui
                                .selectable_label(
                                    matches!(
                                        camera.projection_type(),
                                        ProjectionType::Planar { .. }
                                    ),
                                    "Planar",
                                )
                                .clicked()
                            {
                                camera.set_planar_projection(fov, near_plane, far_plane)
                            }
                        });
                    camera_changed |= ui
                        .add(match camera.projection_type() {
                            ProjectionType::Orthographic { .. } => {
                                Slider::new(&mut height, 0.0..=120.0).text("Height")
                            }
                            _ => Slider::new(&mut fov.0, -120.0..=120.0)
                                .text("FOV")
                                .suffix("°"),
                        })
                        .changed();
                    camera_changed |= ui
                        .add(Slider::new(&mut near_plane, -100.0..=100.0).text("Near plane"))
                        .changed();
                    if near_plane >= far_plane {
                        near_plane = far_plane - 0.1;
                    }
                    camera_changed |= ui
                        .add(Slider::new(&mut far_plane, -100.0..=100.0).text("Far plane"))
                        .changed();
                    if near_plane >= far_plane {
                        far_plane = near_plane + 0.1;
                    }

                    if camera_changed {
                        match camera.projection_type() {
                            ProjectionType::Orthographic { .. } => {
                                camera.set_orthographic_projection(height, near_plane, far_plane)
                            }
                            ProjectionType::Perspective { .. } => {
                                if near_plane < 0.0 {
                                    near_plane = 0.1;
                                    if far_plane < near_plane {
                                        far_plane = near_plane + 0.1;
                                    }
                                }
                                if fov <= degrees(0.0) {
                                    fov = degrees(1.0);
                                }
                                camera.set_perspective_projection(fov, near_plane, far_plane)
                            }
                            ProjectionType::Planar { .. } => {
                                camera.set_planar_projection(fov, near_plane, far_plane)
                            }
                        }
                    }

                    ui.add(Label::new(
                        "Left camera can be changed by the above inputs; right \
                         camera shows a debug visual for the left camera. In the \
                         debug visual, the yellow sphere is the camera position, \
                         the cyan sphere is the camera target, the green arrow \
                         is the camera up direction, and the magenta lines \
                         outline the camera's view frustum.",
                    ));

                    ui.add(Checkbox::new(
                        &mut show_click_info,
                        "Enable position/view_direction debug visuals",
                    ));

                    ui.add(Label::new(
                        "When enabled, clicking shows the camera base position \
                         as an orange sphere and the view direction as a blue \
                         arrow.",
                    ));

                    ui.add(Slider::new(&mut camera_ratio, 0.01..=0.99).text("Splitscreen ratio"));
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: ((frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32) as f32
                * camera_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);

        let debug_viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32
                + ((frame_input.viewport.width
                    - (panel_width * frame_input.device_pixel_ratio) as u32)
                    as f32
                    * camera_ratio) as i32,
            y: 0,
            width: ((frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32) as f32
                * (1.0 - camera_ratio)) as u32,
            height: frame_input.viewport.height,
        };
        debug_camera.set_viewport(debug_viewport);

        // Camera control must be after the gui update.
        control.handle_events(&mut camera, &mut frame_input.events);

        // Update camera debug shapes
        position_marker.set_transformation(Mat4::from_translation(camera.position()));
        target_marker.set_transformation(Mat4::from_translation(camera.target()));
        up_marker.set_transformation(
            Mat4::from_translation(camera.position())
                * rotation_matrix_from_dir_to_dir(Vec3::unit_x(), camera.up()),
        );

        let frustum_vertices = cube_points.map(|point| {
            (camera.projection() * camera.view())
                .inverse_transform()
                .unwrap()
                .transform_point(point)
        });
        frustum_vertex_marker.set_instances(&Instances {
            transformations: frustum_vertices
                .into_iter()
                .map(|vertex| Mat4::from_translation(vertex.to_vec()))
                .collect(),
            ..Default::default()
        });

        let frustum_edge_vertices: Vec<_> = frustum_vertices
            .iter()
            .chain(xy_edges.map(|index| &frustum_vertices[index]))
            .collect();
        let frustum_edges = frustum_edge_vertices.chunks_exact(2);
        frustum_edge_marker.set_instances(&Instances {
            transformations: frustum_edges
                .map(|points| {
                    Mat4::from_translation(points[0].to_vec())
                        * rotation_matrix_from_dir_to_dir(Vec3::unit_x(), points[1] - points[0])
                        * Mat4::from_nonuniform_scale(points[1].distance(*points[0]), 1.0, 1.0)
                })
                .collect(),
            ..Default::default()
        });

        if show_click_info {
            for event in &frame_input.events {
                if let Event::MousePress {
                    button: MouseButton::Left,
                    handled: false,
                    position,
                    ..
                } = event
                {
                    view_position_marker.set_transformation(Mat4::from_translation(
                        camera.position_at_pixel(*position),
                    ));
                    view_direction_marker.set_transformation(
                        Mat4::from_translation(camera.position_at_pixel(*position))
                            * rotation_matrix_from_dir_to_dir(
                                Vec3::unit_x(),
                                camera.view_direction_at_pixel(*position),
                            ),
                    );

                    break;
                }
            }
        }

        let screen = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        screen.render(&camera, axes.into_iter(), &[&light0, &light1]);
        screen.render(
            &debug_camera,
            axes.into_iter()
                .chain(&position_marker)
                .chain(&target_marker)
                .chain(&up_marker)
                .chain(&frustum_vertex_marker)
                .chain(&frustum_edge_marker)
                .chain(
                    show_click_info // only include the view position/direction markers if enabled
                        .then_some(
                            view_position_marker
                                .into_iter()
                                .chain(&view_direction_marker),
                        )
                        .into_iter()
                        .flatten(),
                ),
            &[&light0, &light1],
        );

        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}
