use gl;
use glm;

#[derive(Debug)]
pub enum Error {
}

pub struct Camera {
    position: glm::Vec3
}


impl Camera
{
    pub fn create(gl: &gl::Gl, position: glm::Vec3) -> Result<Camera, Error>
    {

        Ok(Camera { position })
    }
}
