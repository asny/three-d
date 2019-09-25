
use window::{event::*, Window};
use dust::*;

fn main() {
    let mut window = Window::new_default("Texture").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.0, 0.0, 0.0, 1.0)).unwrap();

    let box_mesh = Mesh::new_box(&gl).unwrap();
    let mut textured_box = objects::MeshShader::new(&gl).unwrap();
    textured_box.texture = Some(texture::Texture2D::new_from_bytes(&gl, include_bytes!("../assets/textures/test_texture.jpg")).unwrap());

    let texture3d = texture::Texture3D::new_from_bytes(&gl,
                                                       include_bytes!("../assets/textures/skybox_evening/back.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/front.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/top.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/left.jpg"),
                                                       include_bytes!("../assets/textures/skybox_evening/right.jpg")).unwrap();
    let skybox = objects::Skybox::new(&gl, texture3d);

    renderer.ambient_light().set_intensity(0.2);
    renderer.directional_light(0).unwrap().set_intensity(1.0);

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut renderer.camera);
        }

        // draw
        // Geometry pass
        renderer.geometry_pass(&|camera| {
            let transformation = Mat4::identity();
            textured_box.render(&box_mesh, &transformation, camera);
            skybox.render(camera).unwrap();
        }).unwrap();

        renderer.light_pass().unwrap();
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