use gl;
use std;
use core::state;
use core::texture;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    Texture(texture::Error)
}

impl From<texture::Error> for Error {
    fn from(other: texture::Error) -> Self {
        Error::Texture(other)
    }
}

pub trait Rendertarget {
    fn bind(&self);
    fn clear(&self);
}

// SCREEN RENDER TARGET
pub struct ScreenRendertarget {
    gl: gl::Gl,
    width: usize,
    height: usize
}

impl ScreenRendertarget
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<ScreenRendertarget, Error>
    {
        Ok(ScreenRendertarget { gl: gl.clone(), width, height })
    }
}

impl Rendertarget for ScreenRendertarget
{
    fn bind(&self)
    {
        bind(&self.gl, 0, self.width, self.height);
    }

    fn clear(&self)
    {
        clear(&self.gl);
    }
}

// COLOR RENDER TARGET
pub struct ColorRendertarget {
    gl: gl::Gl,
    id: u32,
    width: usize,
    height: usize,
    pub color_texture: Rc<texture::Texture2D>
}

impl ColorRendertarget
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<ColorRendertarget, Error>
    {
        let id = generate(gl)?;
        bind(gl, id, width, height);

        let draw_buffers = vec![gl::COLOR_ATTACHMENT0];
        let color_texture = texture::Texture2D::create_as_color_rendertarget(gl, width, height, 0)?;

        unsafe {
            gl.DrawBuffers(1, draw_buffers.as_ptr());
        }

        Ok(ColorRendertarget { gl: gl.clone(), id, width, height, color_texture: Rc::from(color_texture) })
    }
}

impl Rendertarget for ColorRendertarget
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

impl Drop for ColorRendertarget {
    fn drop(&mut self) {
        drop(&self.gl, &self.id);
    }
}


// COMMON FUNCTIONS
fn generate(gl: &gl::Gl) -> Result<u32, Error>
{
    let mut id: u32 = 0;
    unsafe {
        gl.GenFramebuffers(1, &mut id);
    }
    Ok(id)
}

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

fn drop(gl: &gl::Gl, id: &u32)
{
    unsafe {
        gl.DeleteFramebuffers(1, id);
    }
}