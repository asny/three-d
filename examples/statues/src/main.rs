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
    let context = window.gl();

    let mut primary_camera = Camera::new_perspective(
        window.viewport(),
        vec3(-300.0, 250.0, 200.0),
        vec3(0.0, 100.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    // Static camera to view frustum culling in effect
    let mut secondary_camera = Camera::new_perspective(
        window.viewport(),
        vec3(-600.0, 600.0, 600.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut control = OrbitControl::new(
        *primary_camera.target(),
        0.5 * primary_camera.target().distance(*primary_camera.position()),
        5.0 * primary_camera.target().distance(*primary_camera.position()),
    );

    // Models from http://texturedmesh.isti.cnr.it/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/COLOMBE.obj", "../assets/pfboy.obj"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/COLOMBE.obj",
            "https://asny.github.io/three-d/assets/pfboy.obj",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let cpu_model: CpuModel = loaded.deserialize("COLOMBE.obj").unwrap();

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
        let mut statue = Model::<PhysicalMaterial>::new(&context, &cpu_model).unwrap();
        statue.iter_mut().for_each(|m| {
            m.set_transformation(translation * scale * rotation);
            m.material.render_states.cull = Cull::Back;
        });
        models.push(statue);
    }

    let mut fountain =
        Model::<PhysicalMaterial>::new(&context, &loaded.deserialize("pfboy.obj").unwrap())
            .unwrap();
    fountain.iter_mut().for_each(|m| {
        m.material.render_states.cull = Cull::Back;
        m.set_transformation(Mat4::from_angle_x(degrees(-90.0)));
    });

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let mut directional = DirectionalLight::new(
        &context,
        10.0,
        Color::new_opaque(204, 178, 127),
        &vec3(0.0, -1.0, -1.0),
    );
    directional.generate_shadow_map(
        1024,
        models.iter().flat_map(|m| m.into_iter()).chain(&fountain),
    );
    // Bounding boxes
    let mut aabb = AxisAlignedBoundingBox::EMPTY;
    let mut bounding_boxes = Vec::new();
    for geometry in models.iter().flat_map(|m| m.into_iter()).chain(&fountain) {
        bounding_boxes.push(Gm::new(
            BoundingBox::new_with_thickness(&context, geometry.aabb(), 0.5),
            ColorMaterial {
                color: Color::RED,
                ..Default::default()
            },
        ));
        aabb.expand_with_aabb(&geometry.aabb());
    }
    bounding_boxes.push(Gm::new(
        BoundingBox::new_with_thickness(&context, aabb, 3.0),
        ColorMaterial {
            color: Color::BLACK,
            ..Default::default()
        },
    ));

    let mut gui = three_d::GUI::new(&context);
    let mut camera_type = CameraType::Primary;
    let mut bounding_box_enabled = false;
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.radio_value(&mut camera_type, CameraType::Primary, "Primary camera");
                    ui.radio_value(&mut camera_type, CameraType::Secondary, "Secondary camera");

                    ui.checkbox(&mut bounding_box_enabled, "Bounding boxes");
                });
                panel_width = gui_context.used_rect().width() as f64;
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        primary_camera.set_viewport(viewport);
        secondary_camera.set_viewport(viewport);
        control.handle_events(&mut primary_camera, &mut frame_input.events);

        // draw
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.7, 1.0, 1.0))
            .write(|| {
                let camera = match camera_type {
                    CameraType::Primary => &primary_camera,
                    CameraType::Secondary => &secondary_camera,
                };
                for object in models
                    .iter()
                    .flatten()
                    .chain(&fountain)
                    .filter(|o| primary_camera.in_frustum(&o.aabb()))
                {
                    object.render(camera, &[&ambient, &directional]);
                }
                if bounding_box_enabled {
                    for bounding_box in bounding_boxes.iter() {
                        bounding_box.render(camera, &[]);
                    }
                }
                gui.render();
            });

        FrameOutput::default()
    });
}
