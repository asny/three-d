use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        window.viewport().unwrap(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)).unwrap(),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        )
        .unwrap(),
    );
    sphere.set_transformation(Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2));
    let mut cylinder = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(16)).unwrap(),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        )
        .unwrap(),
    );
    cylinder
        .set_transformation(Mat4::from_translation(vec3(1.3, 0.0, 0.0)) * Mat4::from_scale(0.2));
    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()).unwrap(),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 100,
                },
                ..Default::default()
            },
        )
        .unwrap(),
    );
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
            camera.set_viewport(frame_input.viewport);
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .unwrap()
                .render(
                    &camera,
                    &[
                        &sphere,
                        &cylinder,
                        &cube,
                        &axes,
                        &bounding_box_sphere,
                        &bounding_box_cube,
                        &bounding_box_cylinder,
                    ],
                    &[&light0, &light1],
                )
                .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}
