use three_d::*;

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Wireframe!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let target = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut camera = Camera::new_perspective(
        window.viewport(),
        target + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
        target,
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 0.1 * scene_radius, 100.0 * scene_radius);

    let mut loaded = three_d_asset::io::load_async(&["examples/assets/suzanne.obj"])
        .await
        .unwrap();

    let mut cpu_mesh: CpuMesh = loaded.deserialize("suzanne.obj").unwrap();
    cpu_mesh
        .transform(&Mat4::from_translation(vec3(0.0, 2.0, 0.0)))
        .unwrap();
    let mut model_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Color::new_opaque(50, 50, 50),
            roughness: 0.7,
            metallic: 0.8,
            ..Default::default()
        },
    );
    model_material.render_states.cull = Cull::Back;
    let model = Gm::new(Mesh::new(&context, &cpu_mesh), model_material);
    let mut wireframe_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Color::new_opaque(220, 50, 50),
            roughness: 0.7,
            metallic: 0.8,
            ..Default::default()
        },
    );
    wireframe_material.render_states.cull = Cull::Back;
    let mut cylinder = CpuMesh::cylinder(10);
    cylinder
        .transform(&Mat4::from_nonuniform_scale(1.0, 0.007, 0.007))
        .unwrap();
    let edges = Gm::new(
        InstancedMesh::new(&context, &edge_transformations(&cpu_mesh), &cylinder),
        wireframe_material.clone(),
    );

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(0.015)).unwrap();
    let vertices = Gm::new(
        InstancedMesh::new(&context, &vertex_transformations(&cpu_mesh), &sphere),
        wireframe_material,
    );

    let ambient = AmbientLight::new(&context, 0.7, Color::WHITE);
    let directional0 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(
                    &camera,
                    model.into_iter().chain(&vertices).chain(&edges),
                    &[&ambient, &directional0, &directional1],
                );
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}

fn vertex_transformations(cpu_mesh: &CpuMesh) -> Instances {
    Instances {
        transformations: cpu_mesh
            .positions
            .to_f32()
            .into_iter()
            .map(|p| Mat4::from_translation(p))
            .collect(),
        ..Default::default()
    }
}

fn edge_transformations(cpu_mesh: &CpuMesh) -> Instances {
    let indices = cpu_mesh.indices.to_u32().unwrap();
    let positions = cpu_mesh.positions.to_f32();
    let mut transformations = Vec::new();
    let mut keys = Vec::new();
    for f in 0..indices.len() / 3 {
        let mut fun = |i1, i2| {
            let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            if !keys.contains(&key) {
                keys.push(key);
                let p1: Vec3 = positions[i1];
                let p2: Vec3 = positions[i2];
                transformations.push(
                    Mat4::from_translation(p1)
                        * Into::<Mat4>::into(Quat::from_arc(
                            vec3(1.0, 0.0, 0.0),
                            (p2 - p1).normalize(),
                            None,
                        ))
                        * Mat4::from_nonuniform_scale((p1 - p2).magnitude(), 1.0, 1.0),
                );
            }
        };
        let i1 = indices[3 * f] as usize;
        let i2 = indices[3 * f + 1] as usize;
        let i3 = indices[3 * f + 2] as usize;
        fun(i1, i2);
        fun(i2, i3);
        fun(i3, i1);
    }
    Instances {
        transformations,
        ..Default::default()
    }
}
