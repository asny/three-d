use gl;
use std;
use state;

#[derive(Debug)]
pub enum Error {

}

pub trait Rendertarget {
    fn bind(&self);
    fn clear(&self);
}

pub struct ScreenRendertarget {
    gl: gl::Gl,
    id: u32,
    width: usize,
    height: usize
}


impl ScreenRendertarget
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<ScreenRendertarget, Error>
    {
        /*let mut id: u32 = 0;
        unsafe {
            gl.GenFramebuffers(1, &mut id);
        }*/
        Ok(ScreenRendertarget { gl: gl.clone(), id: 0, width, height })
    }
}

impl Rendertarget for ScreenRendertarget
{
    fn bind(&self)
    {
        bind(&self.gl, self.id, self.width, self.height);
    }

    fn clear(&self)
    {
        clear(&self.gl);
    }
}



/*impl Drop for Rendertarget {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteFramebuffers(1, &self.id);
        }
    }
}*/

fn bind(gl: &gl::Gl, id: u32, width: usize, height: usize)
{
    unsafe {
        static mut CURRENTLY_USED: u32 = std::u32::MAX;
        if id != CURRENTLY_USED
        {
            gl.BindFramebuffer(gl::FRAMEBUFFER, id);
            gl.Viewport(0, 0, width as i32, height as i32);
            CURRENTLY_USED = id;
        }
    }
}

fn clear(gl: &gl::Gl)
{
    state::depth_write(gl,true);
    unsafe {
        gl.ClearColor(0.0, 0.0, 0.0, 0.0);
        gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

}