use three_d::*;

struct MandelbrotMaterial {}

impl Material for MandelbrotMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("mandelbrot.frag").to_string()
    }

    fn use_uniforms(&self, _program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {}
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

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId(0b11u16)
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
        20.0,
    );
    let mut control = Control2D::new(0.5, 10000.0);

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
    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= control.handle_events(
            &mut camera,
            &mut frame_input.events,
            frame_input.device_pixel_ratio,
        );

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
