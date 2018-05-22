use glm;
use core::texture;
use std::rc::Rc;

pub struct ReflectingInput {
    pub model: glm::Matrix4<f32>,
    pub view: glm::Matrix4<f32>,
    pub projection: glm::Matrix4<f32>,
    pub camera_position: glm::Vec3
}


pub struct EmittingInput {
    pub camera_position: glm::Vec3,
    pub color_texture: Rc<texture::Texture2D>
}