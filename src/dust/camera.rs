use gl;
use glm;
use glm::ext::*;

#[derive(Debug)]
pub enum Error {
}

pub struct Camera {
    position: glm::Vec3,
    direction: glm::Vec3,
    z_near: f32,
    z_far: f32,
    width: i16,
    height: i16,
}


impl Camera
{
    pub fn create(position: glm::Vec3, direction: glm::Vec3) -> Result<Camera, Error>
    {
        Ok(Camera { position: position, direction: direction, z_near: 0.1, z_far: 1000.0, width: 1024, height: 1024 })
    }

    pub fn set_screen_size(&mut self, width: i16, height: i16)
    {
        self.width = width;
        self.height = height;
    }

    pub fn set_view(&mut self, position: glm::Vec3, direction: glm::Vec3)
    {
        self.position = position;
        self.direction = direction;
    }

    fn get_view(&self) -> glm::Matrix4<f32>
    {
        glm::ext::look_at(self.position, self.position + self.direction, glm::vec3(0., 1., 0.))
    }

    fn get_projection(&self) -> glm::Matrix4<f32>
    {
        glm::ext::perspective(glm::radians(45.0), (self.width as f32)/(self.height as f32), self.z_near, self.z_far)
    }
}
