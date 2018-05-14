use gl;
use glm;
use scene;
use input;

pub struct Camera {
    pub position: glm::Vec3,
    pub target: glm::Vec3,
    z_near: f32,
    z_far: f32,
    width: usize,
    height: usize
}


impl Camera
{
    pub fn create(position: glm::Vec3, target: glm::Vec3, width: usize, height: usize) -> Camera
    {
        Camera { position, target, z_near: 0.1, z_far: 1000.0, width, height }
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize)
    {
        self.width = width;
        self.height = height;
    }

    pub fn set_view(&mut self, position: glm::Vec3, target: glm::Vec3)
    {
        self.position = position;
        self.target = target;
    }

    pub fn direction(&self) -> glm::Vec3
    {
        glm::normalize(self.target - self.position)
    }

    pub fn get_view(&self) -> glm::Matrix4<f32>
    {
        glm::ext::look_at(self.position, self.target, glm::vec3(0., 1., 0.))
    }

    pub fn get_projection(&self) -> glm::Matrix4<f32>
    {
        glm::ext::perspective(glm::radians(45.0), (self.width as f32)/(self.height as f32), self.z_near, self.z_far)
    }
}
