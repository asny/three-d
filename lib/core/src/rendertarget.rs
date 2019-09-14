use crate::*;

#[derive(Debug)]
pub enum Error {
    Texture(texture::Error),
    IO(std::io::Error),
    FailedToCreateFramebuffer {message: String}
}

impl From<crate::texture::Error> for Error {
    fn from(other: crate::texture::Error) -> Self {
        Error::Texture(other)
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}

// COLOR RENDER TARGET
pub struct ColorRendertarget {
    gl: Gl,
    id: Option<gl::Framebuffer>,
    pub width: usize,
    pub height: usize,
    pub targets: Vec<Texture2D>,
    pub depth_target: Option<Texture2D>
}

impl ColorRendertarget
{
    pub fn default(gl: &Gl, width: usize, height: usize) -> Result<ColorRendertarget, Error>
    {
        Ok(ColorRendertarget { gl: gl.clone(), width, height, id: None, targets: Vec::new(), depth_target: None })
    }

    pub fn new(gl: &Gl, width: usize, height: usize, no_targets: usize, depth: bool) -> Result<ColorRendertarget, Error>
    {
        let id = generate(gl)?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));

        let mut draw_buffers = Vec::new();
        let mut targets = Vec::new();
        for i in 0..no_targets {
            draw_buffers.push(gl::consts::COLOR_ATTACHMENT0 + i as u32);
            targets.push(Texture2D::new_as_color_target(gl, width, height, i as u32)?)
        }

        gl.draw_buffers(&draw_buffers);

        let depth_target = if depth
        {
            Some(Texture2D::new_as_depth_target(gl, width, height)?)
        }
        else {None};
        gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(ColorRendertarget { gl: gl.clone(), id: Some(id), width, height, targets, depth_target })
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
        if self.depth_target.is_some() {
            self.bind();
            self.gl.read_depths(0, 0, self.width as u32, self.height as u32, gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
        }
    }

    pub fn bind(&self)
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, self.id.as_ref());
        self.gl.viewport(0, 0, self.width as i32, self.height as i32);
    }

    pub fn bind_for_read(&self)
    {
        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, self.id.as_ref());
    }

    pub fn clear(&self, color: &Vec4)
    {
        self.gl.clear_color(color.x, color.y, color.z, color.w);
        if self.depth_target.is_some() {
            depth_write(&self.gl,true);
            self.gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
        }
        else {
            self.gl.clear(gl::consts::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_depth(&self)
    {
        if self.depth_target.is_some() {
            depth_write(&self.gl, true);
            self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
        }
    }

    pub fn blit_to(&self, target: &ColorRendertarget)
    {
        self.bind_for_read();
        target.bind();
        self.gl.blit_framebuffer(0, 0, self.width as u32, self.height as u32,
                                 0, 0, target.width as u32, target.height as u32,
                                 gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
    }
}

impl Drop for ColorRendertarget {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(self.id.as_ref());
    }
}

// DEPTH RENDER TARGET
pub struct DepthRenderTarget {
    gl: Gl,
    id: gl::Framebuffer,
    pub width: usize,
    pub height: usize,
    pub target: Texture2D
}

impl DepthRenderTarget
{
    pub fn new(gl: &Gl, width: usize, height: usize) -> Result<DepthRenderTarget, Error>
    {
        let id = generate(gl)?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));

        let target = Texture2D::new_as_depth_target(gl, width, height)?;
        gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(DepthRenderTarget { gl: gl.clone(), id, width, height, target })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn depths(&self, dst_data: &mut [f32])
    {
        self.bind();
        self.gl.read_depths(0, 0, self.width as u32, self.height as u32, gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }

    pub fn bind(&self)
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&self.id));
        self.gl.viewport(0, 0, self.width as i32, self.height as i32);
    }

    pub fn bind_for_read(&self)
    {
        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, Some(&self.id));
    }

    pub fn clear(&self)
    {
        depth_write(&self.gl,true);
        self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }
}

impl Drop for DepthRenderTarget {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(Some(&self.id));
    }
}

pub struct DepthRenderTargetArray {
    gl: Gl,
    id: gl::Framebuffer,
    pub width: usize,
    pub height: usize,
    pub target: Texture2DArray
}

impl DepthRenderTargetArray
{
    pub fn new(gl: &Gl, width: usize, height: usize, layers: usize) -> Result<DepthRenderTargetArray, Error>
    {
        let id = generate(gl)?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));

        let target = Texture2DArray::new_as_depth_targets(gl, width, height, layers)?;
        gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(DepthRenderTargetArray { gl: gl.clone(), id, width, height, target })
    }

    pub fn bind(&self, layer: usize)
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&self.id));
        self.gl.viewport(0, 0, self.width as i32, self.height as i32);
        self.target.bind_to_framebuffer(layer, 0);
    }

    pub fn clear(&self)
    {
        depth_write(&self.gl,true);
        self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }
}

impl Drop for DepthRenderTargetArray {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(Some(&self.id));
    }
}

// COMMON FUNCTIONS
fn generate(gl: &Gl) -> Result<gl::Framebuffer, Error>
{
    gl.create_framebuffer().ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )
}

#[cfg(target_arch = "x86_64")]
pub fn save_screenshot(path: &str, rendertarget: &ColorRendertarget) -> Result<(), Error>
{
    let mut pixels = vec![0u8; rendertarget.width * rendertarget.height * 3];
    rendertarget.pixels(&mut pixels);
    image::save_buffer(&std::path::Path::new(path), &pixels, rendertarget.width as u32, rendertarget.height as u32, image::RGB(8))?;
    Ok(())
}