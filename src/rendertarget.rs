use gl;
use std;
use state;

#[derive(Debug)]
pub enum Error {

}

pub struct Rendertarget {
    gl: gl::Gl,
    id: u32,
    width: usize,
    height: usize
}

impl Rendertarget
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<Rendertarget, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            //gl.GenFramebuffers(1, &mut id);
        }
        Ok(Rendertarget{ gl: gl.clone(), id, width, height })
    }

    pub fn bind(&self)
    {
        unsafe {
            static mut CURRENTLY_USED: u32 = std::u32::MAX;
            if self.id != CURRENTLY_USED
            {
                self.gl.BindFramebuffer(gl::FRAMEBUFFER, self.id);
                self.gl.Viewport(0, 0, self.width as i32, self.height as i32);
                CURRENTLY_USED = self.id;
            }
        }
    }

    pub fn clear(&self)
    {
        state::depth_write(&self.gl,true);
        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

    }
}

impl Drop for Rendertarget {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteFramebuffers(1, &self.id);
        }
    }
}