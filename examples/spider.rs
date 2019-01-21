
mod scene_objects;
mod window_handler;

use std::time::Instant;
use dust::*;

fn main() {
    let mut window_handler = window_handler::WindowHandler::new_default("Hello, world!");
    let (width, height) = window_handler.size();
    let gl = window_handler.gl();

    // Renderer
    let renderer = pipeline::DeferredPipeline::new(&gl, width, height, true).unwrap();

    // Models
    let mut environment = scene_objects::environment::Environment::create(&gl);
    let mut spider = scene_objects::spider::Spider::create(&gl, vec3(0.0, 0.0, 5.0), vec3(0.0, 0.0, -1.0));

    // Camera
    let mut camera = camera::PerspectiveCamera::new(spider.get_position(&environment),
                                                    spider.get_position(&environment) + spider.get_view_direction(&environment),
                                                    spider.get_up_direction(), degrees(45.0),
                                                    width as f32 / height as f32, 0.1, 1000.0);

    // Lights
    let mut ambient_light = crate::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut directional_light = crate::light::DirectionalLight::new(vec3(0.0, -1.0, 0.0));
    directional_light.enable_shadows(&gl, 10.0, 10.0).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::FIRST);
    let mut now = Instant::now();
    let mut time = 0.0;

    // main loop
    loop {
        window_handler.handle_events( |event| {
            window_handler::WindowHandler::handle_window_close_events(event);
            window_handler::WindowHandler::handle_camera_events(event, &mut camera_handler, &mut camera);
            use glutin::*;
            match event {
                Event::WindowEvent{ event, .. } => match event {
                    WindowEvent::KeyboardInput {input, ..} => {
                        if let Some(keycode) = input.virtual_keycode
                        {
                            match keycode {
                                VirtualKeyCode::W => {
                                    spider.is_moving_forward = input.state == ElementState::Pressed;

                                },
                                VirtualKeyCode::D => {
                                    spider.is_rotating_right = input.state == ElementState::Pressed;

                                },
                                VirtualKeyCode::A => {
                                    spider.is_rotating_left = input.state == ElementState::Pressed;

                                },
                                VirtualKeyCode::S => {
                                    spider.is_moving_backward = input.state == ElementState::Pressed;

                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        });

        let new_now = Instant::now();
        let elapsed_time = 0.000000001 * new_now.duration_since(now).subsec_nanos() as f32;
        now = new_now;
        time += elapsed_time;

        // Update
        spider.update(elapsed_time, &environment);
        let spider_pos = spider.get_position(&environment);
        camera_handler.translate(&mut camera, &spider_pos, &spider.get_view_direction(&environment), &spider.get_up_direction());
        environment.set_position(&spider_pos);

        // Shadow pass
        directional_light.set_target(&spider_pos);
        directional_light.shadow_cast_begin().unwrap();
        //environment.render_opague(directional_light.shadow_camera().unwrap()).unwrap();
        spider.render(directional_light.shadow_camera().unwrap());

        // Draw
        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        environment.render_opague(&camera);
        spider.render(&camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();
        renderer.copy_to_screen().unwrap();

        // After effects
        environment.render_transparent(time, &camera, width, height, renderer.geometry_pass_color_texture(), renderer.geometry_pass_position_texture());

        window_handler.swap_buffers();
    };
}
