use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Wireframe!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let target = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        target + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
        target,
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 0.1 * scene_radius, 100.0 * scene_radius);

    let scene = Loading::new(
        &context,
        &["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"],
        move |context, mut loaded| {
            let (mut meshes, materials) = loaded.obj("suzanne.obj").unwrap();
            let mut cpu_mesh = meshes.remove(0);
            cpu_mesh.transform(&Mat4::from_translation(vec3(0.0, 2.0, 0.0)));
            let mut model = Model::new_with_material(
                &context,
                &cpu_mesh,
                PhysicalMaterial::new(&context, &materials[0]).unwrap(),
            )
            .unwrap();
            model.material.render_states.cull = Cull::Back;
            let wireframe_material = PhysicalMaterial {
                name: "wireframe".to_string(),
                albedo: Color::new_opaque(220, 50, 50),
                roughness: 0.7,
                metallic: 0.8,
                render_states: RenderStates {
                    cull: Cull::Back,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut cylinder = CpuMesh::cylinder(10);
            cylinder.transform(&Mat4::from_nonuniform_scale(1.0, 0.007, 0.007));
            let edges = InstancedModel::new_with_material(
                &context,
                &edge_transformations(&cpu_mesh),
                &cylinder,
                wireframe_material.clone(),
            )
            .unwrap();

            let mut sphere = CpuMesh::sphere(8);
            sphere.transform(&Mat4::from_scale(0.015));
            let vertices = InstancedModel::new_with_material(
                &context,
                &vertex_transformations(&cpu_mesh),
                &sphere,
                wireframe_material,
            )
            .unwrap();
            Ok((model, vertices, edges))
        },
    );

    let ambient = AmbientLight::new(&context, 0.7, Color::WHITE).unwrap();
    let directional0 =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();
    let directional1 =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0)).unwrap();

    // main loop
    let mut loaded = false;
    window
        .render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            if !loaded && scene.is_loaded() {
                redraw = true;
                loaded = true;
            }
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();
            redraw |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            if redraw {
                Screen::write(
                    &context,
                    ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0),
                    || {
                        if let Some(ref scene) = *scene.borrow() {
                            let (model, vertices, edges) = scene.as_ref().unwrap();
                            render_pass(
                                &camera,
                                &[&model, &vertices, &edges],
                                &[&ambient, &directional0, &directional1],
                            )?;
                        }
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
                    ..Default::default()
                }
            }
        })
        .unwrap();
}

fn vertex_transformations(cpu_mesh: &CpuMesh) -> Vec<ModelInstance> {
    let mut iter = cpu_mesh.positions.iter();
    let mut instances = Vec::new();
    while let Some(v) = iter.next() {
        instances.push(ModelInstance {
            geometry_transform: Mat4::from_translation(vec3(
                *v,
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            )),
            ..Default::default()
        });
    }
    instances
}

fn edge_transformations(cpu_mesh: &CpuMesh) -> Vec<ModelInstance> {
    let mut edge_transformations = std::collections::HashMap::new();
    let indices = cpu_mesh.indices.as_ref().unwrap().into_u32();
    for f in 0..indices.len() / 3 {
        let mut fun = |i1, i2| {
            let p1 = vec3(
                cpu_mesh.positions[i1 * 3],
                cpu_mesh.positions[i1 * 3 + 1],
                cpu_mesh.positions[i1 * 3 + 2],
            );
            let p2 = vec3(
                cpu_mesh.positions[i2 * 3],
                cpu_mesh.positions[i2 * 3 + 1],
                cpu_mesh.positions[i2 * 3 + 2],
            );
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
        .map(|(_, v)| ModelInstance {
            geometry_transform: v,
            ..Default::default()
        })
        .collect::<Vec<_>>()
}
