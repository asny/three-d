// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Point Cloud!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.125, -0.25, -0.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.01,
        100.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 0.1, 3.0);

    // Load point cloud
    let mut loaded = three_d_asset::io::load_async(&["examples/assets/hand.pcd"])
        .await
        .unwrap();
    let cpu_point_cloud: PointCloud = loaded.deserialize("hand.pcd").unwrap();

    let mut point_mesh = CpuMesh::sphere(4);
    point_mesh.transform(&Mat4::from_scale(0.001)).unwrap();

    let mut point_cloud = Gm {
        geometry: InstancedMesh::new(&context, &cpu_point_cloud.into(), &point_mesh),
        material: ColorMaterial::default(),
    };
    let c = -point_cloud.aabb().center();
    point_cloud.set_transformation(Mat4::from_translation(c));

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
                    point_cloud
                        .into_iter()
                        .chain(&Axes::new(&context, 0.01, 0.1)),
                    &[],
                );
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}
