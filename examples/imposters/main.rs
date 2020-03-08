
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Imposters!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(180.0, 40.0, 70.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let mut leaves_mesh = CPUMesh::from_bytes(include_bytes!("../assets/models/leaves1.3d")).unwrap().to_mesh(&gl).unwrap();
    let mut tree_mesh = CPUMesh::from_bytes(include_bytes!("../assets/models/tree1.3d")).unwrap().to_mesh(&gl).unwrap();
    tree_mesh.color = vec3(0.5, 0.2, 0.2);
    tree_mesh.specular_intensity = 0.0;
    leaves_mesh.color = vec3(0.7, 0.9, 0.5);
    leaves_mesh.specular_intensity = 0.0;
    let aabb = tree_mesh.axis_aligned_bounding_box().add(leaves_mesh.axis_aligned_bounding_box());
    let mut imposter = Imposter::new(&gl, &|camera: &Camera| {
            tree_mesh.render(&Mat4::identity(), camera);
            leaves_mesh.render(&Mat4::identity(), camera);
        }, (aabb.min, aabb.max), 256);
    let t = 10;
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

    let mut plane_mesh = tri_mesh::MeshBuilder::new().plane().build().unwrap();
    plane_mesh.scale(100.0);
    let mut plane = Mesh::new(&gl, &plane_mesh.indices_buffer(), &plane_mesh.positions_buffer_f32(), &plane_mesh.normals_buffer_f32()).unwrap();
    plane.color = vec3(0.5, 0.7, 0.3);
    plane.diffuse_intensity = 0.5;
    plane.specular_intensity = 0.0;

    let ambient_light = AmbientLight::new(&gl, 0.2, &vec3(1.0, 1.0, 1.0)).unwrap();
    let mut directional_light = DirectionalLight::new(&gl, 0.5, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0)).unwrap();

    let render_scene = |camera: &Camera| {
        tree_mesh.render(&Mat4::identity(), camera);
        leaves_mesh.render(&Mat4::identity(), camera);
        imposter.render(camera);
    };
    directional_light.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 300.0, 300.0, 300.0, 1024, 1024, &render_scene);

    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

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
                        debug_effect.change_type();
                    }
                }
            }
        }

        // Geometry pass
        renderer.geometry_pass(&||
            {
                tree_mesh.render(&Mat4::identity(), &camera);
                leaves_mesh.render(&Mat4::identity(), &camera);
                imposter.render(&camera);
                plane.render(&Mat4::identity(), &camera);
            }).unwrap();

        // Light pass
        RenderTarget::write_to_screen(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 0.0)), None, &|| {
            renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[]).unwrap();
        }).unwrap();

        debug_effect.apply(&camera, renderer.geometry_pass_texture(), renderer.geometry_pass_depth_texture()).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, width, height).unwrap();
            std::process::exit(1);
        }

    }).unwrap();
}