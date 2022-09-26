// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use noise::{NoiseFn, SuperSimplex};
use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Terrain!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-3.0, 3.0, 2.5),
        vec3(0.0, 3.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = FirstPersonControl::new(0.01);

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&[
        "../assets/syferfontein_18d_clear_4k.hdr",
        "../assets/rocks_ground_01_4k/rocks_ground_01_4k.gltf",
    ])
    .await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/syferfontein_18d_clear_4k.hdr",
            "https://asny.github.io/three-d/assets/rocks_ground_01_4k/rocks_ground_01_4k.gltf",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let skybox = Skybox::new_from_equirectangular(&context, &loaded.deserialize("hdr").unwrap());
    let light = AmbientLight::new_with_environment(&context, 1.0, Color::WHITE, skybox.texture());

    let noise_generator = SuperSimplex::new();
    let height_map = Box::new(move |x, y| {
        (noise_generator.get([x as f64 * 0.1, y as f64 * 0.1])
            + 0.25 * noise_generator.get([x as f64 * 0.5, y as f64 * 0.5])
            + 2.0 * noise_generator.get([x as f64 * 0.02, y as f64 * 0.02])) as f32
    });

    let model: CpuModel = loaded.deserialize(".gltf").unwrap();
    let terrain_material = PhysicalMaterial::new_opaque(&context, &model.materials[0]);
    let mut terrain = Terrain::new(
        &context,
        terrain_material,
        height_map,
        1024.0,
        0.3,
        vec2(0.0, 0.0),
    );
    terrain.set_lod(Box::new(|d| {
        if d > 256.0 {
            Lod::Low
        } else if d > 128.0 {
            Lod::Medium
        } else {
            Lod::High
        }
    }));
    let mut water = Water::new(
        &context,
        NormalMaterial::default(),
        512.0,
        0.3,
        vec2(0.0, 0.0),
        WaterParameters::default(),
    );

    let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
        &context,
        1,
        1,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    let mut depth_texture = DepthTargetTexture2D::new(
        &context,
        1,
        1,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        DepthFormat::Depth32F,
    );
    let mut gui = GUI::new(&context);

    let mut parameters = WaterParameters::default();
    // main loop
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);

        change |= gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                egui::Window::new("").vscroll(true).show(gui_context, |ui| {
                    ui.set_max_width(200.0);
                    ui.set_min_width(200.0);

                    ui.add(
                        Slider::new(&mut parameters.min_wavelength, 0.0..=5.0)
                            .text("Min wavelength"),
                    );
                    ui.add(
                        Slider::new(&mut parameters.max_wavelength, 0.0..=10.0)
                            .text("Max wavelength"),
                    );
                });
            },
        );
        water.set_parameters(parameters);

        change |= control.handle_events(&mut camera, &mut frame_input.events);

        let p = vec2(camera.position().x, camera.position().z);
        let y_new = terrain.height_at(p) + 3.0;
        let target = vec3(
            camera.target().x,
            camera.target().y + y_new - camera.position().y,
            camera.target().z,
        );
        camera.set_view(vec3(p.x, y_new, p.y), target, *camera.up());

        terrain.set_center(p);
        water.set_center(p);
        water.update_animation(frame_input.accumulated_time);

        if change {
            color_texture = Texture2D::new_empty::<[u8; 4]>(
                &context,
                frame_input.viewport.width,
                frame_input.viewport.height,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            depth_texture = DepthTargetTexture2D::new(
                &context,
                frame_input.viewport.width,
                frame_input.viewport.height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            );
            RenderTarget::new(
                color_texture.as_color_target(None),
                depth_texture.as_depth_target(),
            )
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(
                &camera,
                skybox.obj_iter().chain(terrain.obj_iter()),
                light.iter(),
            );
        }
        frame_input
            .screen()
            .copy_from(
                Some(&color_texture),
                Some(&depth_texture),
                frame_input.viewport.into(),
                WriteMask::default(),
            )
            .render_with_material(
                &WaterMaterial {
                    environment_texture: skybox.texture(),
                    color_texture: &color_texture,
                    depth_texture: &depth_texture,
                    metallic: 0.0,
                    roughness: 1.0,
                    lighting_model: LightingModel::Cook(
                        NormalDistributionFunction::TrowbridgeReitzGGX,
                        GeometryFunction::SmithSchlickGGX,
                    ),
                },
                &camera,
                water.geo_iter(),
                light.iter(),
            )
            .write(|| {
                gui.render(frame_input.viewport);
            });

        FrameOutput::default()
    });
}
