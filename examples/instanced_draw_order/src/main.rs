use three_d::*;

/*
    This example shows that for the InstancedMesh, instances are ordered by camera depth if the
    material is transparent. Transparent materials need to be drawn back-to-front, failure to do so
    results in rendering results that look incorrect. Currently, instances are ordered based on
    their origin, this means that if two instances overlap, rendering artifacts occur, as can be
    seen in the two panes that overlap, rotating the camera around that alternates which panel is
    drawn over the other.
*/

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Instanced Draw Order".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(1.4933096, 4.8070683, -9.277165),  // position
        vec3(0.14315122, 2.369473, -3.7785282), // target
        vec3(0.0, 1.0, 0.0),                    // up
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(vec3(0.0, 0.0, -0.5), 0.1, 10.0);

    // Shorthand for a 90 degree rotation about z.
    let rot_z90 = Mat4::from_angle_z(Deg(90.0));

    // Instances initially ordered such that drawing them in order without reordering is incorrect.
    let transparent_instances = three_d::renderer::geometry::Instances {
        transformations: vec![
            Mat4::from_translation(vec3(0.0, 0.0, -2.0)),
            Mat4::from_translation(vec3(0.0, 0.0, -1.0)),
            Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
            Mat4::from_translation(vec3(0.0, 0.0, 1.0)),
            // The next two cubes always intersect, even if ordered by depth, they will show
            // rendering artifacts from one view-direction.
            Mat4::from_translation(vec3(3.0, 0.0, 0.0)) * rot_z90 * Mat4::from_angle_x(Deg(45.0)),
            Mat4::from_translation(vec3(3.0, 0.0, 0.5)) * rot_z90 * Mat4::from_angle_x(Deg(-45.0)),
        ],
        colors: Some(vec![
            Color::new(0, 255, 0, 255),   // green, closest, should obscure everything.
            Color::new(255, 0, 255, 255), // purple, behind green, second opaque plane.
            Color::new(255, 0, 0, 128), // Red, third plane, should be behind two opaques, blend in front of blue.
            Color::new(0, 0, 255, 128), // Furthest, blue layer.
            // Next two always intersect.
            Color::new(0, 128, 128, 128), // Limitation of ordering, cyan
            Color::new(128, 128, 0, 128), // Limitation of ordering, yellow
        ]),
        ..Default::default()
    };

    let mut thin_cube = CpuMesh::cube();
    thin_cube
        .transform(&Mat4::from_nonuniform_scale(1.0, 1.0, 0.1))
        .unwrap();

    let transparent_meshes = Gm::new(
        InstancedMesh::new(&context, &transparent_instances, &thin_cube),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );

    // For opaque meshes, the draw order does not matter.
    let opaque_instances = three_d::renderer::geometry::Instances {
        transformations: transparent_instances.transformations,
        colors: Some(
            transparent_instances
                .colors
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| Color {
                    r: c.r,
                    g: c.g,
                    b: c.b,
                    a: 255,
                })
                .collect(),
        ),
        ..Default::default()
    };
    let mut thin_cube_right = CpuMesh::cube();
    thin_cube_right
        .transform(
            &(Mat4::from_translation(vec3(-3.0, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(1.0, 1.0, 0.1)),
        )
        .unwrap();

    let mut opaque_meshes_opaque_instances = Gm::new(
        InstancedMesh::new(&context, &opaque_instances, &thin_cube_right),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    // Testing that changing the instance count still works as expected, blue should disappear.
    opaque_meshes_opaque_instances.set_instance_count(3);

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let ambient_light = three_d::renderer::light::AmbientLight::new(&context, 0.1, Color::WHITE);

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                transparent_meshes
                    .into_iter()
                    .chain(&opaque_meshes_opaque_instances),
                &[&light0, &ambient_light],
            );

        FrameOutput::default()
    });
}
