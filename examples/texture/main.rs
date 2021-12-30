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

    let skybox = Loading::new(
        &context,
        &["examples/assets/flower_road_4k.hdr"],
        move |context, mut loaded| {
            Skybox::new_from_equirectangular(&context, &loaded.hdr_image("")?)
        },
    );

    let objects = Loading::new(
        &context,
        &[
            "examples/assets/test_texture.jpg",
            "examples/assets/PenguinBaseMesh.obj",
            "examples/assets/PenguinBaseMesh.mtl",
            "examples/assets/penguin.png",
        ],
        move |context, mut loaded| {
            let mut box_object = Model::new_with_material(
                &context,
                &CPUMesh::cube(),
                ColorMaterial {
                    texture: Some(std::rc::Rc::new(Texture2D::new(
                        &context,
                        &loaded.image("test_texture")?,
                    )?)),
                    ..Default::default()
                },
            )?;
            box_object.material.opaque_render_states.cull = Cull::Back;
            let (penguin_cpu_meshes, penguin_cpu_materials) = loaded.obj("PenguinBaseMesh.obj")?;
            let mut penguin_object = Model::new_with_material(
                &context,
                &penguin_cpu_meshes[0],
                PhysicalMaterial::new(&context, &penguin_cpu_materials[0])?,
            )?;
            penguin_object.set_transformation(Mat4::from_translation(vec3(0.0, 1.0, 0.5)));
            penguin_object.material.opaque_render_states.cull = Cull::Back;
            Ok((box_object, penguin_object))
        },
    );

    let lights = Lights {
        ambient: Some(AmbientLight {
            intensity: 0.4,
            color: Color::WHITE,
            ..Default::default()
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
    let mut loaded = false;
    window
        .render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();
            redraw |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();
            if !loaded && objects.is_loaded() && skybox.is_loaded() {
                redraw = true;
                loaded = true;
            }

            // draw
            if redraw {
                Screen::write(&context, ClearState::default(), || {
                    if let Some(Ok((ref box_object, ref penguin_object))) = *objects.borrow() {
                        pipeline.render_pass(
                            &camera,
                            &[box_object as &dyn Object, penguin_object],
                            &lights,
                        )?;
                    }
                    if let Some(Ok(ref skybox)) = *skybox.borrow() {
                        skybox.render(&camera)?;
                    }
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
}
