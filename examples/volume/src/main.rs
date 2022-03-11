// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}

use three_d::*;

pub async fn run(screenshot: Option<std::path::PathBuf>) {
    let window = Window::new(WindowSettings {
        title: "Volume!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(4.0, 0.5, 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Source: https://web.cs.ucdavis.edu/~okreylos/PhDStudies/Spring2000/ECS277/DataSets.html
    let bytes = Loader::load_async(&["examples/assets/C60Small.vol"])
        .await
        .unwrap()
        .get_bytes("")
        .unwrap()
        .to_vec();
    let cpu_volume = CpuVolume {
        voxels: CpuTexture3D {
            data: bytes[28..].to_vec(),
            width: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            height: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            depth: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            format: Format::R,
            ..Default::default()
        },
        size: vec3(
            f32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
            f32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            f32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
        ),
        ..Default::default()
    };
    let volume = Volume::new(&context, &cpu_volume).unwrap();

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, -1.0)).unwrap();

    // main loop
    window
        .render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();
            redraw |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            // draw
            if redraw {
                Screen::write(
                    &context,
                    ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                    || render_pass(&camera, &[&volume], &[&ambient, &directional]),
                )
                .unwrap();
            }

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput {
                    swap_buffers: redraw,
                    ..Default::default()
                }
            }
        })
        .unwrap();
}
