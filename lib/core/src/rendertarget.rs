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

// SCREEN RENDER TARGET
pub struct ScreenRendertarget {
}

impl ScreenRendertarget
{
    pub fn write(gl: &Gl, width: usize, height: usize)
    {
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, None);
        gl.viewport(0, 0, width as i32, height as i32);
    }

    pub fn read(gl: &Gl)
    {
        gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, None);
    }

    #[cfg(target_arch = "x86_64")]
    pub fn pixels(gl: &Gl, width: usize, height: usize, dst_data: &mut [u8])
    {
        Self::read(gl);
        gl.read_pixels(0, 0, width as u32, height as u32, gl::consts::RGB,
                            gl::consts::UNSIGNED_BYTE, dst_data);
    }

    #[cfg(target_arch = "x86_64")]
    pub fn depths(gl: &Gl, width: usize, height: usize, dst_data: &mut [f32])
    {
        Self::read(gl);
        gl.read_depths(0, 0, width as u32, height as u32,
                            gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }

    pub fn clear_color(gl: &Gl, color: &Vec4)
    {
        gl.clear_color(color.x, color.y, color.z, color.w);
        gl.clear(gl::consts::COLOR_BUFFER_BIT);
    }

    pub fn clear_color_and_depth(gl: &Gl, color: &Vec4)
    {
        gl.clear_color(color.x, color.y, color.z, color.w);
        depth_write(gl,true);
        gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
    }

    pub fn clear_depth(gl: &Gl)
    {
        depth_write(gl, true);
        gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }
}

// COLOR RENDER TARGET
pub struct ColorRendertarget {
    gl: Gl,
    id: Option<gl::Framebuffer>,
}

impl ColorRendertarget
{
    pub fn new(gl: &Gl) -> Result<ColorRendertarget, Error>
    {
        let id = generate(gl)?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));
        gl.draw_buffers(&[gl::consts::COLOR_ATTACHMENT0]);

        Ok(ColorRendertarget { gl: gl.clone(), id: Some(id)})
    }

    #[cfg(target_arch = "x86_64")]
    pub fn pixels(&self, width: usize, height: usize, dst_data: &mut [u8])
    {
        self.read();
        self.gl.read_pixels(0, 0, width as u32, height as u32, gl::consts::RGB,
                            gl::consts::UNSIGNED_BYTE, dst_data);
    }

    #[cfg(target_arch = "x86_64")]
    pub fn depths(&self, width: usize, height: usize, dst_data: &mut [f32])
    {
        self.read();
        self.gl.read_depths(0, 0, width as u32, height as u32,
                            gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }

    pub fn write_to_color(&self, texture: &Texture2D) -> Result<(), Error>
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, self.id.as_ref());
        self.gl.viewport(0, 0, texture.width as i32, texture.height as i32);
        texture.bind_to_framebuffer(0);
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(())
    }

    pub fn write_to_color_and_depth(&self, texture: &Texture2D, depth_texture: &Texture2D, channel: usize) -> Result<(), Error>
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, self.id.as_ref());
        self.gl.viewport(0, 0, texture.width as i32, texture.height as i32);
        texture.bind_to_framebuffer(channel);
        depth_texture.bind_to_depth_target();
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(())
    }

    pub fn read(&self)
    {
        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, self.id.as_ref());
    }

    pub fn clear_color(&self, color: &Vec4)
    {
        self.gl.clear_color(color.x, color.y, color.z, color.w);
        self.gl.clear(gl::consts::COLOR_BUFFER_BIT);
    }

    pub fn clear_color_and_depth(&self, color: &Vec4)
    {
        self.gl.clear_color(color.x, color.y, color.z, color.w);
        depth_write(&self.gl,true);
        self.gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
    }

    pub fn clear_depth(&self)
    {
        depth_write(&self.gl, true);
        self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }

    /*pub fn blit_to(&self, target: &ColorRendertarget)
    {
        self.read();
        target.bind();
        if self.depth_target.is_some() {
            depth_write(&self.gl, true);
            self.gl.blit_framebuffer(0, 0, self.width as u32, self.height as u32,
                                     0, 0, target.width as u32, target.height as u32,
                                     gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
        }
        else {
            self.gl.blit_framebuffer(0, 0, self.width as u32, self.height as u32,
                                     0, 0, target.width as u32, target.height as u32,
                                     gl::consts::COLOR_BUFFER_BIT, gl::consts::NEAREST);
        }
    }

    pub fn blit_depth_to(&self, target: &ColorRendertarget)
    {
        if self.depth_target.is_some() {
            self.read();
            target.bind();
            depth_write(&self.gl, true);
            self.gl.blit_framebuffer(0, 0, self.width as u32, self.height as u32,
                                     0, 0, target.width as u32, target.height as u32,
                                     gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
        }
    }*/
}

impl Drop for ColorRendertarget {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(self.id.as_ref());
    }
}

// COLOR RENDER TARGET ARRAY
pub struct ColorRendertargetArray {
    gl: Gl,
    id: Option<gl::Framebuffer>,
    pub no_channels: usize
}

impl ColorRendertargetArray
{
    pub fn new(gl: &Gl, no_channels: usize) -> Result<ColorRendertargetArray, Error>
    {
        let id = generate(gl)?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));

        let mut draw_buffers = Vec::new();
        for i in 0..no_channels {
            draw_buffers.push(gl::consts::COLOR_ATTACHMENT0 + i as u32);
        }
        gl.draw_buffers(&draw_buffers);

        Ok(ColorRendertargetArray { gl: gl.clone(), id: Some(id), no_channels })
    }

    pub fn write_to_color(&self, texture: &Texture2DArray, channel_to_texture_layer_map: &dyn Fn(usize) -> usize) -> Result<(), Error>
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, self.id.as_ref());
        self.gl.viewport(0, 0, texture.width as i32, texture.height as i32);

        for channel in 0..self.no_channels {
            texture.bind_to_framebuffer(channel_to_texture_layer_map(channel), channel);
        }
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(())
    }

    pub fn write_to_color_and_depth(&self, color_texture: &Texture2DArray, depth_texture: &Texture2DArray,
                                color_channel_to_texture_layer_map: &dyn Fn(usize) -> usize, depth_layer: usize) -> Result<(), Error>
    {
        self.gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, self.id.as_ref());
        self.gl.viewport(0, 0, color_texture.width as i32, color_texture.height as i32);

        for channel in 0..self.no_channels {
            color_texture.bind_to_framebuffer(color_channel_to_texture_layer_map(channel), channel);
        }
        depth_texture.bind_to_framebuffer(depth_layer, 0);
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        Ok(())
    }

    pub fn read(&self)
    {
        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, self.id.as_ref());
    }

    pub fn clear_color(&self, color: &Vec4)
    {
        self.gl.clear_color(color.x, color.y, color.z, color.w);
        self.gl.clear(gl::consts::COLOR_BUFFER_BIT);
    }

    pub fn clear_color_and_depth(&self, color: &Vec4)
    {
        self.gl.clear_color(color.x, color.y, color.z, color.w);
        depth_write(&self.gl,true);
        self.gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
    }

    pub fn clear_depth(&self)
    {
        depth_write(&self.gl, true);
        self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }

    /*pub fn blit_to(&self, target: &ColorRendertarget)
    {
        self.read();
        target.bind();
        if target.depth_target.is_some() {
            depth_write(&self.gl, true);
            self.gl.blit_framebuffer(0, 0, target.width as u32, target.height as u32,
                                     0, 0, target.width as u32, target.height as u32,
                                     gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
        }
        else {
            self.gl.blit_framebuffer(0, 0, target.width as u32, target.height as u32,
                                     0, 0, target.width as u32, target.height as u32,
                                     gl::consts::COLOR_BUFFER_BIT, gl::consts::NEAREST);
        }
    }

    pub fn blit_depth_to(&self, target: &ColorRendertarget)
    {
        self.read();
        target.bind();
        depth_write(&self.gl, true);
        self.gl.blit_framebuffer(0, 0, target.width as u32, target.height as u32,
                                 0, 0, target.width as u32, target.height as u32,
                                 gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
    }*/
}

impl Drop for ColorRendertargetArray {
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
pub fn save_screenshot(path: &str, gl: &Gl, width: usize, height: usize) -> Result<(), Error>
{
    let mut pixels = vec![0u8; width * height * 3];
    ScreenRendertarget::pixels(gl, width, height, &mut pixels);
    let mut pixels_out = vec![0u8; width * height * 3];
    for row in 0..height {
        for col in 0..width {
            for i in 0..3 {
                pixels_out[3 * width * (height - row - 1) + 3 * col + i] =
                    pixels[3 * width * row + 3 * col + i];
            }
        }
    }

    image::save_buffer(&std::path::Path::new(path), &pixels_out, width as u32, height as u32, image::RGB(8))?;
    Ok(())
}