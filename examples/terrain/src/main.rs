// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use noise::{NoiseFn, SuperSimplex};
use rand::prelude::*;
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
        vec3(22.0, 3.0, -8.0),
        vec3(22.75, 3.0, -7.4),
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
    let height_map = std::sync::Arc::new(move |x, y| {
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
    terrain.set_lod(std::sync::Arc::new(|d| {
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
        0.0,
        vec2(0.0, 0.0),
        512.0,
        0.3,
        [],
    );
    let mut water_material = WaterMaterial {
        background: Background::Texture(skybox.texture().clone()),
        metallic: 0.0,
        roughness: 1.0,
        lighting_model: LightingModel::Cook(
            NormalDistributionFunction::TrowbridgeReitzGGX,
            GeometryFunction::SmithSchlickGGX,
        ),
    };

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
    let mut depth_texture =
        DepthTexture2D::new::<f32>(&context, 1, 1, Wrapping::ClampToEdge, Wrapping::ClampToEdge);
    let mut gui = GUI::new(&context);

    let mut wavelength = 3.0;
    let mut wavelength_variation = 0.5;
    let mut amplitude = 0.03;
    let mut amplitude_variation = 0.005;
    let mut steepness = 2.0;
    let mut steepness_variation = 0.5;
    let mut direction_angle = 0.0;
    let mut direction_variation = 0.125 * std::f32::consts::PI;
    let mut speed = 3.0;
    let mut height = 0.0;
    // main loop
    window.render_loop(move |mut frame_input| {
        let mut parameter_change = frame_input.first_frame;
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                egui::Window::new("").vscroll(true).show(gui_context, |ui| {
                    ui.label("Water parameters");
                    ui.add(Slider::new(&mut height, -5.0..=5.0).text("height"));
                    ui.add(Slider::new(&mut water_material.metallic, 0.0..=1.0).text("metallic"));
                    ui.add(Slider::new(&mut water_material.roughness, 0.0..=1.0).text("roughness"));

                    ui.label("Wave parameters");
                    parameter_change |= ui
                        .add(Slider::new(&mut wavelength, 0.0..=10.0).text("Wavelength"))
                        .changed();
                    parameter_change |= ui
                        .add(
                            Slider::new(&mut wavelength_variation, 0.0..=5.0)
                                .text("Wavelength variation"),
                        )
                        .changed();
                    parameter_change |= ui
                        .add(Slider::new(&mut amplitude, 0.0..=0.2).text("Amplitude"))
                        .changed();
                    parameter_change |= ui
                        .add(
                            Slider::new(&mut amplitude_variation, 0.0..=0.1)
                                .text("Amplitude variation"),
                        )
                        .changed();
                    parameter_change |= ui
                        .add(Slider::new(&mut steepness, 0.0..=10.0).text("Steepness"))
                        .changed();
                    parameter_change |= ui
                        .add(
                            Slider::new(&mut steepness_variation, 0.0..=5.0)
                                .text("Steepness variation"),
                        )
                        .changed();
                    parameter_change |= ui
                        .add(Slider::new(&mut speed, 0.0..=20.0).text("Speed"))
                        .changed();
                    parameter_change |= ui
                        .add(
                            Slider::new(&mut direction_angle, 0.0..=2.0 * std::f32::consts::PI)
                                .text("Direction angle"),
                        )
                        .changed();
                    parameter_change |= ui
                        .add(
                            Slider::new(&mut direction_variation, 0.0..=std::f32::consts::PI)
                                .text("Direction variation"),
                        )
                        .changed();
                });
            },
        );
        change |= parameter_change;
        change |= control.handle_events(&mut camera, &mut frame_input.events);

        if parameter_change {
            let mut rng = rand::thread_rng();
            let mut parameters = [WaveParameters {
                speed,
                ..Default::default()
            }; 4];
            rng.gen::<[f32; 4]>()
                .into_iter()
                .enumerate()
                .for_each(|(i, x)| {
                    let angle = direction_angle + direction_variation * (2.0 * x - 1.0);
                    parameters[i].direction = vec2(angle.cos(), angle.sin()).normalize();
                });
            rng.gen::<[f32; 4]>()
                .into_iter()
                .enumerate()
                .for_each(|(i, x)| {
                    parameters[i].wavelength = wavelength + wavelength_variation * (2.0 * x - 1.0);
                });
            rng.gen::<[f32; 4]>()
                .into_iter()
                .enumerate()
                .for_each(|(i, x)| {
                    parameters[i].amplitude = amplitude + amplitude_variation * (2.0 * x - 1.0);
                });
            rng.gen::<[f32; 4]>()
                .into_iter()
                .enumerate()
                .for_each(|(i, x)| {
                    parameters[i].steepness = steepness + steepness_variation * (2.0 * x - 1.0);
                });
            water.set_parameters(parameters);
        }
        water.set_height(height);

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
        water.animate(frame_input.accumulated_time as f32);

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
            depth_texture = DepthTexture2D::new::<f32>(
                &context,
                frame_input.viewport.width,
                frame_input.viewport.height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            RenderTarget::new(
                color_texture.as_color_target(None),
                depth_texture.as_depth_target(),
            )
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, skybox.into_iter().chain(&terrain), &[&light]);
        }
        frame_input
            .screen()
            .copy_from(
                ColorTexture::Single(&color_texture),
                DepthTexture::Single(&depth_texture),
                camera.viewport(),
                WriteMask::default(),
            )
            .render_with_post_material(
                &water_material,
                &camera,
                &water,
                &[&light],
                Some(ColorTexture::Single(&color_texture)),
                Some(DepthTexture::Single(&depth_texture)),
            )
            .write(|| {
                gui.render();
            });

        FrameOutput::default()
    });
}
