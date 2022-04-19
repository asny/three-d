use three_d::*;

fn main() {
    let viewport = Viewport::new_at_origo(1280, 720);

    // Create a headless graphics context
    let context = Context::new().unwrap();

    // Create a camera
    let camera = Camera::new_perspective(
        &context,
        viewport,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10.0,
    )
    .unwrap();

    // Create the scene - a single colored triangle
    let mut model = Model::new(
        &context,
        &CpuMesh {
            positions: Positions::F32(vec![
                vec3(0.5, -0.5, 0.0),  // bottom right
                vec3(-0.5, -0.5, 0.0), // bottom left
                vec3(0.0, 0.5, 0.0),   // top
            ]),
            colors: Some(vec![
                Color::new(255, 0, 0, 255), // bottom right
                Color::new(0, 255, 0, 255), // bottom left
                Color::new(0, 0, 255, 255), // top
            ]),
            ..Default::default()
        },
    )
    .unwrap();

    // Create a color texture to render into
    let mut texture = Texture2D::new_empty::<[u8; 4]>(
        &context,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    )
    .unwrap();

    // Also create a depth texture to support depth testing
    let mut depth_texture = DepthTargetTexture2D::new(
        &context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        DepthFormat::Depth32F,
    )
    .unwrap();

    // Render three frames
    for frame_index in 0..3 {
        // Create a render target (a combination of a color and a depth texture) to write into and clear the color and depth
        let pixels = RenderTarget::new(
            &context,
            ColorTarget::Texture2D {
                texture: &mut texture,
                mip_level: None,
            },
            DepthTarget::Texture2D {
                texture: &mut depth_texture,
            },
        )
        .unwrap()
        .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
        .unwrap()
        .write(|| {
            // Set the current transformation of the triangle
            model.set_transformation(Mat4::from_angle_y(radians(
                (frame_index as f32 * 0.6) as f32,
            )));

            // Render the triangle with the per vertex colors defined at construction
            model.render(&camera, &[])
        })
        .unwrap()
        .read_color()
        .unwrap();
        // Save the rendered image
        Saver::save_pixels(
            format!("headless-{}.png", frame_index),
            &pixels,
            viewport.width,
            viewport.height,
        )
        .unwrap();
    }
}
