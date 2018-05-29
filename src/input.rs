use glm;

pub struct ReflectingInput {
    pub model: glm::Matrix4<f32>,
    pub view: glm::Matrix4<f32>,
    pub projection: glm::Matrix4<f32>,
    pub normal: glm::Matrix4<f32>,
    pub camera_position: glm::Vec3
}