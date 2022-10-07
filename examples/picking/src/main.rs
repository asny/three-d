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
        vec3(4.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(0.05)).unwrap();
    let mut pick_mesh = Gm::new(
        Mesh::new(&context, &sphere),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Color::RED,
                ..Default::default()
            },
        ),
    );

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));

    let mut loaded = three_d_asset::io::load_async(&["examples/assets/suzanne.obj"])
        .await
        .unwrap();

    let model = loaded.deserialize("suzanne.obj").unwrap();
    let mut monkey = Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    monkey
        .iter_mut()
        .for_each(|m| m.material.render_states.cull = Cull::Back);

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);

        for event in frame_input.events.iter() {
            match event {
                Event::MousePress {
                    button, position, ..
                } => {
                    if *button == MouseButton::Left {
                        let pixel = (
                            (frame_input.device_pixel_ratio * position.0) as f32,
                            (frame_input.viewport.height as f64
                                - frame_input.device_pixel_ratio * position.1)
                                as f32,
                        );
                        if let Some(pick) = pick(&context, &camera, pixel, &monkey) {
                            pick_mesh.set_transformation(Mat4::from_translation(pick));
                            change = true;
                        }
                    }
                }
                _ => {}
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
                    monkey.into_iter().chain(&pick_mesh),
                    &[&ambient, &directional],
                );
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
