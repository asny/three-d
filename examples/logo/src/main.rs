// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Logo!".to_string(),
        max_size: Some((1280, 300)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.2),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10.0,
    );

    let mut cpu_mesh = CpuMesh::sphere(37);
    let mut colors = Vec::new();
    for i in 0..cpu_mesh.positions.len() {
        let mut c = [0, 0, 0, 0];
        for j in 0..4 {
            let t = i * 4 + j;
            c[j] = if t % 4 == 3 {
                255
            } else {
                if t % 3 == 1 {
                    (100 / ((t + 1) % 2 + 1)) as u8
                } else {
                    (100 / ((t + 2) % 3 + 1)) as u8
                }
            };
        }
        colors.push(Color::new(c[0], c[1], c[2], c[3]));
    }
    cpu_mesh.colors = Some(colors);
    let material = PhysicalMaterial::new(
        &context,
        &CpuMaterial {
            roughness: 0.6,
            metallic: 0.6,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            ..Default::default()
        },
    );
    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), material);
    model.set_transformation(Mat4::from_angle_y(degrees(35.0)));

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/syferfontein_18d_clear_4k.hdr"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/syferfontein_18d_clear_4k.hdr",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };
    let environment_map =
        TextureCubeMap::new_from_equirectangular::<f16>(&context, &loaded.deserialize("").unwrap());
    let light = AmbientLight {
        environment: Some(Environment::new(&context, &environment_map)),
        ..Default::default()
    };

    window.render_loop(move |frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .render(&camera, &model, &[&light]);

        FrameOutput::default()
    });
}
