use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut sphere = Model::new_with_material(
        &context,
        &CPUMesh::sphere(16),
        PhysicalMaterial {
            albedo: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 200,
            },
            ..Default::default()
        },
    )
    .unwrap();
    sphere.set_transformation(Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2));
    let mut cylinder = Model::new_with_material(
        &context,
        &CPUMesh::cylinder(16),
        PhysicalMaterial {
            albedo: Color {
                r: 0,
                g: 255,
                b: 0,
                a: 200,
            },
            ..Default::default()
        },
    )
    .unwrap();
    cylinder
        .set_transformation(Mat4::from_translation(vec3(1.3, 0.0, 0.0)) * Mat4::from_scale(0.2));
    let mut cube = Model::new_with_material(
        &context,
        &CPUMesh::cube(),
        PhysicalMaterial {
            albedo: Color {
                r: 0,
                g: 0,
                b: 255,
                a: 100,
            },
            ..Default::default()
        },
    )
    .unwrap();
    cube.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 1.3)) * Mat4::from_scale(0.2));
    let axes = Axes::new(&context, 0.1, 1.0).unwrap();
    let bounding_box_sphere = BoundingBox::new_with_material(
        &context,
        sphere.aabb(),
        ColorMaterial {
            color: Color::BLACK,
            ..Default::default()
        },
    )
    .unwrap();
    let bounding_box_cube = BoundingBox::new_with_material(
        &context,
        cube.aabb(),
        ColorMaterial {
            color: Color::BLACK,
            ..Default::default()
        },
    )
    .unwrap();
    let bounding_box_cylinder = BoundingBox::new_with_material(
        &context,
        cylinder.aabb(),
        ColorMaterial {
            color: Color::BLACK,
            ..Default::default()
        },
    )
    .unwrap();

    let light0 =
        DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5)).unwrap();
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5)).unwrap();

    window
        .render_loop(move |mut frame_input: FrameInput| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            Screen::write(
                &context,
                ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                || {
                    render_pass(
                        &camera,
                        &[
                            &sphere as &dyn Object,
                            &cylinder,
                            &cube,
                            &axes,
                            &bounding_box_sphere,
                            &bounding_box_cube,
                            &bounding_box_cylinder,
                        ],
                        &[&light0, &light1],
                    )?;
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
