use three_d::*;

struct MandelbrotMaterial {}

impl Material for MandelbrotMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("mandelbrot.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {}
    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            ..Default::default()
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn id(&self) -> u16 {
        0b11u16
    }
}

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Mandelbrot!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    // Renderer
    let mut camera = Camera::new_orthographic(
        window.viewport(),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        2.5,
        0.0,
        10.0,
    );

    let mut mesh = Gm::new(
        Mesh::new(
            &context,
            &CpuMesh {
                positions: Positions::F32(vec![
                    vec3(-2.0, -2.0, 0.0),
                    vec3(2.0, -2.0, 0.0),
                    vec3(2.0, 2.0, 0.0),
                    vec3(2.0, 2.0, 0.0),
                    vec3(-2.0, 2.0, 0.0),
                    vec3(-2.0, -2.0, 0.0),
                ]),
                ..Default::default()
            },
        ),
        MandelbrotMaterial {},
    );
    mesh.set_transformation(Mat4::from_scale(10.0));

    // main loop
    window.render_loop(move |frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);

        for event in frame_input.events.iter() {
            match *event {
                Event::MouseMotion { delta, button, .. } => {
                    if button == Some(MouseButton::Left) {
                        let speed = 0.003 * camera.position().z.abs();
                        let right = camera.right_direction();
                        let up = right.cross(camera.view_direction());
                        let delta = -right * speed * delta.0 + up * speed * delta.1;
                        camera.translate(&delta);
                        redraw = true;
                    }
                }
                Event::MouseWheel {
                    delta, position, ..
                } => {
                    let distance = camera.position().z.abs();
                    let mut target = camera.position_at_pixel(position);
                    target.z = 0.0;
                    camera.zoom_towards(&target, distance * 0.05 * delta.1, 0.00001, 10.0);
                    redraw = true;
                }
                _ => {}
            }
        }

        if redraw {
            frame_input
                .screen()
                .clear(ClearState::color(0.0, 1.0, 1.0, 1.0))
                .render(&camera, &mesh, &[]);
        }

        FrameOutput {
            swap_buffers: redraw,
            wait_next_event: true,
            ..Default::default()
        }
    });
}
