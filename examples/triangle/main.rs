use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Create a window (a canvas on web)
    let window = Window::new("Triangle", Some((1280, 720))).unwrap();

    // Get the graphics context from the window
    let context = window.gl().unwrap();

    // Create a camera
    let mut camera = Camera::new_perspective(
        &context,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        window.viewport().unwrap().aspect(),
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

    // Construct a mesh, thereby transferring the mesh data to the GPU
    let mesh = Mesh::new(&context, &cpu_mesh).unwrap();

    // Start the main render loop
    window.render_loop(move |frame_input: FrameInput| // Begin a new frame with an updated frame input
    {
        // Ensure the aspect ratio of the camera matches the aspect ratio of the window viewport
        camera.set_aspect(frame_input.viewport.aspect()).unwrap();

        // Start writing to the screen and clears the color and depth
        Screen::write(&context, &ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0), || {
            // Compute the current rotation of the triangle
            let transformation = Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.005) as f32));

            // Render the triangle with the per vertex colors defined at construction
            mesh.render_color(RenderStates::default(), frame_input.viewport, &transformation, &camera)?;
            Ok(())
        }).unwrap();

        if args.len() > 1 {
            // To automatically generate screenshots of the examples, can safely be ignored.
            FrameOutput {screenshot: Some(args[1].clone().into()), exit: true, ..Default::default()}
        } else {
            // Returns default frame output to end the frame
            FrameOutput::default()
        }
    }).unwrap();
}
