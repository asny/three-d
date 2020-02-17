
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};
    
    let mut window = Window::new_default("Wireframe").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let scene_center = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(scene_center + scene_radius * vec3(0.6, 0.6, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);
    camera.enable_matrix_buffer(&gl);

    // Objects
    let obj_file = include_str!("../assets/models/suzanne.obj").to_string();
    let mut wireframe = objects::ShadedEdges::new_from_obj_source(&gl, obj_file.clone(), 0.01, &scene_center);
    wireframe.diffuse_intensity = 0.8;
    wireframe.specular_intensity = 0.2;
    wireframe.specular_power = 5.0;
    wireframe.color = vec3(0.9, 0.2, 0.2);

    let mut model = Mesh::new_from_obj_source(&gl, obj_file).unwrap().pop().unwrap();
    model.diffuse_intensity = 0.2;
    model.specular_intensity = 0.4;
    model.specular_power = 20.0;

    let mut plane = Mesh::new_plane(&gl).unwrap();
    plane.diffuse_intensity = 0.2;
    plane.specular_intensity = 0.4;
    plane.specular_power = 20.0;

    let mut light = renderer.spot_light(0).unwrap();
    light.set_intensity(0.3);
    light.set_position(&vec3(5.0, 7.0, 5.0));
    light.set_direction(&vec3(-1.0, -1.0, -1.0));
    light.enable_shadows();

    light = renderer.spot_light(1).unwrap();
    light.set_intensity(0.3);
    light.set_position(&vec3(-5.0, 7.0, 5.0));
    light.set_direction(&vec3(1.0, -1.0, -1.0));
    light.enable_shadows();

    light = renderer.spot_light(2).unwrap();
    light.set_intensity(0.3);
    light.set_position(&vec3(-5.0, 7.0, -5.0));
    light.set_direction(&vec3(1.0, -1.0, 1.0));
    light.enable_shadows();

    light = renderer.spot_light(3).unwrap();
    light.set_intensity(0.3);
    light.set_position(&vec3(5.0, 7.0, -5.0));
    light.set_direction(&vec3(-1.0, -1.0, 1.0));
    light.enable_shadows();

    // Shadow pass
    renderer.shadow_pass(&|camera: &Camera| {
        model.render(&Mat4::from_translation(vec3(0.0, 2.0, 0.0)), camera);
        wireframe.render(camera);
    });

    // main loop
    let mut rotating = false;
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    rotating = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if rotating {
                        camera.rotate(delta.0 as f32, delta.1 as f32);
                    }
                },
                Event::MouseWheel {delta} => {
                    camera.zoom(*delta as f32);
                },
                _ => {}
            }
        }

        // Geometry pass
        renderer.geometry_pass(&|| {
            model.render(&Mat4::from_translation(vec3(0.0, 2.0, 0.0)), &camera);
            plane.render(&Mat4::from_scale(100.0), &camera);
            wireframe.render(&camera);
        }).unwrap();

        // Light pass
        renderer.light_pass(&camera).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}