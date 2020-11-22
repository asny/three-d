
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Texture").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(4.0, 1.5, 4.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    Loader::load(&["examples/assets/textures/penguin.png", "examples/assets/models/penguin.3d", "examples/assets/textures/test_texture.jpg",
        "examples/assets/textures/skybox_evening/back.jpg", "examples/assets/textures/skybox_evening/front.jpg", "examples/assets/textures/skybox_evening/top.jpg",
        "examples/assets/textures/skybox_evening/left.jpg", "examples/assets/textures/skybox_evening/right.jpg"], move |loaded|
    {
        let mut box_mesh = CPUMesh {
            positions: cube_positions(),
            texture_path: Some("examples/assets/textures/test_texture.jpg".to_string()),
            ..Default::default() };
        box_mesh.compute_normals();
        let box_mesh = TexturedMesh::from_cpu_mesh(&gl, loaded, &box_mesh).unwrap();

        let skybox = objects::Skybox::new(&gl, loaded, "examples/assets/textures/skybox_evening/back.jpg", "examples/assets/textures/skybox_evening/front.jpg",
            "examples/assets/textures/skybox_evening/top.jpg", "examples/assets/textures/skybox_evening/left.jpg", "examples/assets/textures/skybox_evening/right.jpg").unwrap();

        let mut penguin_cpu_mesh = ThreeD::parse(Loader::get(loaded, "examples/assets/models/penguin.3d").unwrap()).unwrap().remove(0);
        penguin_cpu_mesh.texture_path = Some("examples/assets/textures/penguin.png".to_string());
        let penguin = TexturedMesh::from_cpu_mesh(&gl, loaded, &penguin_cpu_mesh).unwrap();

        let ambient_light = AmbientLight::new(&gl, 0.4, &vec3(1.0, 1.0, 1.0)).unwrap();
        let directional_light = DirectionalLight::new(&gl, 1.0, &vec3(1.0, 1.0, 1.0), &vec3(0.0, -1.0, -1.0)).unwrap();

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
                    Event::Key { state, kind } => {
                        if kind == "R" && *state == State::Pressed
                        {
                            renderer.next_debug_type();
                            println!("{:?}", renderer.debug_type());
                        }
                    }
                }
            }

            // draw
            // Geometry pass
            renderer.geometry_pass(width, height, &|| {
                let mut transformation = Mat4::identity();
                box_mesh.render(&transformation, &camera);
                transformation = Mat4::from_translation(vec3(0.0, 1.0, 0.0));
                penguin.render(&transformation, &camera);
            }).unwrap();

            Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.8, 0.0, 0.0, 1.0)), None, &|| {
                skybox.render(&camera).unwrap();
                renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[]).unwrap();
            }).unwrap();

            if let Some(ref path) = screenshot_path {
                #[cfg(target_arch = "x86_64")]
                Screen::save_color(path, &gl, 0, 0, width, height).unwrap();
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