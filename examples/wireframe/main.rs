use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Wireframe!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let gl = window.gl().unwrap();

    // Renderer
    let target = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut pipeline = DeferredPipeline::new(&gl).unwrap();
    let mut camera = CameraControl::new(
        Camera::new_perspective(
            &gl,
            target + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
            target,
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            window.viewport().unwrap().aspect(),
            0.1,
            1000.0,
        )
        .unwrap(),
    );

    Loader::load(
        &[
            "./examples/assets/suzanne.obj",
            "./examples/assets/suzanne.mtl",
        ],
        move |loaded| {
            let (mut meshes, mut materials) = loaded.obj("./examples/assets/suzanne.obj").unwrap();
            let cpu_mesh = meshes.remove(0);
            let cpu_material = materials.remove(0);
            let mut model = Mesh::new_with_material(
                &gl,
                &cpu_mesh,
                &Material::new(&gl, &cpu_material).unwrap(),
            )
            .unwrap();
            model.transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
            model.cull = CullType::Back;

            let wireframe_material = Material {
                name: "wireframe".to_string(),
                color_source: ColorSource::Color(vec4(0.9, 0.2, 0.2, 1.0)),
                ..Default::default()
            };
            let mut edges = InstancedMesh::new_with_material(
                &gl,
                &edge_transformations(&cpu_mesh),
                &CPUMesh::cylinder(0.007, 1.0, 10),
                &wireframe_material,
            )
            .unwrap();
            edges.transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
            edges.cull = CullType::Back;

            let mut vertices = InstancedMesh::new_with_material(
                &gl,
                &vertex_transformations(&cpu_mesh),
                &CPUMesh::sphere(0.015),
                &wireframe_material,
            )
            .unwrap();
            vertices.transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
            vertices.cull = CullType::Back;

            let mut plane = Mesh::new_with_material(
                &gl,
                &CPUMesh {
                    positions: vec![
                        vec3(-10000.0, -1.0, 10000.0),
                        vec3(10000.0, -1.0, 10000.0),
                        vec3(0.0, -1.0, -10000.0),
                    ],
                    normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                    ..Default::default()
                },
                &Material {
                    color_source: ColorSource::Color(vec4(1.0, 1.0, 1.0, 1.0)),
                    ..Default::default()
                },
            )
            .unwrap();
            plane.cull = CullType::Back;

            let mut spot_light0 = SpotLight::new(
                &gl,
                0.2,
                &vec3(1.0, 1.0, 1.0),
                &vec3(5.0, 7.0, 5.0),
                &vec3(-1.0, -1.0, -1.0),
                25.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();
            let mut spot_light1 = SpotLight::new(
                &gl,
                0.2,
                &vec3(1.0, 1.0, 1.0),
                &vec3(-5.0, 7.0, 5.0),
                &vec3(1.0, -1.0, -1.0),
                25.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();
            let mut spot_light2 = SpotLight::new(
                &gl,
                0.2,
                &vec3(1.0, 1.0, 1.0),
                &vec3(-5.0, 7.0, -5.0),
                &vec3(1.0, -1.0, 1.0),
                25.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();
            let mut spot_light3 = SpotLight::new(
                &gl,
                0.2,
                &vec3(1.0, 1.0, 1.0),
                &vec3(5.0, 7.0, -5.0),
                &vec3(-1.0, -1.0, 1.0),
                25.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();
            spot_light0
                .generate_shadow_map(50.0, 512, &[&model, &edges, &vertices])
                .unwrap();
            spot_light1
                .generate_shadow_map(50.0, 512, &[&model, &edges, &vertices])
                .unwrap();
            spot_light2
                .generate_shadow_map(50.0, 512, &[&model, &edges, &vertices])
                .unwrap();
            spot_light3
                .generate_shadow_map(50.0, 512, &[&model, &edges, &vertices])
                .unwrap();

            // main loop
            let mut rotating = false;
            window
                .render_loop(move |frame_input| {
                    let mut redraw = frame_input.first_frame;
                    redraw |= camera.set_aspect(frame_input.viewport.aspect()).unwrap();

                    for event in frame_input.events.iter() {
                        match event {
                            Event::MouseClick { state, button, .. } => {
                                rotating = *button == MouseButton::Left && *state == State::Pressed;
                            }
                            Event::MouseMotion { delta, .. } => {
                                if rotating {
                                    camera
                                        .rotate_around_with_fixed_up(
                                            &target,
                                            0.1 * delta.0 as f32,
                                            0.1 * delta.1 as f32,
                                        )
                                        .unwrap();
                                    redraw = true;
                                }
                            }
                            Event::MouseWheel { delta, .. } => {
                                camera
                                    .zoom_towards(&target, 0.1 * delta.1 as f32, 3.0, 100.0)
                                    .unwrap();
                                redraw = true;
                            }
                            _ => {}
                        }
                    }

                    if redraw {
                        // Geometry pass
                        pipeline
                            .geometry_pass(
                                frame_input.viewport.width,
                                frame_input.viewport.height,
                                &camera,
                                &[&model, &edges, &vertices, &plane],
                            )
                            .unwrap();

                        // Light pass
                        Screen::write(&gl, ClearState::default(), || {
                            pipeline.light_pass(
                                frame_input.viewport,
                                &camera,
                                None,
                                &[],
                                &[&spot_light0, &spot_light1, &spot_light2, &spot_light3],
                                &[],
                            )?;
                            Ok(())
                        })
                        .unwrap();
                    }

                    if args.len() > 1 {
                        // To automatically generate screenshots of the examples, can safely be ignored.
                        FrameOutput {
                            screenshot: Some(args[1].clone().into()),
                            exit: true,
                            ..Default::default()
                        }
                    } else {
                        FrameOutput {
                            swap_buffers: redraw,
                            wait_next_event: true,
                            ..Default::default()
                        }
                    }
                })
                .unwrap();
        },
    );
}

fn vertex_transformations(cpu_mesh: &CPUMesh) -> Vec<Mat4> {
    cpu_mesh
        .positions
        .iter()
        .copied()
        .map(Mat4::from_translation)
        .collect()
}

fn edge_transformations(cpu_mesh: &CPUMesh) -> Vec<Mat4> {
    let mut edge_transformations = std::collections::HashMap::new();
    let indices = cpu_mesh.indices.as_ref().unwrap().into_u32();
    for f in 0..indices.len() / 3 {
        let mut fun = |i1, i2| {
            let p1: Vec3 = cpu_mesh.positions[i1];
            let p2: Vec3 = cpu_mesh.positions[i2];
            let scale = Mat4::from_nonuniform_scale((p1 - p2).magnitude(), 1.0, 1.0);
            let rotation =
                rotation_matrix_from_dir_to_dir(vec3(1.0, 0.0, 0.0), (p2 - p1).normalize());
            let translation = Mat4::from_translation(p1);
            let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            edge_transformations.insert(key, translation * rotation * scale);
        };
        let i1 = indices[3 * f] as usize;
        let i2 = indices[3 * f + 1] as usize;
        let i3 = indices[3 * f + 2] as usize;
        fun(i1, i2);
        fun(i1, i3);
        fun(i2, i3);
    }
    edge_transformations
        .drain()
        .map(|(_, v)| v)
        .collect::<Vec<Mat4>>()
}
