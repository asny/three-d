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
        title: "Logo!".to_string(),
        max_size: Some((1280, 300)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 2.2),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10.0,
    )
    .unwrap();

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
    )
    .unwrap();
    let mut model = Model::new_with_material(&context, &cpu_mesh, material).unwrap();
    model.set_transformation(Mat4::from_angle_y(degrees(35.0)));

    let mut loaded = Loader::load_async(
        &["examples/assets/syferfontein_18d_clear_4k.hdr"], // Source: https://polyhaven.com/
    )
    .await
    .unwrap();
    let environment_map = TextureCubeMap::new_from_equirectangular::<Vector3<f16>>(
        &context,
        &loaded.hdr_image("").unwrap(),
    )
    .unwrap();
    let light = AmbientLight {
        environment: Some(Environment::new(&context, &environment_map).unwrap()),
        ..Default::default()
    };

    window
        .render_loop(move |frame_input: FrameInput| {
            camera.set_viewport(frame_input.viewport).unwrap();
            Screen::write(
                &context,
                ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0),
                || {
                    /*model.set_transformation(Mat4::from_angle_y(radians(
                        (frame_input.accumulated_time * 0.0002) as f32,
                    )));*/
                    model.render(&camera, &[&light])?;
                    Ok(())
                },
            )
            .unwrap();

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
