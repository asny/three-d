use three_d::core::*;
use three_d::renderer::*;
use three_d::window::*;

struct MandelbrotMaterial {}

impl Material for MandelbrotMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("../../assets/shaders/mandelbrot.frag").to_string()
    }
    fn use_uniforms(
        &self,
        _program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            ..Default::default()
        }
    }
    fn is_transparent(&self) -> bool {
        false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "Mandelbrot!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    // Renderer
    let mut camera = Camera::new_orthographic(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        2.5,
        0.0,
        10.0,
    )
    .unwrap();

    let indices = vec![0u8, 1, 2, 2, 3, 0];
    let positions = vec![
        -2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0, 2.0, 0.0, -2.0, 2.0, 0.0,
    ];
    let mut mesh = Model::new_with_material(
        &context,
        &CpuMesh {
            indices: Some(Indices::U8(indices)),
            positions,
            ..Default::default()
        },
        MandelbrotMaterial {},
    )
    .unwrap();
    mesh.set_transformation(Mat4::from_scale(10.0));

    // main loop
    window
        .render_loop(move |frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseMotion { delta, button, .. } => {
                        if *button == Some(MouseButton::Left) {
                            let speed = 0.003 * camera.position().z.abs();
                            let right = camera.right_direction();
                            let up = right.cross(camera.view_direction());
                            let delta =
                                -right * speed * delta.0 as f32 + up * speed * delta.1 as f32;
                            camera.translate(&delta).unwrap();
                            redraw = true;
                        }
                    }
                    Event::MouseWheel {
                        delta, position, ..
                    } => {
                        let distance = camera.position().z.abs();
                        let pixel = (
                            (frame_input.device_pixel_ratio * position.0) as f32,
                            (frame_input.device_pixel_ratio * position.1) as f32,
                        );
                        let mut target = camera.position_at_pixel(pixel);
                        target.z = 0.0;
                        camera
                            .zoom_towards(&target, distance * 0.05 * delta.1 as f32, 0.00001, 10.0)
                            .unwrap();
                        redraw = true;
                    }
                    _ => {}
                }
            }

            if redraw {
                Screen::write(&context, ClearState::color(0.0, 1.0, 1.0, 1.0), || {
                    mesh.render(&camera, &[])
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
