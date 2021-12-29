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
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut gui = three_d::GUI::new(&context).unwrap();

    let scene = Loading::new(
        &context,
        &[
            "examples/assets/gltf/DamagedHelmet.glb",
            "examples/assets/chinese_garden_4k.hdr",
        ],
        move |context, mut loaded| {
            let environment_map = loaded.hdr_image("chinese").unwrap();
            let skybox = Skybox::new_from_equirectangular(&context, &environment_map).unwrap();

            let (mut cpu_meshes, cpu_materials) = loaded.gltf("DamagedHelmet.glb").unwrap();
            let mut material = PhysicalMaterial::new(&context, &cpu_materials[0]).unwrap();
            material.opaque_render_states.cull = Cull::Back;
            cpu_meshes[0].compute_tangents().unwrap();
            let mut model =
                Model::new_with_material(&context, &cpu_meshes[0], material.clone()).unwrap();
            model.set_transformation(Mat4::from_angle_x(degrees(90.0)));

            let lights = Lights {
                ambient: Some(AmbientLight {
                    environment: Some(Environment::new(&context, skybox.texture())?),
                    ..Default::default()
                }),
                lighting_model: LightingModel::Cook(
                    NormalDistributionFunction::TrowbridgeReitzGGX,
                    GeometryFunction::SmithSchlickGGX,
                ),
                ..Default::default()
            };
            Ok((model, skybox, lights))
        },
    );

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

            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    if let Some(ref scene) = *scene.borrow() {
                        let (model, skybox, lights) = scene.as_ref().unwrap();
                        skybox.render(&camera)?;
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
                        };
                        model.render_with_material(&material, &camera, lights)?;
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
