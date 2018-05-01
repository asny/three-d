use glm;

pub struct DrawInput {
    pub model: glm::Matrix4<f32>,
    pub view: glm::Matrix4<f32>,
    pub projection: glm::Matrix4<f32>,
    pub camera_position: glm::Vec3
}