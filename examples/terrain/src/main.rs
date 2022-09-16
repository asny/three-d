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
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/chinese_garden_4k.hdr"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/chinese_garden_4k.hdr",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let skybox = Skybox::new_from_equirectangular(
        &context,
        &loaded.deserialize("chinese_garden_4k").unwrap(),
    );
    let light = AmbientLight::new_with_environment(&context, 1.0, Color::WHITE, skybox.texture());

    let noise_generator = SuperSimplex::new();
    let height_map = move |x, y| {
        (noise_generator.get([x as f64 * 0.1, y as f64 * 0.1])
            + 0.25 * noise_generator.get([x as f64 * 0.5, y as f64 * 0.5])
            + 2.0 * noise_generator.get([x as f64 * 0.02, y as f64 * 0.02])) as f32
    };

    let mut terrain_geometry = Terrain::new(&context, &height_map, vec3(0.0, 0.0, 0.0));
    let terrain_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            roughness: 1.0,
            metallic: 0.2,
            albedo: Color::new_opaque(150, 170, 150),
            ..Default::default()
        },
    );

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
        change |= control.handle_events(&mut camera, &mut frame_input.events);

        terrain_geometry.update(*camera.position(), &height_map);

        if change {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
                .render(
                    &camera,
                    skybox
                        .obj_iter()
                        .chain(Gm::new(&terrain_geometry, &terrain_material).obj_iter()),
                    light.iter(),
                );
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
