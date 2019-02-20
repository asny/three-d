use gl;
use crate::core::state;
use crate::core::texture;
use crate::types::*;

#[derive(Debug)]
pub enum Error {
    Texture(texture::Error),
    FailedToCreateFramebuffer {message: String}
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
    pub width: usize,
    pub height: usize,
    clear_color: Vec4
}

impl ScreenRendertarget
{
    pub fn new(gl: &gl::Gl, width: usize, height: usize, clear_color: Vec4) -> Result<ScreenRendertarget, Error>
    {
        Ok(ScreenRendertarget { gl: gl.clone(), width, height, clear_color })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn pixels(&self, dst_data: &mut [u8])
    {
        self.bind();
        self.gl.read_pixels(0, 0, self.width as u32, self.height as u32, gl::consts::RGB, gl::consts::UNSIGNED_BYTE, dst_data);
    }

    #[cfg(target_arch = "x86_64")]
    pub fn depths(&self, dst_data: &mut [f32])
    {
        self.bind();
        self.gl.read_depths(0, 0, self.width as u32, self.height as u32, gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }
}

impl Rendertarget for ScreenRendertarget
{
    fn bind(&self)
    {
        self.gl.bind_framebuffer(gl::consts::FRAMEBUFFER, None);
        self.gl.viewport(0, 0, self.width as i32, self.height as i32);
    }

    fn clear(&self)
    {
        state::depth_write(&self.gl,true);
        self.gl.clear_color(self.clear_color.x, self.clear_color.y, self.clear_color.z, self.clear_color.w);
        self.gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
    }
}

// COLOR RENDER TARGET
pub struct ColorRendertarget {
    gl: gl::Gl,
    id: gl::Framebuffer,
    pub width: usize,
    pub height: usize,
    pub targets: Vec<texture::Texture2D>,
    pub depth_target: texture::Texture2D,
    pub clear_color: Vec4
}

impl ColorRendertarget
{
    pub fn new(gl: &gl::Gl, width: usize, height: usize, no_targets: usize, clear_color: Vec4) -> Result<ColorRendertarget, Error>
    {
        let id = generate(gl)?;
        bind(gl, &id, width, height);

        let mut draw_buffers = Vec::new();
        let mut targets = Vec::new();
        for i in 0..no_targets {
            draw_buffers.push(gl::consts::COLOR_ATTACHMENT0 + i as u32);
            targets.push(texture::Texture2D::new_as_color_target(gl, width, height, i as u32)?)
        }

        gl.draw_buffers(&draw_buffers);

        let depth_target = texture::Texture2D::new_as_depth_target(gl, width, height)?;
        gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(ColorRendertarget { gl: gl.clone(), id, width, height, targets, depth_target, clear_color })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn pixels(&self, dst_data: &mut [u8])
    {
        self.bind();
        self.gl.read_pixels(0, 0, self.width as u32, self.height as u32, gl::consts::RGB, gl::consts::UNSIGNED_BYTE, dst_data);
    }

    #[cfg(target_arch = "x86_64")]
    pub fn depths(&self, dst_data: &mut [f32])
    {
        self.bind();
        self.gl.read_depths(0, 0, self.width as u32, self.height as u32, gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }
}

impl Rendertarget for ColorRendertarget
{
    fn bind(&self)
    {
        bind(&self.gl, &self.id, self.width, self.height);
    }

    fn clear(&self)
    {
        state::depth_write(&self.gl,true);
        self.gl.clear_color(self.clear_color.x, self.clear_color.y, self.clear_color.z, self.clear_color.w);
        self.gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
    }
}

impl Drop for ColorRendertarget {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(Some(&self.id));
    }
}

// DEPTH RENDER TARGET
pub struct DepthRenderTarget {
    gl: gl::Gl,
    id: gl::Framebuffer,
    width: usize,
    height: usize,
    pub target: texture::Texture2D
}

impl DepthRenderTarget
{
    pub fn new(gl: &gl::Gl, width: usize, height: usize) -> Result<DepthRenderTarget, Error>
    {
        let id = generate(gl)?;
        bind(gl, &id, width, height);

        let target = texture::Texture2D::new_as_depth_target(gl, width, height)?;
        gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(DepthRenderTarget { gl: gl.clone(), id, width, height, target })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn depths(&self, dst_data: &mut [f32])
    {
        self.bind();
        self.gl.read_depths(0, 0, self.width as u32, self.height as u32, gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }
}

impl Rendertarget for DepthRenderTarget
{
    fn bind(&self)
    {
        bind(&self.gl, &self.id, self.width, self.height);
    }

    fn clear(&self)
    {
        state::depth_write(&self.gl,true);
        self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }
}

impl Drop for DepthRenderTarget {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(Some(&self.id));
    }
}


// COMMON FUNCTIONS
fn generate(gl: &gl::Gl) -> Result<gl::Framebuffer, Error>
{
    gl.create_framebuffer().ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )
}

fn bind(gl: &gl::Gl, id: &gl::Framebuffer, width: usize, height: usize)
{
    gl.bind_framebuffer(gl::consts::FRAMEBUFFER, Some(&id));
    gl.viewport(0, 0, width as i32, height as i32);
}