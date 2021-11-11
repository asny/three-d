use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Texture!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let pipeline = ForwardPipeline::new(&context).unwrap();
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(4.0, 1.5, 4.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    Loader::load(
        &[
            "examples/assets/PenguinBaseMesh.obj",
            "examples/assets/PenguinBaseMesh.mtl",
            "examples/assets/penguin.png",
            "examples/assets/test_texture.jpg",
            "examples/assets/skybox_evening/back.jpg",
            "examples/assets/skybox_evening/front.jpg",
            "examples/assets/skybox_evening/top.jpg",
            "examples/assets/skybox_evening/left.jpg",
            "examples/assets/skybox_evening/right.jpg",
        ],
        move |mut loaded| {
            let mut box_object = Model::new_with_material(
                &context,
                &CPUMesh::cube(),
                ColorMaterial {
                    texture: Some(std::rc::Rc::new(
                        Texture2D::new(
                            &context,
                            &loaded.image("examples/assets/test_texture.jpg").unwrap(),
                        )
                        .unwrap(),
                    )),
                    ..Default::default()
                },
            )
            .unwrap();
            box_object.material.opaque_render_states.cull = Cull::Back;

            let skybox = Skybox::new(
                &context,
                &mut loaded
                    .cube_image(
                        "examples/assets/skybox_evening/right.jpg",
                        "examples/assets/skybox_evening/left.jpg",
                        "examples/assets/skybox_evening/top.jpg",
                        "examples/assets/skybox_evening/top.jpg",
                        "examples/assets/skybox_evening/front.jpg",
                        "examples/assets/skybox_evening/back.jpg",
                    )
                    .unwrap(),
            )
            .unwrap();

            let (penguin_cpu_meshes, penguin_cpu_materials) =
                loaded.obj("examples/assets/PenguinBaseMesh.obj").unwrap();
            let mut penguin_object = Model::new_with_material(
                &context,
                &penguin_cpu_meshes[0],
                PhysicalMaterial::new(&context, &penguin_cpu_materials[0]).unwrap(),
            )
            .unwrap();
            penguin_object.set_transformation(Mat4::from_translation(vec3(0.0, 1.0, 0.5)));
            penguin_object.material.opaque_render_states.cull = Cull::Back;

            let lights = Lights {
                ambient: Some(AmbientLight {
                    intensity: 0.4,
                    color: Color::WHITE,
                }),
                directional: vec![DirectionalLight::new(
                    &context,
                    2.0,
                    Color::WHITE,
                    &vec3(0.0, -1.0, -1.0),
                )
                .unwrap()],
                ..Default::default()
            };

            // main loop
            window
                .render_loop(move |mut frame_input| {
                    let mut redraw = frame_input.first_frame;
                    redraw |= camera.set_viewport(frame_input.viewport).unwrap();
                    redraw |= control
                        .handle_events(&mut camera, &mut frame_input.events)
                        .unwrap();

                    // draw
                    if redraw {
                        Screen::write(&context, ClearState::default(), || {
                            pipeline.render_pass(
                                &camera,
                                &[&box_object as &dyn Object, &penguin_object],
                                &lights,
                            )?;
                            skybox.render(&camera)?;
                            Ok(())
                        })
                        .unwrap();
                    }

                    if args.len() > 1 {
                        // To automatically generate screenshots of the examples, can safely be ignored.
                        FrameOutput {
                            screenshot: Some(args[1].clone().into()),
                            exit: true,
                            ..Default::default()
                        }
                    } else {
                        FrameOutput {
                            swap_buffers: redraw,
                            wait_next_event: true,
                            ..Default::default()
                        }
                    }
                })
                .unwrap();
        },
    );
}
