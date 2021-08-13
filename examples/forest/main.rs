use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Forest!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(180.0, 40.0, 70.0),
        vec3(0.0, 6.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(0.1);

    Loader::load(
        &[
            "examples/assets/Tree1.obj",
            "examples/assets/Tree1.mtl",
            "examples/assets/Tree1Bark.jpg",
            "examples/assets/Tree1Leave.png",
        ],
        move |mut loaded| {
            // Tree
            let (mut meshes, materials) = loaded.obj("examples/assets/Tree1.obj").unwrap();
            for mesh in meshes.iter_mut() {
                if mesh.name == "leaves.001" || mesh.name == "tree.001_Mesh.002" {
                    mesh.compute_normals();
                }
            }
            let tree_cpu_mesh = meshes
                .iter()
                .find(|m| m.name == "tree.001_Mesh.002")
                .unwrap();
            let tree_cpu_material = materials
                .iter()
                .find(|m| &m.name == tree_cpu_mesh.material_name.as_ref().unwrap())
                .unwrap();
            let tree_material = Material::new(&context, &tree_cpu_material).unwrap();
            let mut tree_mesh = Model::new(&context, tree_cpu_mesh).unwrap();
            tree_mesh.cull = CullType::Back;

            let leaves_cpu_mesh = meshes.iter().find(|m| m.name == "leaves.001").unwrap();
            let leaves_material = Material::new(
                &context,
                &materials
                    .iter()
                    .find(|m| &m.name == leaves_cpu_mesh.material_name.as_ref().unwrap())
                    .unwrap(),
            )
            .unwrap();
            let leaves_mesh = Model::new(&context, leaves_cpu_mesh).unwrap();

            // Lights
            let ambient_light = AmbientLight {
                intensity: 0.3,
                color: Color::WHITE,
            };
            let mut directional_light =
                DirectionalLight::new(&context, 4.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0))
                    .unwrap();

            // Imposters
            let mut aabb = tree_cpu_mesh.compute_aabb();
            aabb.expand_with_aabb(&leaves_cpu_mesh.compute_aabb());
            let mut imposters = Imposters::new(&context).unwrap();
            imposters
                .update_texture(
                    |camera: &Camera| {
                        tree_mesh.render_with_lighting(
                            camera,
                            &tree_material,
                            LightingModel::Blinn,
                            Some(&ambient_light),
                            &[&directional_light],
                            &[],
                            &[],
                        )?;
                        leaves_mesh.render_with_lighting(
                            camera,
                            &leaves_material,
                            LightingModel::Blinn,
                            Some(&ambient_light),
                            &[&directional_light],
                            &[],
                            &[],
                        )?;
                        Ok(())
                    },
                    (*aabb.min(), *aabb.max()),
                    256,
                )
                .unwrap();

            let t = 100;
            let mut positions = Vec::new();
            let mut angles = Vec::new();
            for x in -t..t {
                for y in -t..t {
                    if x != 0 || y != 0 {
                        positions.push(10.0 * x as f32);
                        positions.push(0.0);
                        positions.push(10.0 * y as f32);
                        angles.push(0.0);
                    }
                }
            }
            imposters.update_positions(&positions, &angles);

            // Plane
            let plane_material = Material {
                albedo: vec4(0.5, 0.7, 0.3, 1.0),
                metallic: 0.0,
                roughness: 1.0,
                ..Default::default()
            };
            let mut plane = Model::new(
                &context,
                &CPUMesh {
                    positions: vec![
                        -10000.0, 0.0, 10000.0, 10000.0, 0.0, 10000.0, 0.0, 0.0, -10000.0,
                    ],
                    normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                    ..Default::default()
                },
            )
            .unwrap();
            plane.cull = CullType::Back;

            // Shadows
            directional_light
                .generate_shadow_map(
                    &vec3(0.0, 0.0, 0.0),
                    50.0,
                    100.0,
                    512,
                    512,
                    &[&tree_mesh, &leaves_mesh],
                )
                .unwrap();

            // main loop
            window
                .render_loop(move |mut frame_input| {
                    let mut redraw = frame_input.first_frame;
                    redraw |= camera.set_viewport(frame_input.viewport).unwrap();

                    redraw |= control
                        .handle_events(&mut camera, &mut frame_input.events)
                        .unwrap();

                    if redraw {
                        Screen::write(
                            &context,
                            ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                            || {
                                plane.render_with_lighting(
                                    &camera,
                                    &plane_material,
                                    LightingModel::Blinn,
                                    Some(&ambient_light),
                                    &[&directional_light],
                                    &[],
                                    &[],
                                )?;
                                tree_mesh.render_with_lighting(
                                    &camera,
                                    &tree_material,
                                    LightingModel::Blinn,
                                    Some(&ambient_light),
                                    &[&directional_light],
                                    &[],
                                    &[],
                                )?;
                                leaves_mesh.render_with_lighting(
                                    &camera,
                                    &leaves_material,
                                    LightingModel::Blinn,
                                    Some(&ambient_light),
                                    &[&directional_light],
                                    &[],
                                    &[],
                                )?;
                                imposters.render(&camera)?;
                                Ok(())
                            },
                        )
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
