use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Lighting!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut pipeline = DeferredPipeline::new(&context).unwrap();
    let target = vec3(0.0, 0.0, 0.0);
    let mut camera = CameraControl::new(
        Camera::new_perspective(
            &context,
            vec3(2.0, 2.0, 5.0),
            target,
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            window.viewport().unwrap().aspect(),
            0.1,
            1000.0,
        )
        .unwrap(),
    );
    let mut gui = three_d::GUI::new(&context).unwrap();

    Loader::load(
        &["examples/assets/suzanne.obj", "examples/assets/suzanne.mtl"],
        move |loaded| {
            let (monkey_cpu_meshes, monkey_cpu_materials) =
                loaded.obj("examples/assets/suzanne.obj").unwrap();
            let mut monkey = Mesh::new_with_material(
                &context,
                &monkey_cpu_meshes[0],
                &Material::new(&context, &monkey_cpu_materials[0]).unwrap(),
            )
            .unwrap();
            monkey.cull = CullType::Back;

            let mut plane = Mesh::new_with_material(
                &context,
                &CPUMesh {
                    positions: vec![
                        -10000.0, -1.0, 10000.0, 10000.0, -1.0, 10000.0, 0.0, -1.0, -10000.0,
                    ],
                    normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                    ..Default::default()
                },
                &Material {
                    color_source: ColorSource::Color(vec4(0.5, 0.7, 0.3, 1.0)),
                    ..Default::default()
                },
            )
            .unwrap();

            let ambient_light = AmbientLight {
                color: vec3(1.0, 1.0, 1.0),
                intensity: 0.2,
            };
            let mut directional_light0 =
                DirectionalLight::new(&context, 1.0, &vec3(1.0, 0.0, 0.0), &vec3(0.0, -1.0, 0.0))
                    .unwrap();
            let mut directional_light1 =
                DirectionalLight::new(&context, 1.0, &vec3(0.0, 1.0, 0.0), &vec3(0.0, -1.0, 0.0))
                    .unwrap();
            let mut point_light0 = PointLight::new(
                &context,
                1.0,
                &vec3(0.0, 1.0, 0.0),
                &vec3(0.0, 0.0, 0.0),
                0.5,
                0.05,
                0.005,
            )
            .unwrap();
            let mut point_light1 = PointLight::new(
                &context,
                1.0,
                &vec3(1.0, 0.0, 0.0),
                &vec3(0.0, 0.0, 0.0),
                0.5,
                0.05,
                0.005,
            )
            .unwrap();
            let mut spot_light = SpotLight::new(
                &context,
                2.0,
                &vec3(0.0, 0.0, 1.0),
                &vec3(0.0, 0.0, 0.0),
                &vec3(0.0, -1.0, 0.0),
                25.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();

            // main loop
            let mut rotating = false;
            let mut shadows_enabled = true;

            let mut ambient_enabled = true;
            let mut directional_enabled = true;
            let mut spot_enabled = true;
            let mut point_enabled = true;

            window
                .render_loop(move |mut frame_input| {
                    let mut change = frame_input.first_frame;
                    let mut panel_width = frame_input.viewport.width;
                    change |= gui
                        .update(&mut frame_input, |gui_context| {
                            use three_d::egui::*;
                            SidePanel::left("side_panel", panel_width as f32).show(
                                gui_context,
                                |ui| {
                                    ui.heading("Debug Panel");

                                    ui.label("Surface parameters");
                                    ui.add(
                                        Slider::f32(&mut monkey.material.metallic, 0.0..=1.0)
                                            .text("Monkey Metallic"),
                                    );
                                    ui.add(
                                        Slider::f32(&mut monkey.material.roughness, 0.0..=1.0)
                                            .text("Monkey Roughness"),
                                    );
                                    ui.add(
                                        Slider::f32(&mut plane.material.metallic, 0.0..=1.0)
                                            .text("Plane Metallic"),
                                    );
                                    ui.add(
                                        Slider::f32(&mut plane.material.roughness, 0.0..=1.0)
                                            .text("Plane Roughness"),
                                    );

                                    ui.label("Lighting model");
                                    ui.radio_value(
                                        &mut pipeline.lighting_model,
                                        LightingModel::Phong,
                                        "Phong",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.lighting_model,
                                        LightingModel::Blinn,
                                        "Blinn",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.lighting_model,
                                        LightingModel::Cook(
                                            NormalDistributionFunction::Blinn,
                                            GeometryFunction::SmithSchlickGGX,
                                        ),
                                        "Cook (Blinn)",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.lighting_model,
                                        LightingModel::Cook(
                                            NormalDistributionFunction::Beckmann,
                                            GeometryFunction::SmithSchlickGGX,
                                        ),
                                        "Cook (Beckmann)",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.lighting_model,
                                        LightingModel::Cook(
                                            NormalDistributionFunction::TrowbridgeReitzGGX,
                                            GeometryFunction::SmithSchlickGGX,
                                        ),
                                        "Cook (Trowbridge-Reitz GGX)",
                                    );

                                    ui.label("Debug options");
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::NONE,
                                        "None",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::POSITION,
                                        "Position",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::NORMAL,
                                        "Normal",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::COLOR,
                                        "Color",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::DEPTH,
                                        "Depth",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::DIFFUSE,
                                        "Diffuse",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::SPECULAR,
                                        "Specular",
                                    );
                                    ui.radio_value(
                                        &mut pipeline.debug_type,
                                        DebugType::POWER,
                                        "Power",
                                    );

                                    ui.label("Light options");
                                    ui.checkbox(&mut ambient_enabled, "Ambient light");
                                    ui.checkbox(&mut directional_enabled, "Directional lights");
                                    ui.checkbox(&mut spot_enabled, "Spot lights");
                                    ui.checkbox(&mut point_enabled, "Point lights");
                                    if ui.checkbox(&mut shadows_enabled, "Shadows").clicked() {
                                        if !shadows_enabled {
                                            spot_light.clear_shadow_map();
                                            directional_light0.clear_shadow_map();
                                            directional_light1.clear_shadow_map();
                                        }
                                    }
                                },
                            );
                            panel_width =
                                (gui_context.used_size().x * gui_context.pixels_per_point()) as u32;
                        })
                        .unwrap();

                    let viewport = Viewport {
                        x: panel_width as i32,
                        y: 0,
                        width: frame_input.viewport.width - panel_width,
                        height: frame_input.viewport.height,
                    };
                    change |= camera.set_aspect(viewport.aspect()).unwrap();

                    for event in frame_input.events.iter() {
                        match event {
                            Event::MouseClick {
                                state,
                                button,
                                handled,
                                ..
                            } => {
                                if !handled {
                                    rotating =
                                        *button == MouseButton::Left && *state == State::Pressed;
                                    change = true;
                                }
                            }
                            Event::MouseMotion { delta, handled, .. } => {
                                if !handled && rotating {
                                    camera
                                        .rotate_around_with_fixed_up(
                                            &target,
                                            0.1 * delta.0 as f32,
                                            0.1 * delta.1 as f32,
                                        )
                                        .unwrap();
                                    change = true;
                                }
                            }
                            Event::MouseWheel { delta, handled, .. } => {
                                if !handled {
                                    camera
                                        .zoom_towards(&target, 0.02 * delta.1 as f32, 5.0, 100.0)
                                        .unwrap();
                                    change = true;
                                }
                            }
                            _ => {}
                        }
                    }
                    let time = 0.001 * frame_input.accumulated_time;
                    let c = time.cos() as f32;
                    let s = time.sin() as f32;
                    directional_light0.set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
                    directional_light1.set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));
                    spot_light.set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
                    spot_light.set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));
                    point_light0.set_position(&vec3(-5.0 * c, 5.0, -5.0 * s));
                    point_light1.set_position(&vec3(5.0 * c, 5.0, 5.0 * s));

                    // Draw
                    if shadows_enabled {
                        directional_light0
                            .generate_shadow_map(
                                &vec3(0.0, 0.0, 0.0),
                                4.0,
                                4.0,
                                20.0,
                                1024,
                                1024,
                                &[&monkey],
                            )
                            .unwrap();
                        directional_light1
                            .generate_shadow_map(
                                &vec3(0.0, 0.0, 0.0),
                                4.0,
                                4.0,
                                20.0,
                                1024,
                                1024,
                                &[&monkey],
                            )
                            .unwrap();
                        spot_light
                            .generate_shadow_map(20.0, 1024, &[&monkey])
                            .unwrap();
                    }

                    // Geometry pass
                    if change {
                        pipeline
                            .geometry_pass(
                                viewport.width,
                                viewport.height,
                                &camera,
                                &[&monkey, &plane],
                            )
                            .unwrap();
                    }

                    // Light pass
                    Screen::write(&context, ClearState::default(), || {
                        pipeline.light_pass(
                            viewport,
                            &camera,
                            if ambient_enabled {
                                Some(&ambient_light)
                            } else {
                                None
                            },
                            &if directional_enabled {
                                vec![&directional_light0, &directional_light1]
                            } else {
                                vec![]
                            },
                            &if spot_enabled {
                                vec![&spot_light]
                            } else {
                                vec![]
                            },
                            &if point_enabled {
                                vec![&point_light0, &point_light1]
                            } else {
                                vec![]
                            },
                        )?;
                        gui.render().unwrap();
                        Ok(())
                    })
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
