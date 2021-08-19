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

    // Renderer
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
            let box_texture = Texture2D::new(
                &context,
                &loaded.image("examples/assets/test_texture.jpg").unwrap(),
            )
            .unwrap();
            let box_material = Material {
                albedo_texture: Some(std::rc::Rc::new(box_texture)),
                ..Default::default()
            };
            let mut box_mesh = Model::new(&context, &CPUMesh::cube()).unwrap();
            box_mesh.cull = Cull::Back;

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
            let penguin_material = Material::new(&context, &penguin_cpu_materials[0]).unwrap();
            let mut penguin = Model::new(&context, &penguin_cpu_meshes[0]).unwrap();
            penguin.set_transformation(Mat4::from_translation(vec3(0.0, 1.0, 0.5)));
            penguin.cull = Cull::Back;

            let ambient_light = AmbientLight {
                intensity: 0.4,
                color: Color::WHITE,
            };
            let directional_light =
                DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, -1.0)).unwrap();

            let axes = Axes::new(&context, 0.1, 3.0).unwrap();
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
                            pipeline.light_pass(
                                &camera,
                                &[(&box_mesh, &box_material), (&penguin, &penguin_material)],
                                Some(&ambient_light),
                                &[&directional_light],
                                &[],
                                &[],
                            )?;
                            axes.render(&camera)?;
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
