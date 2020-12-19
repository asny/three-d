
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Imposters!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(180.0, 40.0, 70.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 10000.0);

    Loader::load(&["./examples/assets/tree.3d", "./examples/assets/tree_Leaves.png", "./examples/assets/tree_Material.001.png"], move |loaded| {
        let (meshes, materials)  = ThreeD::parse(loaded, "./examples/assets/tree.3d").unwrap();
        loaded.clear();

        // Tree
        let tree_mesh = renderer.new_meshes(&meshes, &materials).unwrap();

        // Imposters
        let mut aabb = AxisAlignedBoundingBox::new();
        meshes.iter().for_each(|m| aabb.add(&m.compute_aabb()));
        let mut imposter = Imposter::new(&gl, &|camera: &Camera| {
            for mesh in tree_mesh.iter() {
                mesh.render_geometry(&Mat4::identity(), camera)?;
            }
            Ok(())
        }, (aabb.min, aabb.max), 256).unwrap();

        let t = 100;
        let mut positions = Vec::new();
        let mut angles = Vec::new();
        for x in -t..t {
            for y in -t..t {
                if x != 0 || y != 0 {
                    positions.push(10.0 * x as f32);
                    positions.push(0.0);
                    positions.push(10.0 * y as f32);
                    angles.push((1.0 + y as f32 / t as f32) * std::f32::consts::PI);
                }
            }
        }
        imposter.update_positions(&positions, &angles);

        // Plane
        let plane = renderer.new_mesh(
            &CPUMesh {
                positions: vec!(-10000.0, -1.0, 10000.0, 10000.0, -1.0, 10000.0, 0.0, -1.0, -10000.0),
                normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                ..Default::default()},
            &Material {color_source: ColorSource::Color(vec4(0.5, 0.7, 0.3, 1.0)),
                diffuse_intensity: 1.0,
                specular_intensity: 0.0, ..Default::default()}
        ).unwrap();

        // Lights
        let ambient_light = AmbientLight::new(&gl, 0.2, &vec3(1.0, 1.0, 1.0)).unwrap();
        let mut directional_light = DirectionalLight::new(&gl, 0.9, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0)).unwrap();
        directional_light.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 1000.0, 1000.0, 500.0, 4096, 4096, &|camera: &Camera| {
            for mesh in tree_mesh.iter() {
                mesh.render_geometry(&Mat4::identity(), camera)?;
            }
            imposter.render(camera)?;
            Ok(())
        });

        // main loop
        let mut rotating = false;
        window.render_loop(move |frame_input|
        {
            camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick {state, button, ..} => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    },
                    Event::MouseMotion {delta} => {
                        if rotating {
                            camera.rotate(delta.0 as f32, delta.1 as f32);
                        }
                    },
                    Event::MouseWheel {delta} => {
                        camera.zoom(*delta as f32);
                    },
                    Event::Key { ref state, ref kind } => {
                        if kind == "R" && *state == State::Pressed
                        {
                            renderer.next_debug_type();
                            println!("{:?}", renderer.debug_type());
                        }
                    }
                }
            }

            // Geometry pass
            renderer.geometry_pass(width, height, &||
                {
                    state::cull(&gl, state::CullType::Back);
                    for mesh in tree_mesh.iter() {
                        if mesh.name() == "trunk" {
                            mesh.render_geometry(&Mat4::identity(), &camera)?;
                        }
                    }
                    imposter.render(&camera)?;
                    plane.render_geometry(&Mat4::identity(), &camera)?;
                    Ok(())
                }).unwrap();

            // Light pass
            Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.8, 0.8, 0.8, 1.0)), Some(1.0), &|| {
                renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[])?;

                state::cull(&gl, state::CullType::None);
                state::blend(&gl, state::BlendType::SrcAlphaOneMinusSrcAlpha);
                for mesh in tree_mesh.iter() {
                    if mesh.name() == "leaves" {
                        mesh.mesh().render_with_ambient_and_directional(&Mat4::identity(), &camera, &ambient_light, &directional_light)?;
                    }
                }
                Ok(())
            }).unwrap();

            #[cfg(target_arch = "x86_64")]
            if let Some(ref path) = screenshot_path {
                let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
                Saver::save_pixels(path, &pixels, width, height).unwrap();
                std::process::exit(1);
            }
        }).unwrap();
    });

}