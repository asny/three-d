// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Picking!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(2.0, 2.0, 25.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(Mat4::from_scale(0.1)).unwrap();
    let mut pick_mesh = Gm::new(
        Mesh::new(&context, &sphere),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(255, 255, 0, 255),
                ..Default::default()
            },
        ),
    );

    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(-1.0, -1.0, -1.0));

    let mut loaded = three_d_asset::io::load_async(&["examples/assets/suzanne.obj"])
        .await
        .unwrap();

    let model = loaded.deserialize("suzanne.obj").unwrap();
    let mut monkey: Gm<_, _> = Model::<PhysicalMaterial>::new(&context, &model)
        .unwrap()
        .remove(0)
        .into();
    monkey.material.render_states.cull = Cull::Back;
    monkey.set_transformation(Mat4::from_translation(vec3(2.0, -2.0, 0.0)));
    let original_color = monkey.material.albedo;

    let mut cone = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial {
            albedo: Srgba::BLUE,
            ..Default::default()
        },
    );
    cone.set_transformation(Mat4::from_translation(vec3(-2.0, 2.0, 0.0)));

    let transformations: Vec<_> = (-30..30)
        .flat_map(|i| {
            (-30..30).map(move |j| {
                Mat4::from_translation(vec3(i as f32, j as f32, 0.0)) * Mat4::from_scale(3.0)
            })
        })
        .collect();
    let no_instances = transformations.len();
    let instances = Instances {
        transformations,
        colors: Some(vec![Srgba::GREEN; no_instances]),
        ..Default::default()
    };
    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &instances, &sphere),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::WHITE,
                ..Default::default()
            },
        ),
    );

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);

        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button, position, ..
            } = *event
            {
                if button == MouseButton::Left {
                    // Reset colors and pick mesh position
                    let mut instances = instances.clone();
                    instanced_mesh.set_instances(&instances);
                    monkey.material.albedo = original_color;
                    cone.material.albedo = Srgba::BLUE;
                    pick_mesh.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 0.0)));

                    // Pick
                    if let Some(pick) = pick(
                        &context,
                        &camera,
                        position,
                        monkey.into_iter().chain(&cone).chain(&instanced_mesh),
                        Cull::Back,
                    ) {
                        pick_mesh.set_transformation(
                            Mat4::from_translation(pick.position) * Mat4::from_scale(0.3),
                        );
                        match pick.geometry_id {
                            0 => {
                                monkey.material.albedo = Srgba::RED;
                            }
                            1 => {
                                cone.material.albedo = Srgba::RED;
                            }
                            2 => {
                                instances.colors.as_mut().unwrap()[pick.instance_id as usize] =
                                    Srgba::RED;
                                instanced_mesh.set_instances(&instances);
                            }
                            _ => {
                                unreachable!()
                            }
                        };
                        change = true;
                    }
                }
            }
        }

        change |= control.handle_events(&mut camera, &mut frame_input.events);

        // draw
        if change {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(
                    &camera,
                    monkey
                        .into_iter()
                        .chain(&instanced_mesh)
                        .chain(&cone)
                        .chain(&pick_mesh),
                    &[&ambient, &directional],
                );
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
