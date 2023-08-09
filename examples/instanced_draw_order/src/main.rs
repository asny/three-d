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
            Srgba::new(255, 255, 255, 128),
            Srgba::new(255, 0, 255, 128),
            Srgba::new(255, 0, 0, 128),
            Srgba::new(0, 0, 255, 128),
            // Next two always intersect.
            Srgba::new(255, 255, 255, 128),
            Srgba::new(0, 0, 255, 128),
        ]),
        ..Default::default()
    };

    let mut thin_cube = CpuMesh::cube();
    thin_cube
        .transform(&Mat4::from_nonuniform_scale(1.0, 1.0, 0.04))
        .unwrap();

    let transparent_models = Gm::new(
        InstancedMesh::new(&context, &transparent_instances, &thin_cube),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba::WHITE,
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
                .map(|c| Srgba {
                    r: c.r,
                    g: c.g,
                    b: c.b,
                    a: 255,
                })
                .collect(),
        ),
        ..Default::default()
    };

    let mut opaque_models = Gm::new(
        InstancedMesh::new(&context, &opaque_instances, &thin_cube),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::WHITE,
                ..Default::default()
            },
        ),
    );
    opaque_models.set_transformation(Mat4::from_translation(vec3(-6.0, 0.0, 0.0)));

    let mut opaque_model = Gm::new(
        Mesh::new(&context, &thin_cube),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(128, 128, 128, 255),
                ..Default::default()
            },
        ),
    );
    opaque_model.set_transformation(Mat4::from_translation(vec3(0.0, -0.4, -3.0)));

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let ambient_light = three_d::renderer::light::AmbientLight::new(&context, 0.8, Srgba::WHITE);

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                transparent_models
                    .into_iter()
                    .chain(&opaque_models)
                    .chain(&opaque_model),
                &[&light0, &ambient_light],
            );

        FrameOutput::default()
    });
}
