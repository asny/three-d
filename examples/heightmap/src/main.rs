//! Heightmap Example
//!
//! Demonstrates Parallax Occlusion Mapping (POM) with PhysicalMaterial and
//! automatic normal/AO map generation from heightmaps.
//!
//! Shows 9 rendering modes for 3 different materials (brick, concrete, metal):
//! 1. PBR (Albedo + Metallic + Roughness)
//! 2. PBR + Normal
//! 3. PBR + AO
//! 4. PBR + Normal + AO
//! 5. PBR + Height + Normal + AO (POM enabled)
//! 6. PBR + Height only (POM enabled)
//! 7. PBR + Height + Generated Normal
//! 8. PBR + Height + Generated AO
//! 9. PBR + Height + Generated Normal + Generated AO

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

/// A complete set of PBR textures for a material
struct TextureSet {
    name: String,
    albedo: CpuTexture,
    normal: CpuTexture,
    ao: CpuTexture,
    heightmap: CpuTexture,
    metallic_roughness: CpuTexture,
    generated_normal: CpuTexture,
    generated_ao: CpuTexture,
}

/// Track which quads have POM enabled (columns 4-8 have height textures)
fn has_pom(quad_index: usize) -> bool {
    let col = quad_index % 9;
    col >= 4
}

/// Project a 3D world position to 2D screen coordinates
fn world_to_screen(camera: &Camera, world_pos: Vec3, viewport: Viewport) -> Option<(f32, f32)> {
    let view_proj = camera.projection() * camera.view();
    let clip = view_proj * world_pos.extend(1.0);

    // Behind camera check
    if clip.w <= 0.0 {
        return None;
    }

    // Perspective divide to get NDC (-1 to 1)
    let ndc = clip.truncate() / clip.w;

    // Convert NDC to screen coordinates
    let screen_x = (ndc.x + 1.0) * 0.5 * viewport.width as f32 + viewport.x as f32;
    let screen_y = (1.0 - ndc.y) * 0.5 * viewport.height as f32 + viewport.y as f32;

    Some((screen_x, screen_y))
}

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Heightmap (Parallax Occlusion Mapping)".to_string(),
        max_size: Some((1920, 1080)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    // ========================================================================
    // Load textures
    // ========================================================================
    println!("Loading textures...");

    println!("  Loading brick textures...");
    let brick = load_texture_set(
        "brick",
        "examples/assets/textures/brick/brick",
    )
    .await;

    println!("  Loading concrete textures...");
    let concrete = load_texture_set(
        "concrete",
        "examples/assets/textures/concrete/concrete",
    )
    .await;

    println!("  Loading metal textures...");
    let metal = load_texture_set(
        "metal",
        "examples/assets/textures/metal/metal",
    )
    .await;

    println!("All textures loaded!");

    // ========================================================================
    // Create materials and quads for each texture set
    // ========================================================================
    println!("Creating materials...");

    let texture_sets = [brick, concrete, metal];
    let mut all_quads: Vec<Gm<Mesh, PhysicalMaterial>> = Vec::new();

    // Create quad mesh with tangents
    let mut quad_cpu = CpuMesh::square();
    quad_cpu.compute_tangents();

    // Labels for columns and rows
    let col_labels = [
        "PBR",
        "+Normal",
        "+AO",
        "+Normal+AO",
        "+Height+Normal+AO",
        "+Height",
        "+Height+GenNormal",
        "+Height+GenAO",
        "+Height+GenNormal+GenAO",
    ];
    let row_labels = ["Brick", "Concrete", "Metal"];

    // Spacing (8 columns, 3 rows)
    let num_cols = col_labels.len();
    let col_spacing = 2.2_f32;
    let row_spacing = 2.4_f32;
    let start_x = -((num_cols as f32 - 1.0) / 2.0) * col_spacing;
    let start_y = ((texture_sets.len() as f32 - 1.0) / 2.0) * row_spacing;

    for (row, tex_set) in texture_sets.iter().enumerate() {
        println!("  Creating {} materials...", tex_set.name);

        let y = start_y - row as f32 * row_spacing;

        // Preload textures to GPU
        let albedo_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.albedo);
        let metallic_roughness_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.metallic_roughness);
        let normal_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.normal);
        let ao_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.ao);
        let height_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.heightmap);
        let gen_normal_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.generated_normal);
        let gen_ao_tex = Texture2DRef::from_cpu_texture(&context, &tex_set.generated_ao);

        // Column 0: PBR (Albedo + Metallic + Roughness)
        let mat0 = PhysicalMaterial {
            name: format!("{}_pbr", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: None,
            normal_scale: 1.0,
            normal_texture: None,
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: None,
            height_scale: 0.0,
            height_quality: HeightQuality::High,
        };

        // Column 1: PBR + Normal
        let mat1 = PhysicalMaterial {
            name: format!("{}_pbr_normal", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: None,
            normal_scale: 1.0,
            normal_texture: Some(normal_tex.clone()),
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: None,
            height_scale: 0.0,
            height_quality: HeightQuality::High,
        };

        // Column 2: PBR + AO
        let mat2 = PhysicalMaterial {
            name: format!("{}_pbr_ao", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: Some(ao_tex.clone()),
            normal_scale: 1.0,
            normal_texture: None,
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: None,
            height_scale: 0.0,
            height_quality: HeightQuality::High,
        };

        // Column 3: PBR + Normal + AO
        let mat3 = PhysicalMaterial {
            name: format!("{}_pbr_normal_ao", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: Some(ao_tex.clone()),
            normal_scale: 1.0,
            normal_texture: Some(normal_tex.clone()),
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: None,
            height_scale: 0.0,
            height_quality: HeightQuality::High,
        };

        // Column 4: PBR + Height + Normal + AO (POM enabled)
        let mat4 = PhysicalMaterial {
            name: format!("{}_pbr_height_normal_ao", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: Some(ao_tex.clone()),
            normal_scale: 1.0,
            normal_texture: Some(normal_tex.clone()),
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: Some(height_tex.clone()),
            height_scale: 0.05,
            height_quality: HeightQuality::High,
        };

        // Column 5: PBR + Height only (POM enabled)
        let mat5 = PhysicalMaterial {
            name: format!("{}_pbr_height", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: None,
            normal_scale: 1.0,
            normal_texture: None,
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: Some(height_tex.clone()),
            height_scale: 0.05,
            height_quality: HeightQuality::High,
        };

        // Column 6: PBR + Height + Generated Normal
        let mat6 = PhysicalMaterial {
            name: format!("{}_pbr_height_gen_normal", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: None,
            normal_scale: 1.0,
            normal_texture: Some(gen_normal_tex.clone()),
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: Some(height_tex.clone()),
            height_scale: 0.05,
            height_quality: HeightQuality::High,
        };

        // Column 7: PBR + Height + Generated AO
        let mat7 = PhysicalMaterial {
            name: format!("{}_pbr_height_gen_ao", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: Some(gen_ao_tex.clone()),
            normal_scale: 1.0,
            normal_texture: None,
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: Some(height_tex.clone()),
            height_scale: 0.05,
            height_quality: HeightQuality::High,
        };

        // Column 8: PBR + Height + Generated Normal + Generated AO
        let mat8 = PhysicalMaterial {
            name: format!("{}_pbr_height_gen_normal_ao", tex_set.name),
            albedo: Srgba::WHITE,
            albedo_texture: Some(albedo_tex.clone()),
            metallic: 1.0,
            roughness: 1.0,
            metallic_roughness_texture: Some(metallic_roughness_tex.clone()),
            occlusion_strength: 1.0,
            occlusion_texture: Some(gen_ao_tex.clone()),
            normal_scale: 1.0,
            normal_texture: Some(gen_normal_tex.clone()),
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            height_texture: Some(height_tex.clone()),
            height_scale: 0.05,
            height_quality: HeightQuality::High,
        };

        let materials = [mat0, mat1, mat2, mat3, mat4, mat5, mat6, mat7, mat8];

        for (col, mat) in materials.into_iter().enumerate() {
            let x = start_x + col as f32 * col_spacing;
            let mut quad = Gm::new(Mesh::new(&context, &quad_cpu), mat);
            quad.set_transformation(Mat4::from_translation(vec3(x, y, 0.0)));
            all_quads.push(quad);
        }
    }

    println!("Setup complete! Starting render loop...");

    // Camera and controls
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 12.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.05);

    // Lighting
    let ambient = AmbientLight::new(&context, 0.3, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(-1.0, -1.0, -1.0));

    // GUI
    let mut gui = three_d::GUI::new(&context);
    let mut height_scale = 0.05_f32;
    let mut height_quality_idx = 3_usize; // High
    let mut pom_enabled = true;

    // Keyboard state for WASD
    let mut move_forward = false;
    let mut move_backward = false;
    let mut move_left = false;
    let mut move_right = false;
    let move_speed = 0.1_f32;

    // Main loop
    window.render_loop(move |mut frame_input| {
        // Handle keyboard input for WASD
        for event in frame_input.events.iter_mut() {
            match event {
                Event::KeyPress { kind, handled, .. } if !*handled => {
                    match kind {
                        Key::W => move_forward = true,
                        Key::S => move_backward = true,
                        Key::A => move_left = true,
                        Key::D => move_right = true,
                        _ => {}
                    }
                }
                Event::KeyRelease { kind, handled, .. } if !*handled => {
                    match kind {
                        Key::W => move_forward = false,
                        Key::S => move_backward = false,
                        Key::A => move_left = false,
                        Key::D => move_right = false,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Apply WASD movement (W/S = up/down, A/D = left/right)
        let right = camera.right_direction();
        let up = vec3(0.0, 1.0, 0.0);
        let mut movement = vec3(0.0, 0.0, 0.0);
        if move_forward {
            movement += up * move_speed;
        }
        if move_backward {
            movement -= up * move_speed;
        }
        if move_right {
            movement += right * move_speed;
        }
        if move_left {
            movement -= right * move_speed;
        }
        if movement.magnitude() > 0.0 {
            camera.translate(movement);
        }

        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Heightmap Demo");
                    ui.separator();

                    ui.checkbox(&mut pom_enabled, "Enable POM");

                    ui.add(Slider::new(&mut height_scale, 0.0..=0.15).text("Height Scale"));

                    ui.label("Quality:");
                    ComboBox::from_label("")
                        .selected_text(match height_quality_idx {
                            0 => "Very Low",
                            1 => "Low",
                            2 => "Medium",
                            3 => "High",
                            _ => "Very High",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut height_quality_idx, 0, "Very Low");
                            ui.selectable_value(&mut height_quality_idx, 1, "Low");
                            ui.selectable_value(&mut height_quality_idx, 2, "Medium");
                            ui.selectable_value(&mut height_quality_idx, 3, "High");
                            ui.selectable_value(&mut height_quality_idx, 4, "Very High");
                        });

                    ui.separator();
                    ui.label("Controls:");
                    ui.label("  W/S: Up/Down");
                    ui.label("  A/D: Left/Right");
                    ui.label("  Scroll: In/Out");
                    ui.label("  Left-drag: Look");
                    ui.label("  Right-drag: Pan");
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        // Update material parameters
        let quality = match height_quality_idx {
            0 => HeightQuality::VeryLow,
            1 => HeightQuality::Low,
            2 => HeightQuality::Medium,
            3 => HeightQuality::High,
            _ => HeightQuality::VeryHigh,
        };

        for (i, quad) in all_quads.iter_mut().enumerate() {
            // Only apply POM settings to quads in columns 2, 3, 4
            if has_pom(i) {
                if pom_enabled {
                    quad.material.height_scale = height_scale;
                    quad.material.height_quality = quality;
                } else {
                    quad.material.height_scale = 0.0;
                }
            }
        }

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Render scene
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.15, 0.15, 0.18, 1.0, 1.0))
            .render(
                &camera,
                all_quads.iter().collect::<Vec<_>>().as_slice(),
                &[&ambient, &directional],
            )
            .write(|| {
                // Draw 3D labels using egui painter
                let painter = gui.context().layer_painter(three_d::egui::LayerId::new(
                    three_d::egui::Order::Foreground,
                    three_d::egui::Id::new("labels"),
                ));

                // Column labels (above top row)
                for (col, label) in col_labels.iter().enumerate() {
                    let x = start_x + col as f32 * col_spacing;
                    let world_pos = vec3(x, start_y + 1.2, 0.0);
                    if let Some((screen_x, screen_y)) = world_to_screen(&camera, world_pos, viewport)
                    {
                        painter.text(
                            three_d::egui::pos2(screen_x, screen_y),
                            three_d::egui::Align2::CENTER_BOTTOM,
                            label,
                            three_d::egui::FontId::proportional(12.0),
                            three_d::egui::Color32::WHITE,
                        );
                    }
                }

                // Row labels (left of each row)
                for (row, label) in row_labels.iter().enumerate() {
                    let y = start_y - row as f32 * row_spacing;
                    let world_pos = vec3(start_x - 1.2, y, 0.0);
                    if let Some((screen_x, screen_y)) = world_to_screen(&camera, world_pos, viewport)
                    {
                        painter.text(
                            three_d::egui::pos2(screen_x, screen_y),
                            three_d::egui::Align2::RIGHT_CENTER,
                            label,
                            three_d::egui::FontId::proportional(12.0),
                            three_d::egui::Color32::WHITE,
                        );
                    }
                }

                gui.render()
            })
            .unwrap();

        FrameOutput::default()
    });
}

async fn load_texture_set(name: &str, base_path: &str) -> TextureSet {
    let albedo_path = format!("{}_albedo.jpg", base_path);
    let normal_path = format!("{}_normal.jpg", base_path);
    let ao_path = format!("{}_ao.jpg", base_path);
    let height_path = format!("{}_height.jpg", base_path);
    let roughness_path = format!("{}_roughness.jpg", base_path);
    let metallic_path = format!("{}_metallic.jpg", base_path);

    let mut loaded = three_d_asset::io::load_async(&[
        albedo_path.as_str(),
        normal_path.as_str(),
        ao_path.as_str(),
        height_path.as_str(),
        roughness_path.as_str(),
        metallic_path.as_str(),
    ])
    .await
    .unwrap_or_else(|_| panic!("Failed to load {} textures", name));

    let albedo: CpuTexture = loaded.deserialize("albedo").unwrap();
    let normal: CpuTexture = loaded.deserialize("normal").unwrap();
    let ao: CpuTexture = loaded.deserialize("ao").unwrap();
    let heightmap: CpuTexture = loaded.deserialize("height").unwrap();
    let roughness: CpuTexture = loaded.deserialize("roughness").unwrap();
    let metallic: CpuTexture = loaded.deserialize("metallic").unwrap();

    println!("    Generating normal map from heightmap...");
    let generated_normal = create_normal_from_heightmap(&heightmap, 2.0);

    println!("    Generating AO map from heightmap...");
    let generated_ao = create_ao_from_heightmap(&heightmap, 5, 8, 3.0, 0.1);

    let metallic_roughness = create_metallic_roughness(&metallic, &roughness);

    TextureSet {
        name: name.to_string(),
        albedo,
        normal,
        ao,
        heightmap,
        metallic_roughness,
        generated_normal,
        generated_ao,
    }
}

