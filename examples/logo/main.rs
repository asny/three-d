use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

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

    let mut cpu_mesh = CPUMesh::sphere(37);
    let mut colors: Vec<u8> = vec![];
    for i in 0..4 * cpu_mesh.positions.len() / 3 {
        colors.push(if i % 4 == 3 {
            255
        } else {
            if i % 3 == 1 {
                (100 / ((i + 1) % 2 + 1)) as u8
            } else {
                (100 / ((i + 2) % 3 + 1)) as u8
            }
        })
    }
    cpu_mesh.colors = Some(colors);
    let material = PhysicalMaterial::new(
        &context,
        &CPUMaterial {
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

    let light = Loading::new(
        &context,
        &["examples/assets/syferfontein_18d_clear_4k.hdr"], // Source: https://polyhaven.com/
        move |context, mut loaded| {
            let environment_map =
                TextureCubeMap::<f32>::new_from_equirectangular(&context, &loaded.hdr_image("")?)?;
            Ok(AmbientLight {
                environment: Some(Environment::new(&context, &environment_map)?),
                ..Default::default()
            })
        },
    );

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
                    if let Some(ref light) = *light.borrow() {
                        let light = light.as_ref().unwrap();
                        model.render(&camera, &[light])?;
                    }
                    Ok(())
                },
            )
            .unwrap();

            if args.len() > 1 {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(args[1].clone().into()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
