// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}

use three_d::core::*;
use three_d::*;

pub async fn run(screenshot: Option<std::path::PathBuf>) {
    let window = Window::new(WindowSettings {
        title: "Image!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();
    let image_effect = ImageEffect::new(&context, include_str!("shader.frag")).unwrap();

    let mut loaded = Loader::load_async(
        &["examples/assets/syferfontein_18d_clear_4k.hdr"], // Source: https://polyhaven.com/
    )
    .await
    .unwrap();
    let image = Texture2D::new(&context, &loaded.hdr_image("").unwrap()).unwrap();

    let mut gui = GUI::new(&context).unwrap();

    // main loop
    let mut tone_mapping = 1.0;
    window
        .render_loop(move |mut frame_input| {
            let mut panel_width = 0;
            gui.update(&mut frame_input, |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut tone_mapping, 0.0..=50.0).text("Tone mapping"));
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

            Screen::write(&context, ClearState::default(), || {
                image_effect.use_texture("image", &image)?;
                image_effect.use_uniform("parameter", tone_mapping)?;
                image_effect.apply(RenderStates::default(), viewport)?;
                gui.render()?;
                Ok(())
            })
            .unwrap();

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
