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

    let model = Loading::new(
        &context,
        &["examples/assets/gltf/DamagedHelmet.glb"],
        move |context, mut loaded| {
            let (mut cpu_meshes, cpu_materials) = loaded.gltf("DamagedHelmet.glb").unwrap();
            let mut material = PhysicalMaterial::new(&context, &cpu_materials[0]).unwrap();
            material.opaque_render_states.cull = Cull::Back;
            cpu_meshes[0].compute_tangents().unwrap();
            let mut model =
                Model::new_with_material(&context, &cpu_meshes[0], material.clone()).unwrap();
            model.set_transformation(Mat4::from_angle_x(degrees(90.0)));
            Ok(model)
        },
    );

    let mut lights = Lights {
        ambient: Some(AmbientLight {
            color: Color::WHITE,
            intensity: 0.4,
        }),
        directional: vec![
            DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap(),
            DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap(),
        ],
        spot: vec![SpotLight::new(
            &context,
            2.0,
            Color::WHITE,
            &vec3(0.0, 0.0, 0.0),
            &vec3(0.0, -1.0, 0.0),
            degrees(20.0),
            0.1,
            0.001,
            0.0001,
        )
        .unwrap()],
        lighting_model: LightingModel::Cook(
            NormalDistributionFunction::TrowbridgeReitzGGX,
            GeometryFunction::SmithSchlickGGX,
        ),
        ..Default::default()
    };

    // main loop
    let mut normal_map_enabled = true;
    let mut occlusion_map_enabled = true;
    let mut metallic_roughness_enabled = true;
    let mut albedo_map_enabled = true;
    let mut emissive_map_enabled = true;
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
                    ui.checkbox(&mut emissive_map_enabled, "Emissive map");
                });
                panel_width = gui_context.used_size().x as u32;
            })
            .unwrap();

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
            lights.directional[0].set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
            lights.directional[1].set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));
            lights.spot[0].set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
            lights.spot[0].set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));

            // Draw
            if let Some(Ok(ref model)) = *model.borrow() {
                lights.directional[0]
                    .generate_shadow_map(2.0, 1024, 1024, &[&model])
                    .unwrap();
                lights.directional[1]
                    .generate_shadow_map(2.0, 1024, 1024, &[&model])
                    .unwrap();
                lights.spot[0].generate_shadow_map(1024, &[&model]).unwrap();
            }
            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    if let Some(Ok(ref model)) = *model.borrow() {
                        let material = PhysicalMaterial {
                            name: model.material.name.clone(),
                            albedo: model.material.albedo,
                            albedo_texture: if albedo_map_enabled {
                                model.material.albedo_texture.clone()
                            } else {
                                None
                            },
                            metallic: model.material.metallic,
                            roughness: model.material.roughness,
                            metallic_roughness_texture: if metallic_roughness_enabled {
                                model.material.metallic_roughness_texture.clone()
                            } else {
                                None
                            },
                            normal_scale: model.material.normal_scale,
                            normal_texture: if normal_map_enabled {
                                model.material.normal_texture.clone()
                            } else {
                                None
                            },
                            occlusion_strength: model.material.occlusion_strength,
                            occlusion_texture: if occlusion_map_enabled {
                                model.material.occlusion_texture.clone()
                            } else {
                                None
                            },
                            emissive: if emissive_map_enabled {
                                model.material.emissive
                            } else {
                                Color::BLACK
                            },
                            emissive_texture: if emissive_map_enabled {
                                model.material.emissive_texture.clone()
                            } else {
                                None
                            },
                            opaque_render_states: model.material.opaque_render_states,
                            transparent_render_states: model.material.transparent_render_states,
                            alpha_cutout: model.material.alpha_cutout,
                        };
                        model.render_forward(&material, &camera, &lights)?;
                    }
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
}
