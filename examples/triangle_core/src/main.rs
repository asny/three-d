use three_d::core::{
    degrees, radians, vec3, ClearState, Context, Mat4, Program, RenderStates, Srgba, VertexBuffer,
};
use three_d::window::{FrameOutput, Window, WindowSettings};
use three_d::CoreError;
use three_d_asset::Camera;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Core Triangle!".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();

    // Define triangle vertices and colors
    let positions = VertexBuffer::new_with_data(
        &context,
        &[
            vec3(0.5, -0.5, 0.0),  // bottom right
            vec3(-0.5, -0.5, 0.0), // bottom left
            vec3(0.0, 0.5, 0.0),   // top
        ],
    );
    let colors = VertexBuffer::new_with_data(
        &context,
        &[
            Srgba::RED.to_linear_srgb(),   // bottom right
            Srgba::GREEN.to_linear_srgb(), // bottom left
            Srgba::BLUE.to_linear_srgb(),  // top
        ],
    );

    let program = Program::from_source(
        &context,
        include_str!("triangle.vert"),
        include_str!("triangle.frag"),
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
            .write::<CoreError>(|| {
                let time = frame_input.accumulated_time as f32;
                program.use_uniform("model", Mat4::from_angle_y(radians(time * 0.005)));
                program.use_uniform("viewProjection", camera.projection() * camera.view());
                program.use_vertex_attribute("position", &positions);
                program.use_vertex_attribute("color", &colors);
                program.draw_arrays(
                    RenderStates::default(),
                    frame_input.viewport,
                    positions.vertex_count(),
                );
                Ok(())
            })
            .unwrap();

        FrameOutput::default()
    });
}
