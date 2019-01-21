
mod scene_objects;
mod window_handler;

use crate::window_handler::WindowHandler;
use dust::*;

fn main() {
    let mut window_handler = WindowHandler::new_default("Texture");
    let (width, height) = window_handler.size();
    let gl = window_handler.gl();

    // Renderer
    let renderer = pipeline::DeferredPipeline::new(&gl, width, height, false).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), width as f32 / height as f32, 0.1, 100.0);

    let positions = positions();
    let indices: Vec<u32> = (0..positions.len() as u32/3).collect();
    let mut textured_box = objects::ShadedMesh::create(&gl, &indices,&att!["position"=> (positions, 3),
                                                                    "normal"=> (normals(), 3)]).unwrap();
    textured_box.texture = Some(texture::Texture2D::new_from_file(&gl, "examples/assets/textures/test_texture.jpg").unwrap());

    let texture3d = texture::Texture3D::new_from_files(&gl, "examples/assets/textures/skybox_evening/",
                                                       "back.jpg", "front.jpg", "top.jpg", "left.jpg", "right.jpg").unwrap();
    let skybox = objects::Skybox::create(&gl, texture3d);

    let light = dust::light::DirectionalLight::new(vec3(0.0, -1.0, 0.0));

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    loop {
        window_handler.handle_events( |event| {
            WindowHandler::handle_window_close_events(event);
            WindowHandler::handle_camera_events(event, &mut camera_handler, &mut camera);
        });

        // draw
        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        let transformation = Mat4::identity();
        textured_box.render(&transformation, &camera);
        skybox.render(&camera).unwrap();

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_directional_light(&light).unwrap();

        window_handler.swap_buffers();
    };
}

fn positions() -> Vec<f32>
{
    vec![
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,

        -1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,

        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, -1.0,

        -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, -1.0
    ]
}

fn normals() -> Vec<f32>
{
    vec![
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,

        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,

        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,

        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,

        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,

        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0
    ]
}