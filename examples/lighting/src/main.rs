// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

#[derive(Debug, Eq, PartialEq)]
enum Pipeline {
    Forward,
    Deferred,
}
use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Lighting!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let forward_pipeline = ForwardPipeline::new(&context).unwrap();
    let mut deferred_pipeline = DeferredPipeline::new(&context).unwrap();
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(2.0, 2.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        30.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut gui = three_d::GUI::new(&context).unwrap();

    // Source: https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0
    let mut models: CpuModels =
        three_d_asset::io::load_async(&["examples/assets/gltf/DamagedHelmet.glb"])
            .await
            .unwrap()
            .deserialize("")
            .unwrap();
    models.geometries[0].compute_tangents().unwrap();
    let mut model = Models::<PhysicalMaterial>::new(&context, &models)
        .unwrap()
        .remove(0);

    let mut plane = Model::new_with_material(
        &context,
        &CpuMesh::square(),
        PhysicalMaterial {
            albedo: Color::new_opaque(128, 200, 70),
            ..Default::default()
        },
    )
    .unwrap();
    plane.set_transformation(
        Mat4::from_translation(vec3(0.0, -1.0, 0.0))
            * Mat4::from_scale(10.0)
            * Mat4::from_angle_x(degrees(-90.0)),
    );

    let mut ambient = AmbientLight::new(&context, 0.2, Color::WHITE).unwrap();
    let mut directional0 =
        DirectionalLight::new(&context, 1.0, Color::RED, &vec3(0.0, -1.0, 0.0)).unwrap();
    let mut directional1 =
        DirectionalLight::new(&context, 1.0, Color::GREEN, &vec3(0.0, -1.0, 0.0)).unwrap();
    let mut spot0 = SpotLight::new(
        &context,
        2.0,
        Color::BLUE,
        &vec3(0.0, 0.0, 0.0),
        &vec3(0.0, -1.0, 0.0),
        degrees(25.0),
        Attenuation {
            constant: 0.1,
            linear: 0.001,
            quadratic: 0.0001,
        },
    )
    .unwrap();
    let mut point0 = PointLight::new(
        &context,
        1.0,
        Color::GREEN,
        &vec3(0.0, 0.0, 0.0),
        Attenuation {
            constant: 0.5,
            linear: 0.05,
            quadratic: 0.005,
        },
    )
    .unwrap();
    let mut point1 = PointLight::new(
        &context,
        1.0,
        Color::RED,
        &vec3(0.0, 0.0, 0.0),
        Attenuation {
            constant: 0.5,
            linear: 0.05,
            quadratic: 0.005,
        },
    )
    .unwrap();

    // main loop
    let mut shadows_enabled = true;
    let mut lighting_model = LightingModel::Blinn;
    let mut current_pipeline = Pipeline::Forward;

    window
        .render_loop(move |mut frame_input| {
            let mut change = frame_input.first_frame;
            let mut panel_width = 0.0;
            change |= gui
                .update(&mut frame_input, |gui_context| {
                    use three_d::egui::*;
                    SidePanel::left("side_panel").show(gui_context, |ui| {
                        ui.heading("Debug Panel");

                        ui.label("Surface parameters");
                        ui.add(
                            Slider::new::<f32>(&mut model.material.metallic, 0.0..=1.0)
                                .text("Model Metallic"),
                        );
                        ui.add(
                            Slider::new::<f32>(&mut model.material.roughness, 0.0..=1.0)
                                .text("Model Roughness"),
                        );
                        ui.add(
                            Slider::new(&mut plane.material.metallic, 0.0..=1.0)
                                .text("Plane Metallic"),
                        );
                        ui.add(
                            Slider::new(&mut plane.material.roughness, 0.0..=1.0)
                                .text("Plane Roughness"),
                        );

                        ui.label("Light options");
                        ui.add(
                            Slider::new(&mut ambient.intensity, 0.0..=1.0)
                                .text("Ambient intensity"),
                        );
                        ui.add(
                            Slider::new(&mut directional0.intensity, 0.0..=1.0)
                                .text("Directional 0 intensity"),
                        );
                        ui.add(
                            Slider::new(&mut directional1.intensity, 0.0..=1.0)
                                .text("Directional 1 intensity"),
                        );
                        ui.add(Slider::new(&mut spot0.intensity, 0.0..=1.0).text("Spot intensity"));
                        ui.add(
                            Slider::new(&mut point0.intensity, 0.0..=1.0).text("Point 0 intensity"),
                        );
                        ui.add(
                            Slider::new(&mut point1.intensity, 0.0..=1.0).text("Point 1 intensity"),
                        );
                        if ui.checkbox(&mut shadows_enabled, "Shadows").clicked() {
                            if !shadows_enabled {
                                spot0.clear_shadow_map();
                                directional0.clear_shadow_map();
                                directional1.clear_shadow_map();
                            }
                        }

                        ui.label("Lighting model");
                        ui.radio_value(&mut lighting_model, LightingModel::Phong, "Phong");
                        ui.radio_value(&mut lighting_model, LightingModel::Blinn, "Blinn");
                        ui.radio_value(
                            &mut lighting_model,
                            LightingModel::Cook(
                                NormalDistributionFunction::Blinn,
                                GeometryFunction::SmithSchlickGGX,
                            ),
                            "Cook (Blinn)",
                        );
                        ui.radio_value(
                            &mut lighting_model,
                            LightingModel::Cook(
                                NormalDistributionFunction::Beckmann,
                                GeometryFunction::SmithSchlickGGX,
                            ),
                            "Cook (Beckmann)",
                        );
                        ui.radio_value(
                            &mut lighting_model,
                            LightingModel::Cook(
                                NormalDistributionFunction::TrowbridgeReitzGGX,
                                GeometryFunction::SmithSchlickGGX,
                            ),
                            "Cook (Trowbridge-Reitz GGX)",
                        );

                        ui.label("Pipeline");
                        ui.radio_value(&mut current_pipeline, Pipeline::Forward, "Forward");
                        ui.radio_value(&mut current_pipeline, Pipeline::Deferred, "Deferred");
                        ui.label("Debug options");
                        ui.radio_value(&mut deferred_pipeline.debug_type, DebugType::NONE, "None");
                        ui.radio_value(
                            &mut deferred_pipeline.debug_type,
                            DebugType::POSITION,
                            "Position",
                        );
                        ui.radio_value(
                            &mut deferred_pipeline.debug_type,
                            DebugType::NORMAL,
                            "Normal",
                        );
                        ui.radio_value(
                            &mut deferred_pipeline.debug_type,
                            DebugType::COLOR,
                            "Color",
                        );
                        ui.radio_value(&mut deferred_pipeline.debug_type, DebugType::UV, "UV");
                        ui.radio_value(
                            &mut deferred_pipeline.debug_type,
                            DebugType::DEPTH,
                            "Depth",
                        );
                        ui.radio_value(&mut deferred_pipeline.debug_type, DebugType::ORM, "ORM");
                    });
                    panel_width = gui_context.used_size().x as f64;
                })
                .unwrap();

            let viewport = Viewport {
                x: (panel_width * frame_input.device_pixel_ratio) as i32,
                y: 0,
                width: frame_input.viewport.width
                    - (panel_width * frame_input.device_pixel_ratio) as u32,
                height: frame_input.viewport.height,
            };
            change |= camera.set_viewport(viewport).unwrap();
            change |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            let time = 0.001 * frame_input.accumulated_time;
            let c = time.cos() as f32;
            let s = time.sin() as f32;
            directional0.direction = vec3(-1.0 - c, -1.0, 1.0 + s);
            directional1.direction = vec3(1.0 + c, -1.0, -1.0 - s);
            spot0.position = vec3(3.0 + c, 5.0 + s, 3.0 - s);
            spot0.direction = -vec3(3.0 + c, 5.0 + s, 3.0 - s);
            point0.position = vec3(-5.0 * c, 5.0, -5.0 * s);
            point1.position = vec3(5.0 * c, 5.0, 5.0 * s);

            model.material.lighting_model = lighting_model;

            // Draw
            if shadows_enabled {
                directional0.generate_shadow_map(1024, &[&model]).unwrap();
                directional1.generate_shadow_map(1024, &[&model]).unwrap();
                spot0.generate_shadow_map(1024, &[&model]).unwrap();
            }

            // Geometry pass
            if change && current_pipeline == Pipeline::Deferred {
                deferred_pipeline
                    .render_pass(
                        &camera,
                        &[
                            (
                                &model,
                                &DeferredPhysicalMaterial::from_physical_material(&model.material),
                            ),
                            (
                                &plane,
                                &DeferredPhysicalMaterial::from_physical_material(&plane.material),
                            ),
                        ],
                    )
                    .unwrap();
            }

            let lights = [
                &ambient as &dyn Light,
                &spot0,
                &directional0,
                &directional1,
                &point0,
                &point1,
            ];

            // Light pass
            frame_input
                .screen()
                .clear(ClearState::default())
                .unwrap()
                .write(|| {
                    match current_pipeline {
                        Pipeline::Forward => {
                            match deferred_pipeline.debug_type {
                                DebugType::NORMAL => {
                                    plane.render_with_material(
                                        &NormalMaterial::from_physical_material(&plane.material),
                                        &camera,
                                        &lights,
                                    )?;
                                    model.render_with_material(
                                        &NormalMaterial::from_physical_material(&model.material),
                                        &camera,
                                        &lights,
                                    )?;
                                }
                                DebugType::DEPTH => {
                                    let depth_material = DepthMaterial::default();
                                    plane.render_with_material(
                                        &depth_material,
                                        &camera,
                                        &lights,
                                    )?;
                                    model.render_with_material(
                                        &depth_material,
                                        &camera,
                                        &lights,
                                    )?;
                                }
                                DebugType::ORM => {
                                    plane.render_with_material(
                                        &ORMMaterial::from_physical_material(&plane.material),
                                        &camera,
                                        &lights,
                                    )?;
                                    model.render_with_material(
                                        &ORMMaterial::from_physical_material(&model.material),
                                        &camera,
                                        &lights,
                                    )?;
                                }
                                DebugType::POSITION => {
                                    let position_material = PositionMaterial::default();
                                    plane.render_with_material(
                                        &position_material,
                                        &camera,
                                        &lights,
                                    )?;
                                    model.render_with_material(
                                        &position_material,
                                        &camera,
                                        &lights,
                                    )?;
                                }
                                DebugType::UV => {
                                    let uv_material = UVMaterial::default();
                                    plane.render_with_material(&uv_material, &camera, &lights)?;
                                    model.render_with_material(&uv_material, &camera, &lights)?;
                                }
                                DebugType::COLOR => {
                                    plane.render_with_material(
                                        &ColorMaterial::from_physical_material(&plane.material),
                                        &camera,
                                        &lights,
                                    )?;
                                    model.render_with_material(
                                        &ColorMaterial::from_physical_material(&model.material),
                                        &camera,
                                        &lights,
                                    )?;
                                }
                                DebugType::NONE => forward_pipeline.render_pass(
                                    &camera,
                                    &[&plane, &model],
                                    &lights,
                                )?,
                            };
                        }
                        Pipeline::Deferred => {
                            deferred_pipeline.lighting_pass(&camera, &lights)?;
                        }
                    }
                    gui.render()?;
                    Ok(())
                })
                .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}
