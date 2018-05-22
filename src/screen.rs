
use glm;
use gl;
use core::rendertarget;
use core::rendertarget::Rendertarget;

#[derive(Debug)]
pub enum Error {
    Rendertarget(rendertarget::Error)
}

impl From<rendertarget::Error> for Error {
    fn from(other: rendertarget::Error) -> Self {
        Error::Rendertarget(other)
    }
}

pub struct Screen {
    gl: gl::Gl,
    pub width: usize,
    pub height: usize,
    pub rendertarget: rendertarget::ScreenRendertarget
}

impl Screen {
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<Screen, Error>
    {
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        Ok(Screen {gl: gl.clone(), width, height, rendertarget})
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height)?;
        self.width = width;
        self.height = height;
        Ok(())
    }
}