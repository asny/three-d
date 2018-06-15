use camera;
use glm;

pub fn rotate(camera: &mut camera::Camera, xrel: i32, yrel: i32)
{
    let x = -xrel as f32;
    let y = yrel as f32;
    let direction = camera.direction();
    let mut up_direction = glm::vec3(0., 1., 0.);
    let right_direction = glm::cross(direction, up_direction);
    up_direction = glm::cross(right_direction, direction);
    let mut camera_position = camera.position;
    let zoom = glm::length(camera_position);
    camera_position = camera_position + (right_direction * x + up_direction * y) * 0.1;
    camera_position = glm::normalize(camera_position) * zoom;
    let target = camera.target;
    camera.set_view(camera_position, target);
}

pub fn zoom(camera: &mut camera::Camera, wheel:i32)
{
    let mut position = camera.position;
    let target = camera.target;
    let mut zoom = glm::length(position);
    zoom += wheel as f32;
    zoom = zoom.max(1.0);
    position = glm::normalize(position - target) * zoom;
    camera.set_view(position, target);
}