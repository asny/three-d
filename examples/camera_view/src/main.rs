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
    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let mut debug_camera = Camera::new_orthographic(
        window.viewport(),
        vec3(10.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        24.0,
        -1000.0,
        1000.0,
    );

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2));
    let mut cylinder = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    cylinder
        .set_transformation(Mat4::from_translation(vec3(1.3, 0.0, 0.0)) * Mat4::from_scale(0.2));
    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 100,
                },
                ..Default::default()
            },
        ),
    );
    cube.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 1.3)) * Mat4::from_scale(0.2));
    let axes = Axes::new(&context, 0.1, 2.0);
    let bounding_box_sphere = Gm::new(
        BoundingBox::new(&context, sphere.aabb()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    let bounding_box_cube = Gm::new(
        BoundingBox::new(&context, cube.aabb()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    let bounding_box_cylinder = Gm::new(
        BoundingBox::new(&context, cylinder.aabb()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    // Shapes to represent the main camera view through the debug camera
    let mut marker_sphere = CpuMesh::sphere(16);
    marker_sphere.transform(&Mat4::from_scale(0.15)).unwrap();
    let mut marker_arrow = CpuMesh::arrow(0.8, 0.6, 16);
    marker_arrow
        .transform(&Mat4::from_nonuniform_scale(1.0, 0.15, 0.15))
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

    let mut marker_vertex = CpuMesh::sphere(16);
    marker_vertex.transform(&Mat4::from_scale(0.05)).unwrap();
    let mut marker_edge = CpuMesh::cylinder(16);
    marker_edge
        .transform(&Mat4::from_nonuniform_scale(1.0, 0.05, 0.05))
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

    let mut gui = three_d::GUI::new(&context);
    window.render_loop(move |mut frame_input| {
        // Gui panel to control the number of cubes and whether or not instancing is turned on.
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

                    ui.add(
                        Slider::new(&mut camera_ratio, 0.01..=0.99)
                            .text("Main camera to debug camera screen ratio"),
                    );
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

        let meta_viewport = Viewport {
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
        debug_camera.set_viewport(meta_viewport);

        // Camera control must be after the gui update.
        control.handle_events(&mut camera, &mut frame_input.events);

        // Update camera debug shapes
        position_marker.set_transformation(Mat4::from_translation(*camera.position()));
        target_marker.set_transformation(Mat4::from_translation(*camera.target()));
        up_marker.set_transformation(
            Mat4::from_translation(*camera.position())
                * Mat4::from_axis_angle(
                    Vec3::unit_x().cross(*camera.up()),
                    Vec3::unit_x().angle(*camera.up()),
                ),
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
                        * Mat4::from_axis_angle(
                            Vec3::unit_x().cross(points[1] - points[0]).normalize(),
                            Vec3::unit_x().angle(points[1] - points[0]),
                        )
                        * Mat4::from_nonuniform_scale(points[1].distance(*points[0]), 1.0, 1.0)
                })
                .collect(),
            ..Default::default()
        });

        let screen = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        screen.render(
            &camera,
            sphere
                .into_iter()
                .chain(&cylinder)
                .chain(&cube)
                .chain(&axes)
                .chain(&bounding_box_sphere)
                .chain(&bounding_box_cube)
                .chain(&bounding_box_cylinder),
            &[&light0, &light1],
        );
        screen.render(
            &debug_camera,
            sphere
                .into_iter()
                .chain(&cylinder)
                .chain(&cube)
                .chain(&axes)
                .chain(&bounding_box_sphere)
                .chain(&bounding_box_cube)
                .chain(&bounding_box_cylinder)
                .chain(&position_marker)
                .chain(&target_marker)
                .chain(&up_marker)
                .chain(&frustum_vertex_marker)
                .chain(&frustum_edge_marker),
            &[&light0, &light1],
        );

        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}