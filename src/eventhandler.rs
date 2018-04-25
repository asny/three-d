use camera;
use glm;


#[derive(Debug)]
pub enum Error {

}

pub fn navigate_spherical(camera: &mut camera::Camera, xrel: i32, yrel: i32)
{
    let x = -xrel as f32;
    let y = yrel as f32;
    let mut direction = camera.direction();
    let mut up_direction = glm::vec3(0., 1., 0.);
    let right_direction = glm::cross(direction, up_direction);
    up_direction = glm::cross(right_direction, direction);
    let mut camera_position = camera.position();
    let zoom = glm::length(camera_position);
    camera_position = camera_position + (right_direction * x + up_direction * y) * 0.1;
    camera_position = glm::normalize(camera_position) * zoom;
    camera.set_view(camera_position, glm::vec3(0.0, 0.0, 0.0));
}