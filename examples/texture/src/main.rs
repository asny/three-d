// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Texture!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 1.0, 20.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let mut loaded = three_d_asset::io::load_async(&[
        "examples/assets/skybox_evening/right.jpg",
        "examples/assets/skybox_evening/left.jpg",
        "examples/assets/skybox_evening/top.jpg",
        "examples/assets/skybox_evening/front.jpg",
        "examples/assets/skybox_evening/back.jpg",
        "examples/assets/Skybox_example.png",
        "examples/assets/PenguinBaseMesh.obj",
        "examples/assets/checkerboard.jpg",
    ])
    .await
    .unwrap();

    // Skybox
    let top_tex = loaded.deserialize("top").unwrap();
    let skybox = Skybox::new(
        &context,
        &loaded.deserialize("right").unwrap(),
        &loaded.deserialize("left").unwrap(),
        &top_tex,
        &top_tex,
        &loaded.deserialize("front").unwrap(),
        &loaded.deserialize("back").unwrap(),
    );

    // Box
    let mut cpu_texture: CpuTexture = loaded.deserialize("Skybox_example").unwrap();
    cpu_texture.data.to_linear_srgb();
    let mut box_object = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        ColorMaterial {
            texture: Some(Texture2DRef::from_cpu_texture(&context, &cpu_texture)),
            ..Default::default()
        },
    );
    box_object.material.render_states.cull = Cull::Back;

    // Penguin
    let model = loaded.deserialize("PenguinBaseMesh.obj").unwrap();
    let mut penguin = Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    penguin.iter_mut().for_each(|m| {
        m.set_transformation(Mat4::from_translation(vec3(0.0, 1.0, 0.5)));
        m.material.render_states.cull = Cull::Back;
    });

    // Ground
    let mut cpu_texture: CpuTexture = loaded.deserialize("checkerboard").unwrap();
    cpu_texture.data.to_color();
    let mut ground_geometry = CpuMesh::square();
    ground_geometry
        .uvs
        .as_mut()
        .unwrap()
        .iter_mut()
        .for_each(|uv| *uv = 5.0 * (*uv - vec2(0.4, 0.4)));
    let mut ground_object = Gm::new(
        Mesh::new(&context, &ground_geometry),
        ColorMaterial::default(),
    );
    ground_object.set_transformation(
        Mat4::from_translation(vec3(0.0, -1.0, 0.0))
            * Mat4::from_angle_x(degrees(-90.0))
            * Mat4::from_scale(20.0),
    );

    // Lights
    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(0.0, -1.0, -1.0));

    // GUI
    let mut gui = three_d::GUI::new(&context);
    let mut max_ratio = 1;
    let mut max_levels = 8;
    let mut mipmap_filter = Interpolation::Nearest;
    let mut min_filter = Interpolation::Nearest;
    let mut mag_filter = Interpolation::Nearest;
    let mut wrap_s = Wrapping::Repeat;
    let mut wrap_t = Wrapping::Repeat;

    // main loop
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
                    ui.heading("Debug panel for ground texture");
                    ui.label("Wrap s");
                    ui.radio_value(&mut wrap_s, Wrapping::ClampToEdge, "ClampToEdge");
                    ui.radio_value(&mut wrap_s, Wrapping::MirroredRepeat, "MirroredRepeat");
                    ui.radio_value(&mut wrap_s, Wrapping::Repeat, "Repeat");
                    ui.label("Wrap t");
                    ui.radio_value(&mut wrap_t, Wrapping::ClampToEdge, "ClampToEdge");
                    ui.radio_value(&mut wrap_t, Wrapping::MirroredRepeat, "MirroredRepeat");
                    ui.radio_value(&mut wrap_t, Wrapping::Repeat, "Repeat");
                    ui.label("Min filter");
                    ui.radio_value(&mut min_filter, Interpolation::Nearest, "Nearest");
                    ui.radio_value(&mut min_filter, Interpolation::Linear, "Linear");
                    ui.label("Mag filter");
                    ui.radio_value(&mut mag_filter, Interpolation::Nearest, "Nearest");
                    ui.radio_value(&mut mag_filter, Interpolation::Linear, "Linear");
                    ui.label("Mipmap settings");
                    ui.add(Slider::new(&mut max_levels, 1..=8).text("Max levels"));
                    ui.add(Slider::new(&mut max_ratio, 1..=8).text("Max ratio of anisotropy"));
                    ui.radio_value(&mut mipmap_filter, Interpolation::Nearest, "Nearest");
                    ui.radio_value(&mut mipmap_filter, Interpolation::Linear, "Linear");
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Update ground texture texture settings
        let mipmap = Some(Mipmap {
            max_ratio,
            max_levels,
            filter: mipmap_filter,
        });
        if cpu_texture.mipmap != mipmap
            || cpu_texture.min_filter != min_filter
            || cpu_texture.mag_filter != mag_filter
            || cpu_texture.wrap_s != wrap_s
            || cpu_texture.wrap_t != wrap_t
        {
            cpu_texture.min_filter = min_filter;
            cpu_texture.mag_filter = mag_filter;
            cpu_texture.mipmap = mipmap;
            cpu_texture.wrap_s = wrap_s;
            cpu_texture.wrap_t = wrap_t;
            ground_object.material.texture =
                Some(Texture2DRef::from_cpu_texture(&context, &cpu_texture));
        }

        // draw
        frame_input
            .screen()
            .clear(ClearState::default())
            .render(
                &camera,
                penguin
                    .into_iter()
                    .chain(&box_object)
                    .chain(&ground_object)
                    .chain(&skybox),
                &[&ambient, &directional],
            )
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default()
    });
}
