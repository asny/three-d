use three_d::*;

fn main() {
    let viewport = Viewport::new_at_origo(1280, 720);

    // Create a headless graphics context
    let context = HeadlessContext::new().unwrap();

    // Create a camera
    let camera = Camera::new_perspective(
        viewport,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10.0,
    );

    // Create the scene - a single colored triangle
    let mut model = Gm::new(
        Mesh::new(
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
        ),
        ColorMaterial::default(),
    );

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
    );

    // Also create a depth texture to support depth testing
    let mut depth_texture = DepthTexture2D::new::<f32>(
        &context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );

    // Render three frames
    for frame_index in 0..3 {
        // Set the current transformation of the triangle
        model.set_transformation(Mat4::from_angle_y(radians(
            (frame_index as f32 * 0.6) as f32,
        )));

        // Create a render target (a combination of a color and a depth texture) to write into
        let pixels = RenderTarget::new(
            texture.as_color_target(None),
            depth_texture.as_depth_target(),
        )
        // Clear color and depth of the render target
        .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
        // Render the triangle with the per vertex colors defined at construction
        .render(&camera, &model, &[])
        // Read out the colors from the render target
        .read_color();

        // Save the rendered image
        use three_d_asset::io::Serialize;

        three_d_asset::io::save(
            &CpuTexture {
                data: TextureData::RgbaU8(pixels),
                width: texture.width(),
                height: texture.height(),
                ..Default::default()
            }
            .serialize(format!("headless-{}.png", frame_index))
            .unwrap(),
        )
        .unwrap();
    }
}
