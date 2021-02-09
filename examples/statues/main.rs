
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new("Statues", None).unwrap();
    let context = window.gl();

    // Renderer
    let mut pipeline = PhongDeferredPipeline::new(&context).unwrap();
    let mut primary_camera = Camera::new_perspective(&context, vec3(-200.0, 200.0, 100.0), vec3(0.0, 100.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                     degrees(45.0), window.viewport().aspect(), 0.1, 10000.0);
    // Static camera to view frustum culling in effect
    let mut secondary_camera = Camera::new_perspective(&context, vec3(-500.0, 700.0, 500.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                       degrees(45.0), window.viewport().aspect(), 0.1, 10000.0);

    // Models from http://texturedmesh.isti.cnr.it/
    Loader::load(&["examples/assets/COLOMBE.obj", "examples/assets/COLOMBE.mtl",
        "examples/assets/COLOMBE.png","examples/assets/pfboy.obj", "examples/assets/pfboy.mtl",
        "examples/assets/pfboy.png"], move |loaded|
    {
        let (statue_cpu_meshes, statue_cpu_materials) = Obj::parse(loaded, "examples/assets/COLOMBE.obj").unwrap();
        let statue_material = PhongMaterial::new(&context, &statue_cpu_materials[0]).unwrap();
        let statue = PhongForwardMesh::new(&context, &statue_cpu_meshes[0], &statue_material).unwrap();
        let scale = Mat4::from_scale(10.0);
        let mut statue_transforms_and_aabb = Vec::new();
        for i in 0..8 {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / 8.0;
            let rotation = Mat4::from_angle_y(radians(0.8 * std::f32::consts::PI - angle));
            let dist = 300.0;
            let translation = Mat4::from_translation(vec3(angle.cos() * dist, (1.2*std::f32::consts::PI - angle).cos() * 21.0 - 33.0, angle.sin() * dist));
            let transform = translation * scale * rotation;
            let aabb = AxisAlignedBoundingBox::new().expand_with_transformation(&statue_cpu_meshes[0].positions, &transform);
            statue_transforms_and_aabb.push((transform, aabb));
        }

        let (fountain_cpu_meshes, fountain_cpu_materials) = Obj::parse(loaded, "examples/assets/pfboy.obj").unwrap();
        let materials = fountain_cpu_materials.iter().map(|m| PhongMaterial::new(&context, m).unwrap()).collect::<Vec<PhongMaterial>>();
        let fountain = PhongForwardMesh::new_meshes(&context, &fountain_cpu_meshes, &materials).unwrap().remove(0);

        let ambient_light = AmbientLight {intensity: 0.4, color: vec3(1.0, 1.0, 1.0)};
        let mut directional_light = DirectionalLight::new(&context, 1.0, &vec3(0.8, 0.7, 0.5), &vec3(0.0, -1.0, -1.0)).unwrap();

        directional_light.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 1000.0, 1000.0, 2000.0, 1024, 1024, &|viewport: Viewport, camera: &Camera| {
            for (transform, _aabb) in statue_transforms_and_aabb.iter() {
                statue.render_depth(RenderStates {cull: CullType::Back, ..Default::default()},
                                    viewport,
                                    transform,
                                    &camera)?;
            }

            fountain.render_depth(RenderStates {cull: CullType::Back, ..Default::default()},
                                                         viewport,
                                                         &Mat4::from_angle_x(degrees(-90.0)),
                                                         &camera)?;
            Ok(())
        });

        // main loop
        let mut rotating = false;
        let mut is_primary_camera = true;
        window.render_loop(move |frame_input|
        {
            primary_camera.set_aspect(frame_input.viewport.aspect());
            secondary_camera.set_aspect(frame_input.viewport.aspect());

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick {state, button, ..} => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    },
                    Event::MouseMotion {delta, ..} => {
                        if rotating {
                            primary_camera.rotate_around_up(10.0 * delta.0 as f32, 10.0 * delta.1 as f32);
                        }
                    },
                    Event::Key { state, kind } => {
                        if kind == "R" && *state == State::Pressed
                        {
                            pipeline.next_debug_type();
                            println!("{:?}", pipeline.debug_type());
                        }
                        if kind == "C" && *state == State::Pressed
                        {
                            is_primary_camera = !is_primary_camera;
                        }
                    },
                    _ => {}
                }
            }

            // draw
            Screen::write(&context, Some(&vec4(0.8, 0.8, 0.7, 1.0)), Some(1.0), ||
            {
                for (transform, aabb) in statue_transforms_and_aabb.iter() {
                    if primary_camera.in_frustum(aabb) {
                        statue.render_with_ambient_and_directional(RenderStates { cull: CullType::Back, ..Default::default() },
                                                                   frame_input.viewport,
                                                                   &transform,
                                                                   if is_primary_camera { &primary_camera } else { &secondary_camera },
                                                                   &ambient_light,
                                                                   &directional_light)?;
                    }
                }

                fountain.render_with_ambient_and_directional(RenderStates {cull: CullType::Back, ..Default::default()},
                                                             frame_input.viewport,
                                                             &Mat4::from_angle_x(degrees(-90.0)),
                                                             if is_primary_camera { &primary_camera } else { &secondary_camera },
                                                             &ambient_light,
                                                             &directional_light)?;
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
