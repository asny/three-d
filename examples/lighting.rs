
use window::event::*;
use window::Window;
use dust::*;

fn main() {
    let mut window = Window::new_default("Hello, world!");
    let (width, height) = window.size();
    let gl = window.gl();

    // Renderer
    let renderer = pipeline::DeferredPipeline::new(&gl, width, height, true).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let monkey_file = include_str!("assets/models/suzanne.obj").to_string();
    let monkey_objs = wavefront_obj::obj::parse(monkey_file).unwrap();
    let monkey_obj = monkey_objs.objects.first().unwrap();

    let mut positions = Vec::new();
    monkey_obj.vertices.iter().for_each(|v| {positions.push(v.x as f32); positions.push(v.y as f32); positions.push(v.z as f32);});
    let mut normals = Vec::new();
    monkey_obj.normals.iter().for_each(|v| {normals.push(v.x as f32); normals.push(v.y as f32); normals.push(v.z as f32);});
    let mut indices = Vec::new();
    for shape in monkey_obj.geometry.first().unwrap().shapes.iter() {
        match shape.primitive {
            wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                indices.push(i0.0 as u32);
                indices.push(i1.0 as u32);
                indices.push(i2.0 as u32);
            },
            _ => {}
        }
    }
    let mut monkey = objects::ShadedMesh::create(&gl, &indices, &att!["position" => (positions, 3), "normal" => (normals, 3)]).unwrap();

    let plane_positions: Vec<f32> = vec![
        -1.0, 0.0, -1.0,
        1.0, 0.0, -1.0,
        1.0, 0.0, 1.0,
        -1.0, 0.0, 1.0
    ];
    let plane_normals: Vec<f32> = vec![
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0
    ];
    let plane_indices: Vec<u32> = vec![
        0, 2, 1,
        0, 3, 2,
    ];
    let plane = crate::objects::ShadedMesh::create(&gl, &plane_indices, &att!["position" => (plane_positions, 3), "normal" => (plane_normals, 3)]).unwrap();

    let mut ambient_light = crate::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut directional_light = dust::light::DirectionalLight::new(vec3(1.0, -1.0, -1.0));
    directional_light.base.color = vec3(1.0, 0.0, 0.0);
    directional_light.enable_shadows(&gl, 2.0, 10.0).unwrap();

    let mut point_light = dust::light::PointLight::new(vec3(0.0, 5.0, 5.0));
    point_light.base.color = vec3(0.0, 1.0, 0.0);

    let mut spot_light = dust::light::SpotLight::new(vec3(5.0, 5.0, 5.0), vec3(-1.0, -1.0, -1.0));
    spot_light.base.color = vec3(0.0, 0.0, 1.0);
    spot_light.enable_shadows(&gl, 20.0).unwrap();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut camera);
            //handle_ambient_light_parameters(event, &mut ambient_light);
            //handle_directional_light_parameters(event, &mut directional_light);
            //handle_surface_parameters(event, &mut monkey);
        }

        // Draw
        let render_scene = |camera: &Camera| {
            monkey.render(&Mat4::identity(), camera);
        };

        // Shadow pass
        directional_light.shadow_cast_begin().unwrap();
        render_scene(directional_light.shadow_camera().unwrap());

        spot_light.shadow_cast_begin().unwrap();
        render_scene(spot_light.shadow_camera().unwrap());

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);
        plane.render(&(Mat4::from_translation(vec3(0.0, -1.0, 0.0)) * Mat4::from_scale(10.0)), &camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&directional_light).unwrap();
        renderer.shine_point_light(&point_light).unwrap();
        renderer.shine_spot_light(&spot_light).unwrap();

        renderer.copy_to_screen().unwrap();
    });
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
        Event::MouseClick {state, button} => {
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

/*fn handle_ambient_light_parameters(event: &Event, light: &mut light::AmbientLight)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::X => {
                            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
                            println!("Ambient light intensity: {}", light.base.intensity);
                        },
                        VirtualKeyCode::Z => {
                            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
                            println!("Ambient light intensity: {}", light.base.intensity);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}

fn handle_directional_light_parameters(event: &Event, light: &mut light::DirectionalLight)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::V => {
                            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
                            println!("Directional light intensity: {}", light.base.intensity);
                        },
                        VirtualKeyCode::C => {
                            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
                            println!("Directional light intensity: {}", light.base.intensity);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}

fn handle_surface_parameters(event: &Event, surface: &mut crate::objects::ShadedMesh)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::S => {
                            surface.diffuse_intensity = (surface.diffuse_intensity + 0.1).min(1.0);
                            println!("Diffuse intensity: {}", surface.diffuse_intensity);
                        },
                        VirtualKeyCode::A => {
                            surface.diffuse_intensity = (surface.diffuse_intensity - 0.1).max(0.0);
                            println!("Diffuse intensity: {}", surface.diffuse_intensity);
                        },
                        VirtualKeyCode::F => {
                            surface.specular_intensity = (surface.specular_intensity + 0.1).min(1.0);
                            println!("Specular intensity: {}", surface.specular_intensity);
                        },
                        VirtualKeyCode::D => {
                            surface.specular_intensity = (surface.specular_intensity - 0.1).max(0.0);
                            println!("Specular intensity: {}", surface.specular_intensity);
                        },
                        VirtualKeyCode::H => {
                            surface.specular_power = surface.specular_power + 1.0;
                            println!("Specular power: {}", surface.specular_power);
                        },
                        VirtualKeyCode::G => {
                            surface.specular_power = (surface.specular_power - 1.0).max(0.0);
                            println!("Specular power: {}", surface.specular_power);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}*/