// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum CameraType {
    Primary,
    Secondary,
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Statues!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut primary_camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-300.0, 250.0, 200.0),
        vec3(0.0, 100.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    )
    .unwrap();
    // Static camera to view frustum culling in effect
    let mut secondary_camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-600.0, 600.0, 600.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(
        *primary_camera.target(),
        0.5 * primary_camera.target().distance(*primary_camera.position()),
        5.0 * primary_camera.target().distance(*primary_camera.position()),
    );

    // Models from http://texturedmesh.isti.cnr.it/
    let mut loaded = three_d_io::Loader::load_async(&[
        "examples/assets/COLOMBE.obj",
        "examples/assets/COLOMBE.mtl",
        "examples/assets/COLOMBE.png",
        "examples/assets/pfboy.obj",
        "examples/assets/pfboy.mtl",
        "examples/assets/pfboy.png",
    ])
    .await
    .unwrap();

    let (statue_cpu_meshes, statue_cpu_materials) =
        loaded.obj("examples/assets/COLOMBE.obj").unwrap();

    let mut models = Vec::new();
    let scale = Mat4::from_scale(10.0);
    for i in 0..8 {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / 8.0;
        let rotation = Mat4::from_angle_y(radians(0.8 * std::f32::consts::PI - angle));
        let dist = 300.0;
        let translation = Mat4::from_translation(vec3(
            angle.cos() * dist,
            (1.2 * std::f32::consts::PI - angle).cos() * 21.0 - 33.0,
            angle.sin() * dist,
        ));
        let mut statue_material =
            PhysicalMaterial::new(&context, &statue_cpu_materials[0]).unwrap();
        statue_material.render_states.cull = Cull::Back;
        let mut statue =
            Model::new_with_material(&context, &statue_cpu_meshes[0], statue_material).unwrap();
        statue.set_transformation(translation * scale * rotation);
        models.push(statue);
    }

    let (fountain_cpu_meshes, fountain_cpu_materials) =
        loaded.obj("examples/assets/pfboy.obj").unwrap();
    let mut fountain_material =
        PhysicalMaterial::new(&context, &fountain_cpu_materials[0]).unwrap();
    fountain_material.render_states.cull = Cull::Back;
    let mut fountain =
        Model::new_with_material(&context, &fountain_cpu_meshes[0], fountain_material).unwrap();
    fountain.set_transformation(Mat4::from_angle_x(degrees(-90.0)));
    models.push(fountain);

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let mut directional = DirectionalLight::new(
        &context,
        10.0,
        Color::new_opaque(204, 178, 127),
        &vec3(0.0, -1.0, -1.0),
    )
    .unwrap();
    directional
        .generate_shadow_map(
            1024,
            &models
                .iter()
                .map(|m| m as &dyn Geometry)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    // Bounding boxes
    let mut aabb = AxisAlignedBoundingBox::EMPTY;
    let mut bounding_boxes = Vec::new();
    for geometry in models.iter() {
        bounding_boxes.push(
            BoundingBox::new_with_material_and_thickness(
                &context,
                geometry.aabb(),
                ColorMaterial {
                    color: Color::RED,
                    ..Default::default()
                },
                0.5,
            )
            .unwrap(),
        );
        aabb.expand_with_aabb(&geometry.aabb());
    }
    bounding_boxes.push(
        BoundingBox::new_with_material_and_thickness(
            &context,
            aabb,
            ColorMaterial {
                color: Color::BLACK,
                ..Default::default()
            },
            3.0,
        )
        .unwrap(),
    );

    let mut gui = three_d::GUI::new(&context).unwrap();
    let mut camera_type = CameraType::Primary;
    let mut bounding_box_enabled = false;
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = 0.0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.radio_value(&mut camera_type, CameraType::Primary, "Primary camera");
                    ui.radio_value(&mut camera_type, CameraType::Secondary, "Secondary camera");

                    ui.checkbox(&mut bounding_box_enabled, "Bounding boxes");
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
            primary_camera.set_viewport(viewport).unwrap();
            secondary_camera.set_viewport(viewport).unwrap();
            control
                .handle_events(&mut primary_camera, &mut frame_input.events)
                .unwrap();

            // draw
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.7, 1.0, 1.0))
                .unwrap()
                .write(|| {
                    let camera = match camera_type {
                        CameraType::Primary => &primary_camera,
                        CameraType::Secondary => &secondary_camera,
                    };
                    for model in models
                        .iter()
                        .filter(|o| primary_camera.in_frustum(&o.aabb()))
                    {
                        model.render(camera, &[&ambient, &directional])?;
                    }
                    if bounding_box_enabled {
                        for bounding_box in bounding_boxes.iter() {
                            bounding_box.render(camera, &[])?;
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
