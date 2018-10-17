use camera;
use gust::*;

pub fn rotate(camera: &mut camera::Camera, xrel: i32, yrel: i32)
{
    let x = -xrel as f32;
    let y = yrel as f32;
    let target = *camera.target();
    let direction = (target - *camera.position()).normalize();
    let mut up_direction = vec3(0., 1., 0.);
    let right_direction = direction.cross(&up_direction);
    up_direction = right_direction.cross(&direction);
    let mut camera_position = *camera.position();
    let zoom = camera_position.norm();
    camera_position = camera_position + (right_direction * x + up_direction * y) * 0.1;
    camera_position = camera_position.normalize() * zoom;
    camera.set_view(camera_position, target, up_direction);
}

pub fn zoom(camera: &mut camera::Camera, wheel:i32)
{
    let mut position = *camera.position();
    let target = *camera.target();
    let up = *camera.up();
    let mut zoom = (position - target).norm();
    zoom += wheel as f32;
    zoom = zoom.max(1.0);
    position = target + (position - target).normalize() * zoom;
    camera.set_view(position, target, up);
}