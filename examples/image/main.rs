use three_d::core::*;
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Image!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();
    let image_effect = ImageEffect::new(&context, include_str!("shader.frag")).unwrap();

    let image = Loading::new(
        &context,
        &["examples/assets/chinese_garden_4k.hdr"],
        move |context, mut loaded| Texture2D::new(&context, &loaded.hdr_image("")?),
    );

    // main loop
    window
        .render_loop(move |frame_input| {
            let mut redraw = frame_input.first_frame;

            if redraw {
                Screen::write(&context, ClearState::color(0.0, 1.0, 1.0, 1.0), || {
                    if let Some(ref image) = *image.borrow() {
                        let image = image.as_ref().unwrap();
                        image_effect.use_texture("image", &image)?;
                        image_effect.apply(RenderStates::default(), frame_input.viewport)?;
                    }
                    Ok(())
                })
                .unwrap();
            }

            if args.len() > 1 {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(args[1].clone().into()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput {
                    swap_buffers: redraw,
                    wait_next_event: true,
                    ..Default::default()
                }
            }
        })
        .unwrap();
}
