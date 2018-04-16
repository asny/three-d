use gl;
use glm;
use dust::scene;

#[derive(Debug)]
pub enum Error {
}

pub struct Camera {
    gl: gl::Gl,
    position: glm::Vec3,
    direction: glm::Vec3,
    z_near: f32,
    z_far: f32,
    width: u32,
    height: u32,
}


impl Camera
{
    pub fn create(gl: &gl::Gl, position: glm::Vec3, direction: glm::Vec3, width: u32, height: u32) -> Result<Camera, Error>
    {
        let mut camera = Camera { gl: gl.clone(), position: position, direction: direction, z_near: 0.1, z_far: 1000.0, width: width, height: height };
        camera.set_screen_size(width, height);
        Ok(camera)
    }

    pub fn set_screen_size(&mut self, width: u32, height: u32)
    {
        unsafe {
            self.gl.Viewport(0, 0, width as i32, height as i32);
        }
        self.width = width;
        self.height = height;
    }

    pub fn set_view(&mut self, position: glm::Vec3, direction: glm::Vec3)
    {
        self.position = position;
        self.direction = direction;
    }

    pub fn draw(&self, scene: &scene::Scene)
    {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        scene.draw(&self.width, &self.height, &self.position, &self.get_view(), &self.get_projection());
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
