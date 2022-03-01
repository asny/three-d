// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}

use three_d::*;

pub async fn run(screenshot: Option<std::path::PathBuf>) {
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

    let mut loaded = Loader::load_async(&[
        "examples/assets/Tree1.obj",
        "examples/assets/Tree1.mtl",
        "examples/assets/Tree1Bark.jpg",
        "examples/assets/Tree1Leave.png",
    ])
    .await
    .unwrap();
    // Tree
    let (mut meshes, materials) = loaded.obj("examples/assets/Tree1.obj").unwrap();
    let mut tree_cpu_mesh = meshes
        .iter()
        .position(|m| m.name == "tree.001_Mesh.002")
        .map(|index| meshes.remove(index))
        .unwrap();
    tree_cpu_mesh.compute_normals();
    let mut tree_mesh = Model::new_with_material(
        &context,
        &tree_cpu_mesh,
        PhysicalMaterial::new(
            &context,
            &materials
                .iter()
                .find(|m| Some(&m.name) == tree_cpu_mesh.material_name.as_ref())
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    tree_mesh.material.render_states.cull = Cull::Back;

    let mut leaves_cpu_mesh = meshes
        .iter()
        .position(|m| m.name == "leaves.001")
        .map(|index| meshes.remove(index))
        .unwrap();
    leaves_cpu_mesh.compute_normals();
    let leaves_mesh = Model::new_with_material(
        &context,
        &leaves_cpu_mesh,
        PhysicalMaterial::new(
            &context,
            &materials
                .iter()
                .find(|m| Some(&m.name) == leaves_cpu_mesh.material_name.as_ref())
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    // Lights
    let ambient = AmbientLight::new(&context, 0.3, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 4.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();

    // Imposters
    let mut aabb = tree_cpu_mesh.compute_aabb();
    aabb.expand_with_aabb(&leaves_cpu_mesh.compute_aabb());
    let mut imposters = Imposters::new(&context).unwrap();
    imposters
        .update_texture(&[&tree_mesh, &leaves_mesh], &[&ambient, &directional], 256)
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
    let mut plane = Model::new_with_material(
        &context,
        &CpuMesh {
            positions: vec![
                -10000.0, 0.0, 10000.0, 10000.0, 0.0, 10000.0, 0.0, 0.0, -10000.0,
            ],
            normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
            ..Default::default()
        },
        PhysicalMaterial {
            albedo: Color::new_opaque(128, 200, 70),
            metallic: 0.0,
            roughness: 1.0,
            ..Default::default()
        },
    )
    .unwrap();
    plane.material.render_states.cull = Cull::Back;

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
                        render_pass(
                            &camera,
                            &[&plane, &tree_mesh, &leaves_mesh],
                            &[&ambient, &directional],
                        )?;
                        imposters.render(&camera)?;
                        Ok(())
                    },
                )
                .unwrap();
            }

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
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
