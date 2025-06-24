use std::collections::HashMap;

use three_d::{renderer::*, WindowedContext};

struct Scene {
    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
}

pub fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];
    let colors = vec![
        Srgba::new(255, 0, 0, 255), // bottom right
        Srgba::new(0, 255, 0, 255), // bottom left
        Srgba::new(0, 0, 255, 255), // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let mut windows = HashMap::new();
    for i in 0..2 {
        #[cfg(not(target_arch = "wasm32"))]
        let window_builder = winit::window::Window::default_attributes()
            .with_title("winit window")
            .with_min_inner_size(winit::dpi::LogicalSize::new(720, 720))
            .with_inner_size(winit::dpi::LogicalSize::new(720, 720))
            .with_position(winit::dpi::LogicalPosition::new(300 * i, 100));
        #[cfg(target_arch = "wasm32")]
        let window_builder = {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;
            window::Window::default_attributes()
                .with_canvas(Some(
                    web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("canvas")
                        .item(i)
                        .unwrap()
                        .dyn_into::<web_sys::HtmlCanvasElement>()
                        .unwrap(),
                ))
                .with_inner_size(winit::dpi::LogicalSize::new(720, 720))
                .with_prevent_default(true)
        };
        let window = event_loop.create_window(window_builder).unwrap();
        let context = WindowedContext::from_winit_window(
            &window,
            three_d::SurfaceSettings {
                vsync: false, // Wayland hangs in swap_buffers when one window is minimized or occluded
                ..three_d::SurfaceSettings::default()
            },
        )
        .unwrap();

        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 0.0, 2.0 + i as f32 * 4.0),
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

    _ = event_loop.run(move |event, event_loop| match &event {
        winit::event::Event::AboutToWait => {
            for (window, _, _, _) in windows.values() {
                window.request_redraw();
            }
        }
        winit::event::Event::WindowEvent { event, window_id } => {
            if let Some((_, context, frame_input_generator, _)) = windows.get_mut(window_id) {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        if let Some((window, context, frame_input_generator, scene)) =
                            windows.get_mut(window_id)
                        {
                            context.make_current().unwrap();
                            let frame_input = frame_input_generator.generate(context);

                            scene.camera.set_viewport(frame_input.viewport);
                            scene.model.animate(frame_input.accumulated_time as f32);
                            frame_input
                                .screen()
                                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                                .render(&scene.camera, &scene.model, &[]);

                            context.swap_buffers().unwrap();
                            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
                            window.request_redraw();
                        }
                    }
                    // winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //     context.resize(**new_inner_size);
                    // }
                    winit::event::WindowEvent::CloseRequested => {
                        if let Some((_, context, _, _)) = windows.get_mut(window_id) {
                            context.make_current().unwrap();
                        }

                        windows.remove(window_id);

                        if windows.is_empty() {
                            event_loop.exit();
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => {}
    });
}
