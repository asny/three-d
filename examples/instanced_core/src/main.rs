use three_d::core::{
    degrees, radians, vec3, Camera, ClearState, Color, Context, ElementBuffer, InstanceBuffer,
    Mat4, Program, RenderStates, VertexBuffer,
};
use three_d::window::{FrameOutput, Window, WindowSettings};

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Core Instances!".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();

    // Define a + shape
    let vertices = VertexBuffer::new_with_data(
        &context,
        &[
            vec3(-0.1, 0.4, 0.0),
            vec3(-0.1, -0.4, 0.0),
            vec3(0.1, -0.4, 0.0),
            vec3(0.1, 0.4, 0.0),
            vec3(-0.4, -0.1, 0.0),
            vec3(0.4, -0.1, 0.0),
            vec3(-0.4, 0.1, 0.0),
            vec3(0.4, 0.1, 0.0),
        ],
    );
    let indices = ElementBuffer::new_with_data(&context, &[0u16, 1, 2, 0, 2, 3, 4, 5, 6, 6, 5, 7]);

    // Define 5 + instances
    let translations = vec![
        Mat4::from_translation(vec3(-0.5, 0.0, 0.0)),
        Mat4::from_translation(vec3(-0.25, 0.0, 0.1)),
        Mat4::from_translation(vec3(0.0, 0.0, 0.2)),
        Mat4::from_translation(vec3(0.25, 0.0, 0.3)),
        Mat4::from_translation(vec3(0.5, 0.0, 0.4)),
    ];
    // Create a mutable buffer which will hold frame-by-frame instance matrices
    let mut instances = translations.clone();
    let mut instance_buffer = InstanceBuffer::new_with_data(&context, &instances[..]);
    let instance_count = instances.len();
    let colors = InstanceBuffer::new_with_data(
        &context,
        &[
            Color::new_opaque(107, 144, 128),
            Color::new_opaque(164, 195, 178),
            Color::new_opaque(204, 227, 222),
            Color::new_opaque(234, 244, 244),
            Color::new_opaque(246, 255, 248),
        ],
    );

    let program = Program::from_source(
        &context,
        include_str!("cross.vert"),
        include_str!("cross.frag"),
    )
    .unwrap();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        frame_input
            .screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .write(|| {
                let time = frame_input.accumulated_time as f32;
                for i in 0..instance_count {
                    instances[i] = translations[i]
                        * Mat4::from_angle_z(radians((i as f32 + 1.0) * 0.00005 * time));
                }
                instance_buffer.fill(&instances[..]);

                program.use_uniform("viewProjection", camera.projection() * camera.view());
                program.use_vertex_attribute("position", &vertices);
                program.use_instance_attribute("instance", &instance_buffer);
                program.use_instance_attribute("color", &colors);
                program.draw_elements_instanced(
                    RenderStates::default(),
                    frame_input.viewport,
                    &indices,
                    instance_count as u32,
                );
            });

        FrameOutput::default()
    });
}
