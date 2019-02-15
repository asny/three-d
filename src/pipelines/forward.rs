
use gl;
use crate::core::rendertarget;
use crate::core::rendertarget::Rendertarget;
use crate::pipelines::Error;

pub struct ForwardPipeline {
    gl: gl::Gl,
    rendertarget: rendertarget::ScreenRendertarget
}

impl ForwardPipeline
{
    pub fn new(gl: &gl::Gl, screen_width: usize, screen_height: usize) -> Result<ForwardPipeline, Error>
    {
        let rendertarget = rendertarget::ScreenRendertarget::new(gl, screen_width, screen_height)?;
        Ok(ForwardPipeline {gl: gl.clone(), rendertarget})
    }

    pub fn resize(&mut self, screen_width: usize, screen_height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::new(&self.gl, screen_width, screen_height)?;
        Ok(())
    }

    pub fn render_pass_begin(&self)
    {
        self.rendertarget.bind();
        self.rendertarget.clear();
    }
}