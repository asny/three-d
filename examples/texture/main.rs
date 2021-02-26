
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let window = Window::new("Texture", None).unwrap();
    let context = window.gl();

    // Renderer
    let mut pipeline = PhongDeferredPipeline::new(&context).unwrap();
    let mut camera = Camera::new_perspective(&context, vec3(4.0, 1.5, 4.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), window.viewport().aspect(), 0.1, 1000.0);

    Loader::load(&["examples/assets/PenguinBaseMesh.obj", "examples/assets/PenguinBaseMesh.mtl",
        "examples/assets/penguin.png", "examples/assets/test_texture.jpg",
        "examples/assets/skybox_evening/back.jpg", "examples/assets/skybox_evening/front.jpg",
        "examples/assets/skybox_evening/top.jpg", "examples/assets/skybox_evening/left.jpg",
        "examples/assets/skybox_evening/right.jpg"], move |loaded|
    {
        let mut box_cpu_mesh = CPUMesh {
            positions: cube_positions(),
            uvs: Some(cube_uvs()),
            ..Default::default()
        };
        box_cpu_mesh.compute_normals();
        let box_texture = Texture2D::new_with_u8(&context, &Loader::get_texture(loaded, "examples/assets/test_texture.jpg").unwrap()).unwrap();
        let box_material = PhongMaterial {
            color_source: ColorSource::Texture(std::rc::Rc::new(box_texture)),
            ..Default::default()
        };
        let box_mesh = PhongDeferredMesh::new(&context, &box_cpu_mesh, &box_material).unwrap();

        let skybox = Skybox::new(&context, &mut Loader::get_cube_texture(loaded, "examples/assets/skybox_evening/right.jpg",
                                                                         "examples/assets/skybox_evening/left.jpg",
                                                                         "examples/assets/skybox_evening/top.jpg",
                                                                         "examples/assets/skybox_evening/top.jpg",
                                                                         "examples/assets/skybox_evening/front.jpg",
                                                                         "examples/assets/skybox_evening/back.jpg").unwrap()).unwrap();

        let (penguin_cpu_meshes, penguin_cpu_materials) = Obj::parse(loaded, "examples/assets/PenguinBaseMesh.obj").unwrap();
        let materials = penguin_cpu_materials.iter().map(|m| PhongMaterial::new(&context, m).unwrap()).collect::<Vec<PhongMaterial>>();
        let penguin_deferred = PhongDeferredMesh::new_meshes(&context, &penguin_cpu_meshes, &materials).unwrap().remove(0);
        let penguin_forward = PhongForwardMesh::new_meshes(&context, &penguin_cpu_meshes, &materials).unwrap().remove(0);

        let ambient_light = AmbientLight {intensity: 0.4, color: vec3(1.0, 1.0, 1.0)};
        let directional_light = DirectionalLight::new(&context, 1.0, &vec3(1.0, 1.0, 1.0), &vec3(0.0, -1.0, -1.0)).unwrap();

        // main loop
        let mut rotating = false;
        window.render_loop(move |frame_input|
        {
            camera.set_aspect(frame_input.viewport.aspect());

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick {state, button, ..} => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    },
                    Event::MouseMotion {delta, ..} => {
                        if rotating {
                            camera.rotate_around_up(delta.0 as f32, delta.1 as f32);
                        }
                    },
                    Event::MouseWheel {delta, ..} => {
                        camera.zoom(*delta as f32);
                    },
                    Event::Key { state, kind } => {
                        if kind == "R" && *state == State::Pressed
                        {
                            pipeline.next_debug_type();
                            println!("{:?}", pipeline.debug_type());
                        }
                    }
                }
            }

            // draw
            // Geometry pass
            pipeline.geometry_pass(frame_input.viewport.width, frame_input.viewport.height, &|| {
                let mut transformation = Mat4::identity();
                box_mesh.render_geometry(RenderStates {cull: CullType::Back, ..Default::default()},
                                         frame_input.viewport, &transformation, &camera)?;
                transformation = Mat4::from_translation(vec3(-0.5, 1.0, 0.0));
                penguin_deferred.render_geometry(RenderStates {cull: CullType::Back, ..Default::default()},
                                                 frame_input.viewport, &transformation, &camera)?;
                Ok(())
            }).unwrap();

            Screen::write(&context, &ClearState::default(), ||
            {
                pipeline.light_pass(frame_input.viewport, &camera, Some(&ambient_light), &[&directional_light], &[], &[])?;
                let transformation = Mat4::from_translation(vec3(0.5, 1.0, 0.0));
                penguin_forward.render_with_ambient_and_directional(RenderStates {cull: CullType::Back, ..Default::default()},
                                                                    frame_input.viewport, &transformation, &camera, &ambient_light, &directional_light)?;
                skybox.render(frame_input.viewport, &camera)?;
                Ok(())
            }).unwrap();

            #[cfg(target_arch = "x86_64")]
            if let Some(ref path) = screenshot_path {
                let pixels = Screen::read_color(&context, frame_input.viewport).unwrap();
                Saver::save_pixels(path, &pixels, frame_input.viewport.width, frame_input.viewport.height).unwrap();
                std::process::exit(1);
            }
        }).unwrap();
    });
}

fn cube_positions() -> Vec<f32> {
    vec![
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,

        -1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,

        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, -1.0,

        -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, -1.0
    ]
}

fn cube_uvs() -> Vec<f32> {
    vec![
        1.0, 0.0,
        0.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        1.0, 1.0,
        0.0, 0.0,

        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,

        1.0, 0.0,
        0.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        1.0, 1.0,
        0.0, 0.0,

        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,

        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,

        1.0, 0.0,
        0.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        1.0, 1.0,
        0.0, 0.0
    ]
}