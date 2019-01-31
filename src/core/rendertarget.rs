use gl;
use crate::core::state;
use crate::core::texture;

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
        bind(&self.gl, &0, self.width, self.height);
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
    pub width: usize,
    pub height: usize,
    pub targets: Vec<texture::Texture2D>,
    pub depth_target: texture::Texture2D
}

impl ColorRendertarget
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize, no_targets: usize) -> Result<ColorRendertarget, Error>
    {
        let id = generate(gl)?;
        bind(gl, &id, width, height);

        let mut draw_buffers = Vec::new();
        let mut targets = Vec::new();
        for i in 0..no_targets {
            draw_buffers.push(gl::bindings::COLOR_ATTACHMENT0 + i as u32);
            targets.push(texture::Texture2D::new_as_color_target(gl, width, height, i as u32)?)
        }

        gl.draw_buffers(&draw_buffers);

        let depth_target = texture::Texture2D::new_as_depth_target(gl, width, height)?;
        Ok(ColorRendertarget { gl: gl.clone(), id, width, height, targets, depth_target })
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
        clear(&self.gl);
    }
}

impl Drop for ColorRendertarget {
    fn drop(&mut self) {
        drop(&self.gl, &self.id);
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
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<DepthRenderTarget, Error>
    {
        let id = generate(gl)?;
        bind(gl, &id, width, height);

        let target = texture::Texture2D::new_as_depth_target(gl, width, height)?;
        Ok(DepthRenderTarget { gl: gl.clone(), id, width, height, target })
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
        clear(&self.gl);
    }
}

impl Drop for DepthRenderTarget {
    fn drop(&mut self) {
        drop(&self.gl, &self.id);
    }
}


// COMMON FUNCTIONS
fn generate(gl: &gl::Gl) -> Result<u32, Error>
{
    gl.create_framebuffer().ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )
}

fn bind(gl: &gl::Gl, id: &gl::Framebuffer, width: usize, height: usize)
{
    gl.bind_framebuffer(gl::bindings::FRAMEBUFFER, &id);
    gl.viewport(0, 0, width as i32, height as i32);
}

fn clear(gl: &gl::Gl)
{
    state::depth_write(gl,true);
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(gl::bindings::COLOR_BUFFER_BIT | gl::bindings::DEPTH_BUFFER_BIT);
}

fn drop(gl: &gl::Gl, id: &gl::Framebuffer)
{
    gl.delete_framebuffer(id);
}