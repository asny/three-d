use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "PBR!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut pipeline = ForwardPipeline::new(&context).unwrap();
    pipeline.lighting_model = LightingModel::Cook(
        NormalDistributionFunction::Beckmann,
        GeometryFunction::SmithSchlickGGX,
    );

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(3.0, 1.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut gui = three_d::GUI::new(&context).unwrap();

    Loader::load(
        &["examples/assets/gltf/DamagedHelmet.glb"],
        move |mut loaded| {
            let (cpu_meshes, cpu_materials) = loaded
                .gltf("examples/assets/gltf/DamagedHelmet.glb")
                .unwrap();
            let material = Material::new(&context, &cpu_materials[0]).unwrap();
            let mut model = Model::new(&context, &cpu_meshes[0]).unwrap();
            model.cull = Cull::Back;
            model.set_transformation(Mat4::from_angle_x(degrees(90.0)));

            let ambient_light = AmbientLight {
                color: Color::WHITE,
                intensity: 0.4,
            };
            let mut directional_light0 =
                DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap();
            let mut directional_light1 =
                DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap();
            let mut spot_light = SpotLight::new(
                &context,
                2.0,
                Color::WHITE,
                &vec3(0.0, 0.0, 0.0),
                &vec3(0.0, -1.0, 0.0),
                20.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();

            // main loop
            let mut normal_map_enabled = true;
            let mut occlusion_map_enabled = true;
            let mut metallic_roughness_enabled = true;
            let mut albedo_map_enabled = true;
            window
                .render_loop(move |mut frame_input| {
                    let mut panel_width = 0;
                    gui.update(&mut frame_input, |gui_context| {
                        use three_d::egui::*;
                        SidePanel::left("side_panel").show(gui_context, |ui| {
                            ui.heading("Debug Panel");
                            ui.checkbox(&mut albedo_map_enabled, "Albedo map");
                            ui.checkbox(&mut metallic_roughness_enabled, "Metallic roughness map");
                            ui.checkbox(&mut normal_map_enabled, "Normal map");
                            ui.checkbox(&mut occlusion_map_enabled, "Occlusion map");
                        });
                        panel_width = gui_context.used_size().x as u32;
                    })
                    .unwrap();
                    let material = Material {
                        name: material.name.clone(),
                        albedo: material.albedo,
                        albedo_texture: if albedo_map_enabled {
                            material.albedo_texture.clone()
                        } else {
                            None
                        },
                        metallic: material.metallic,
                        roughness: material.roughness,
                        metallic_roughness_texture: if metallic_roughness_enabled {
                            material.metallic_roughness_texture.clone()
                        } else {
                            None
                        },
                        normal_scale: material.normal_scale,
                        normal_texture: if normal_map_enabled {
                            material.normal_texture.clone()
                        } else {
                            None
                        },
                        occlusion_strength: material.occlusion_strength,
                        occlusion_texture: if occlusion_map_enabled {
                            material.occlusion_texture.clone()
                        } else {
                            None
                        },
                    };

                    let viewport = Viewport {
                        x: panel_width as i32,
                        y: 0,
                        width: frame_input.viewport.width - panel_width,
                        height: frame_input.viewport.height,
                    };
                    camera.set_viewport(viewport).unwrap();
                    control
                        .handle_events(&mut camera, &mut frame_input.events)
                        .unwrap();

                    let time = 0.001 * frame_input.accumulated_time;
                    let c = time.cos() as f32;
                    let s = time.sin() as f32;
                    directional_light0.set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
                    directional_light1.set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));
                    spot_light.set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
                    spot_light.set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));

                    // Draw
                    directional_light0
                        .generate_shadow_map(&vec3(0.0, 0.0, 0.0), 2.0, 20.0, 1024, 1024, &[&model])
                        .unwrap();
                    directional_light1
                        .generate_shadow_map(&vec3(0.0, 0.0, 0.0), 2.0, 20.0, 1024, 1024, &[&model])
                        .unwrap();
                    spot_light
                        .generate_shadow_map(15.0, 1024, &[&model])
                        .unwrap();
                    Screen::write(
                        &context,
                        ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                        || {
                            pipeline.light_pass(
                                &camera,
                                &[(&model, &material)],
                                Some(&ambient_light),
                                &[&directional_light0, &directional_light1],
                                &[&spot_light],
                                &[],
                            )?;
                            gui.render()?;
                            Ok(())
                        },
                    )
                    .unwrap();

                    if args.len() > 1 {
                        // To automatically generate screenshots of the examples, can safely be ignored.
                        FrameOutput {
                            screenshot: Some(args[1].clone().into()),
                            exit: true,
                            ..Default::default()
                        }
                    } else {
                        FrameOutput::default()
                    }
                })
                .unwrap();
        },
    );
}
