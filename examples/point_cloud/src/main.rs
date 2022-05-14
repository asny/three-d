// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::core::*;
use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Point Cloud!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.25, -0.5, -2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 0.1, 100.0);

    let point_material = PhysicalMaterial {
        name: "point_material".to_string(),
        roughness: 0.7,
        metallic: 0.5,
        render_states: RenderStates {
            cull: Cull::Back,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut loaded = Loader::load_async(&["examples/assets/hand.pcd"])
        .await
        .unwrap();
    let cpu_point_cloud = loaded.pcd("examples/assets/hand.pcd", true, true).unwrap();

    // let cpu_point_cloud = CpuPointCloud::random(250_000);

    let point_cloud = Gm {
        geometry: PointCloud::new(&context, cpu_point_cloud).unwrap(),
        material: point_material,
    };

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional1 =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();
    let directional2 =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0)).unwrap();

    // main loop
    window
        .render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();
            redraw |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            if redraw {
                frame_input
                    .screen()
                    .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                    .unwrap()
                    .render(
                        &camera,
                        &[&point_cloud],
                        &[&ambient, &directional1, &directional2],
                    )
                    .unwrap();
            }

            FrameOutput {
                swap_buffers: redraw,
                ..Default::default()
            }
        })
        .unwrap();
}
