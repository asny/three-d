use std::collections::HashMap;

use three_d::{renderer::*, WindowedContext};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

struct Scene {
    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
}

pub fn main() {
    let event_loop = EventLoop::new();

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];
    let colors = vec![
        Color::new(255, 0, 0, 255), // bottom right
        Color::new(0, 255, 0, 255), // bottom left
        Color::new(0, 0, 255, 255), // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let mut windows = HashMap::new();
    for _ in 0..2 {
        let window = Window::new(&event_loop).unwrap();
        let context =
            WindowedContext::from_winit_window(&window, three_d::SurfaceSettings::default())
                .unwrap();

        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            10.0,
        );

        let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
        model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

        let frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);
        windows.insert(
            window.id(),
            (
                window,
                context,
                frame_input_generator,
                Scene { camera, model },
            ),
        );
    }

    event_loop.run(move |event, _, control_flow| match &event {
        winit::event::Event::MainEventsCleared => {
            for (window, _, _, _) in windows.values() {
                window.request_redraw();
            }
        }
        winit::event::Event::RedrawRequested(_) => {
            for (window, context, frame_input_generator, scene) in windows.values_mut() {
                let frame_input = frame_input_generator.generate(context);

                scene.camera.set_viewport(frame_input.viewport);
                scene.model.animate(frame_input.accumulated_time as f32);
                frame_input
                    .screen()
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .render(&scene.camera, &scene.model, &[]);

                context.swap_buffers().unwrap();
                control_flow.set_poll();
                window.request_redraw();
            }
        }
        winit::event::Event::WindowEvent { event, window_id } => {
            if let Some((_, context, frame_input_generator, _)) = windows.get_mut(window_id) {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        context.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested => {
                        windows.remove(window_id);

                        if windows.is_empty() {
                            control_flow.set_exit();
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => {}
    });
}