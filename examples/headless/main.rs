use three_d::Viewport;
use three_d::*;

fn main() {
    let viewport = Viewport::new_at_origo(1280, 720);

    // create headless graphic context
    let mut headless_context = HeadlessContext::new().unwrap();
    // Get the graphics context from the HeadlessContext
    let context = headless_context.gl();

    // Create a camera
    let camera = Camera::new_perspective(
        &context,
        viewport,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    )
    .unwrap();

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        0.0, 0.5, 0.0, // top
    ];
    let colors: Vec<u8> = vec![
        255, 0, 0, 255, // bottom right
        0, 255, 0, 255, // bottom left
        0, 0, 255, 255, // top
    ];
    let cpu_mesh = CPUMesh {
        positions,
        colors: Some(colors),
        ..Default::default()
    };

    let mut texture = Texture2D::<u8>::new_empty(
        &context,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        Format::RGBA,
    )
    .unwrap();
    let mut depth_texture = DepthTargetTexture2D::new(
        &context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        DepthFormat::Depth32F,
    )
    .unwrap();

    let mut model = Model::new(&context, &cpu_mesh).unwrap();

    // Render three frames
    for frame_index in 0..3 {
        // Start writing to the screen and clears the color and depth
        RenderTarget::new(&context, &mut texture, &mut depth_texture)
            .unwrap()
            .write(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0), || {
                // Set the current transformation of the triangle
                model.set_transformation(Mat4::from_angle_y(radians(
                    (frame_index as f32 * 0.6) as f32,
                )));

                // Render the triangle with the per vertex colors defined at construction
                model.render(&camera, &Lights::default())?;
                Ok(())
            })
            .unwrap();

        let path = format!("headless-{}.png", frame_index);
        let pixels = texture.read(viewport).unwrap();
        Saver::save_pixels(path, &pixels, viewport.width, viewport.height).unwrap();
    }
}
