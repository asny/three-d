// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Picking Instances!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut cube_mesh = CpuMesh::cube();
    cube_mesh
        .transform(&(Mat4::from_translation(vec3(-2.0, 0.0, 0.0)) * Mat4::from_scale(0.5)))
        .unwrap();
    let mut cube = Gm::new(
        Mesh::new(&context, &cube_mesh),
        PhysicalMaterial::new_opaque(
            &context,
            &mut CpuMaterial {
                albedo: Srgba::WHITE,
                ..Default::default()
            },
        ),
    );

    let mut sphere_instances = Instances {
        transformations: [
            Mat4::from_translation(vec3(2.0, 0.0, 0.0)),
            Mat4::from_translation(vec3(4.0, 0.0, 0.0)),
        ]
        .to_vec(),
        ..Default::default()
    };

    let mut sphere_mesh = CpuMesh::sphere(8);
    sphere_mesh.transform(&Mat4::from_scale(0.5)).unwrap();
    let mut spheres = Gm::new(
        InstancedMesh::new(&context, &sphere_instances, &sphere_mesh),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::WHITE,
                ..Default::default()
            },
        ),
    );

    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(-1.0, -0.8, -1.2));

    let mut loaded = three_d_asset::io::load_async(&["examples/assets/suzanne.obj"])
        .await
        .unwrap();

    let model = loaded.deserialize("suzanne.obj").unwrap();
    let mut monkey = Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    let monkey_color = monkey[0].material.albedo;
    monkey.iter_mut().for_each(|m| {
        m.material.albedo = Srgba::WHITE;
        m.material.render_states.cull = Cull::Back;
    });

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
                    if let Some(pick) = pick_instance(
                        &context,
                        &camera,
                        position,
                        cube.into_iter().chain(&spheres).chain(&monkey),
                    ) {
                        if pick.index == 0 {
                            println!("Clicked cube");
                            cube.material.albedo = Srgba::GREEN;
                        } else {
                            cube.material.albedo = Srgba::WHITE;
                        }

                        if pick.index == 1 {
                            println!(
                                "Clicked sphere {}",
                                pick.instance.map_or("N/A".to_string(), |i| i.to_string())
                            );
                            sphere_instances.colors = Some(
                                sphere_instances
                                    .transformations
                                    .iter()
                                    .enumerate()
                                    .map(|(i, _)| {
                                        if Some(i) == pick.instance {
                                            Srgba::RED
                                        } else {
                                            Srgba::WHITE
                                        }
                                    })
                                    .collect(),
                            );
                        } else {
                            sphere_instances.colors = None;
                        }
                        spheres.set_instances(&sphere_instances);

                        if pick.index > 1 && pick.index <= 1 + monkey.len() {
                            println!("Clicked monkey");
                            monkey
                                .iter_mut()
                                .for_each(|m| m.material.albedo = monkey_color);
                        } else {
                            monkey
                                .iter_mut()
                                .for_each(|m| m.material.albedo = Srgba::WHITE);
                        }

                        change = true;
                    } else {
                        println!("Nothing Clicked!")
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
                    monkey.into_iter().chain(&cube).chain(&spheres),
                    &[&ambient, &directional],
                );
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
