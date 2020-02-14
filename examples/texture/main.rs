
use window::{event::*, Window};
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Texture").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.0, 0.0, 0.0, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);
    camera.enable_matrix_buffer(&gl);

    let mut box_mesh = Mesh::new_box(&gl).unwrap();
    box_mesh.texture = Some(texture::Texture2D::new_from_bytes(&gl, include_bytes!("../assets/textures/test_texture.jpg")).unwrap());

    let texture3d = texture::Texture3D::new_from_bytes(&gl,
                                                       include_bytes!("../assets/textures/skybox_evening/back.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/front.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/top.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/left.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/right.jpg")).unwrap();
    let skybox = objects::Skybox::new(&gl, texture3d);

    renderer.ambient_light().set_intensity(0.2);
    renderer.directional_light(0).unwrap().set_intensity(1.0);

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

        // draw
        // Geometry pass
        renderer.geometry_pass(&|| {
            let transformation = Mat4::identity();
            box_mesh.render(&transformation, &camera);
            skybox.render(&camera).unwrap();
        }).unwrap();

        renderer.light_pass(&camera).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}