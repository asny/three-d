
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

    let mut loader = Loader::new();
    loader.start_loading("./examples/assets/models/leaves1.3d");
    loader.start_loading("./examples/assets/models/tree1.3d");
    loader.wait_all(move |loaded| {
        let leaves_cpu_mesh = ThreeD::parse(loaded.get("./examples/assets/models/leaves1.3d").unwrap()).unwrap();
        let tree_cpu_mesh = ThreeD::parse(loaded.get("./examples/assets/models/tree1.3d").unwrap()).unwrap();
        loaded.clear();

        // Tree
        let mut leaves_mesh = Mesh::from_cpu_mesh(&gl, &leaves_cpu_mesh).unwrap();
        let mut tree_mesh = Mesh::from_cpu_mesh(&gl, &tree_cpu_mesh).unwrap();
        tree_mesh.color = vec3(0.5, 0.2, 0.2);
        tree_mesh.specular_intensity = 0.0;
        tree_mesh.diffuse_intensity = 1.0;
        leaves_mesh.color = vec3(0.7, 0.9, 0.5);
        leaves_mesh.specular_intensity = 0.0;
        leaves_mesh.diffuse_intensity = 1.0;

        // Imposters
        let mut aabb = AxisAlignedBoundingBox::new(&leaves_cpu_mesh.positions);
        aabb.expand(&tree_cpu_mesh.positions);
        let mut imposter = Imposter::new(&gl, &|camera: &Camera| {
                state::cull(&gl, state::CullType::Back);
                tree_mesh.render(&Mat4::identity(), camera);
                leaves_mesh.render(&Mat4::identity(), camera);
            }, (aabb.min, aabb.max), 256);

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
        let mut plane_mesh = tri_mesh::MeshBuilder::new().plane().build().unwrap();
        plane_mesh.scale(1000.0);
        let mut plane = Mesh::new(&gl, &plane_mesh.indices_buffer(), &plane_mesh.positions_buffer_f32(), &plane_mesh.normals_buffer_f32()).unwrap();
        plane.color = vec3(0.5, 0.7, 0.3);
        plane.diffuse_intensity = 1.0;
        plane.specular_intensity = 0.0;

        // Lights
        let ambient_light = AmbientLight::new(&gl, 0.2, &vec3(1.0, 1.0, 1.0)).unwrap();
        let mut directional_light0 = DirectionalLight::new(&gl, 0.9, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0)).unwrap();
        let directional_light1 = DirectionalLight::new(&gl, 0.4, &vec3(1.0, 1.0, 1.0), &vec3(1.0, 1.0, 1.0)).unwrap();
        directional_light0.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 1000.0, 1000.0, 500.0, 4096, 4096, &|camera: &Camera| {
            state::cull(&gl, state::CullType::Back);
            tree_mesh.render(&Mat4::identity(), camera);
            leaves_mesh.render(&Mat4::identity(), camera);
            imposter.render(camera);
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
                    tree_mesh.render(&Mat4::identity(), &camera);
                    leaves_mesh.render(&Mat4::identity(), &camera);
                    imposter.render(&camera);
                    plane.render(&Mat4::identity(), &camera);
                }).unwrap();

            // Light pass
            Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.8, 0.8, 0.8, 1.0)), None, &|| {
                renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light0, &directional_light1], &[], &[]).unwrap();
            }).unwrap();

            if let Some(ref path) = screenshot_path {
                #[cfg(target_arch = "x86_64")]
                Screen::save_color(path, &gl, 0, 0, width, height).unwrap();
                std::process::exit(1);
            }

        }).unwrap();
    });

}