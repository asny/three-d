use glm::*;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    z_near: f32,
    z_far: f32,
    pub width: usize,
    pub height: usize
}


impl Camera
{
    pub fn create(position: Vec3, target: Vec3, width: usize, height: usize) -> Camera
    {
        Camera { position, target, z_near: 0.1, z_far: 1000.0, width, height }
    }

    pub fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.position = position;
        self.target = target;
    }

    pub fn direction(&self) -> Vec3
    {
        normalize(self.target - self.position)
    }

    pub fn get_view(&self) -> Matrix4<f32>
    {
        ext::look_at(self.position, self.target, vec3(0., 1., 0.))
    }

    pub fn get_projection(&self) -> Matrix4<f32>
    {
        ext::perspective(radians(45.0), (self.width as f32)/(self.height as f32), self.z_near, self.z_far)
    }
}
