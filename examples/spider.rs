
mod scene_objects;

use dust::*;
use window::{event::*, Window};

fn main() {
    let mut window = Window::new_default("Spider").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let renderer = DeferredPipeline::new(&gl, width, height, vec4(1.0, 1.0, 1.0, 1.0)).unwrap();
    let light_pass_rendertarget = rendertarget::ColorRendertarget::new(&gl, width, height, 1).unwrap();
    let copy_effect = effects::CopyEffect::new(&gl).unwrap();

    // Models
    let mut environment = scene_objects::environment::Environment::new(&gl);
    let mut spider = scene_objects::spider::Spider::new(&gl, vec3(0.0, 0.0, 5.0), vec3(0.0, 0.0, -1.0));

    // Camera
    let mut camera = camera::PerspectiveCamera::new(spider.get_position(&environment),
                                                    spider.get_position(&environment) + spider.get_view_direction(&environment),
                                                    spider.get_up_direction(), degrees(45.0),
                                                    width as f32 / height as f32, 0.1, 1000.0);

    // Lights
    let ambient_light = light::AmbientLight::new();

    let mut directional_light = light::DirectionalLight::new(vec3(0.0, -1.0, 0.0));
    directional_light.enable_shadows(&gl, 10.0, 10.0).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::FIRST);
    let mut time = 0.0;

    // Effects
    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

    // main loop
    window.render_loop(move |events, elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut camera);
            match event {
                Event::Key { state, kind } => {
                    if kind == "W"
                    {
                        spider.is_moving_forward = *state == State::Pressed;
                    }
                    if kind == "D"
                    {
                        spider.is_rotating_right = *state == State::Pressed;
                    }
                    if kind == "A"
                    {
                        spider.is_rotating_left = *state == State::Pressed;
                    }
                    if kind == "S"
                    {
                        spider.is_moving_backward = *state == State::Pressed;
                    }
                    if kind == "R" && *state == State::Pressed
                    {
                        debug_effect.change_type();
                    }
                },
                _ => {}
            }
        }
        time += elapsed_time;

        // Update
        spider.update(elapsed_time as f32, &environment);
        let spider_pos = spider.get_position(&environment);
        camera_handler.translate(&mut camera, &spider_pos, &spider.get_view_direction(&environment), &spider.get_up_direction());
        environment.set_position(&spider_pos);

        // Shadow pass
        directional_light.set_target(&spider_pos);
        directional_light.shadow_cast_begin().unwrap();
        //environment.render_opague(directional_light.shadow_camera().unwrap()).unwrap();
        //spider.render(directional_light.shadow_camera().unwrap());

        // Draw
        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        environment.render_opague(&camera);
        //spider.render(&camera);

        // Light pass
        renderer.light_pass_render_to(&camera, &light_pass_rendertarget).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();

        renderer.screen_rendertarget().bind();
        renderer.screen_rendertarget().clear_depth();
        copy_effect.apply(renderer.full_screen(), &light_pass_rendertarget.targets[0], renderer.geometry_pass_depth_texture()).unwrap();

        // After effects
        environment.render_transparent(renderer.full_screen(), time as f32, &camera, width, height, renderer.geometry_pass_color_texture(), renderer.geometry_pass_position_texture());

        debug_effect.apply(renderer.full_screen(), renderer.geometry_pass_color_texture(), renderer.geometry_pass_position_texture(), renderer.geometry_pass_normal_texture(), renderer.geometry_pass_depth_texture()).unwrap();
    }).unwrap();
}

pub fn handle_camera_events(event: &Event, camera_handler: &mut dust::camerahandler::CameraHandler, camera: &mut Camera)
{
    match event {
        Event::Key {state, kind} => {
            if kind == "Tab" && *state == State::Pressed
            {
                camera_handler.next_state();
            }
        },
        Event::MouseClick {state, button, ..} => {
            if *button == MouseButton::Left
            {
                if *state == State::Pressed { camera_handler.start_rotation(); }
                else { camera_handler.end_rotation() }
            }
        },
        Event::MouseMotion {delta} => {
            camera_handler.rotate(camera, delta.0 as f32, delta.1 as f32);
        },
        Event::MouseWheel {delta} => {
            camera_handler.zoom(camera, *delta as f32);
        }
    }
}
