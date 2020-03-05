use crate::core::*;

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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn pixels(gl: &Gl, width: usize, height: usize, dst_data: &mut [u8])
    {
        Self::read(gl);
        gl.read_pixels(0, 0, width as u32, height as u32, gl::consts::RGB,
                            gl::consts::UNSIGNED_BYTE, dst_data);
    }

    #[cfg(not(target_arch = "wasm32"))]
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

    pub fn clear_color_and_depth(gl: &Gl, color: &Vec4, depth: f32)
    {
        gl.clear_color(color.x, color.y, color.z, color.w);
        gl.clear_depth(depth);
        depth_write(gl,true);
        gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
    }

    pub fn clear_depth(gl: &Gl, depth: f32)
    {
        depth_write(gl, true);
        gl.clear_depth(depth);
        gl.clear(gl::consts::DEPTH_BUFFER_BIT);
    }
}

pub struct RenderTarget {
    gl: Gl,
    pub color_texture: Option<Texture2D>,
    pub depth_texture: Option<Texture2D>,
    pub color_texture_array: Option<Texture2DArray>,
    pub depth_texture_array: Option<Texture2DArray>
}

impl RenderTarget
{
    fn new_framebuffer(gl: &Gl, no_color_channels: usize) -> Result<gl::Framebuffer, Error>
    {
        let id = gl.create_framebuffer()
            .ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));

        let mut draw_buffers = Vec::new();
        for i in 0..no_color_channels {
            draw_buffers.push(gl::consts::COLOR_ATTACHMENT0 + i as u32);
        }
        gl.draw_buffers(&draw_buffers);
        Ok(id)
    }

    pub fn new(gl: &Gl, width: usize, height: usize, color_layers: usize, depth_layers: usize) -> Result<RenderTarget, Error>
    {
        let color_texture = if color_layers == 1 && depth_layers <= 1 { Some(Texture2D::new_as_color_target(gl, width, height)?) } else {None};
        let depth_texture = if color_layers <= 1 && depth_layers == 1 { Some(Texture2D::new_as_depth_target(gl, width, height)?) } else {None};
        let color_texture_array = if depth_layers > 1 && color_layers == 1 || color_layers > 1 { Some(Texture2DArray::new_as_color_targets(gl, width, height, color_layers)?)} else {None};
        let depth_texture_array = if color_layers > 1 && depth_layers == 1 || depth_layers > 1 { Some(Texture2DArray::new_as_depth_targets(gl, width, height, depth_layers)?)} else {None};
        Ok(RenderTarget { gl: gl.clone(), color_texture, depth_texture, color_texture_array, depth_texture_array })
    }

    pub fn write_to_color(&self, clear_color: Option<&Vec4>, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write_to_color_and_depth(clear_color, None, render)
    }

    pub fn write_to_depth(&self, clear_depth: Option<f32>, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write_to_color_and_depth(None, clear_depth, render)
    }

    pub fn write_to_color_and_depth(&self, clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: &dyn Fn()) -> Result<(), Error>
    {
        let id = RenderTarget::new_framebuffer(&self.gl, if self.color_texture.is_some() {1} else {0})?;

        if let Some(ref color_texture) = self.color_texture {
            self.gl.viewport(0, 0, color_texture.width as i32, color_texture.height as i32);
            color_texture.bind_to_framebuffer(0);
        }

        if let Some(ref depth_texture) = self.depth_texture {
            self.gl.viewport(0, 0, depth_texture.width as i32, depth_texture.height as i32);
            depth_texture.bind_to_depth_target();
        }

        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        self.clear(clear_color, clear_depth);

        render();

        self.gl.delete_framebuffer(Some(&id));
        Ok(())
    }

    pub fn write_to_color_array(&self, clear_color: Option<&Vec4>, color_channel_count: usize,
        color_channel_to_texture_layer: &dyn Fn(usize) -> usize, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write_to_color_array_and_depth_array(clear_color, None, color_channel_count, color_channel_to_texture_layer, 0, render)
    }

    pub fn write_to_depth_array(&self, clear_depth: Option<f32>, depth_layer: usize, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write_to_color_array_and_depth_array(None, clear_depth, 0, &|i| {i}, depth_layer, render)
    }

    pub fn write_to_color_array_and_depth_array(&self, clear_color: Option<&Vec4>, clear_depth: Option<f32>, color_channel_count: usize,
                                                color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                                                depth_layer: usize, render: &dyn Fn()) -> Result<(), Error>
    {
        let id = RenderTarget::new_framebuffer(&self.gl, color_channel_count)?;

        if let Some(ref color_texture) = self.color_texture_array {
            self.gl.viewport(0, 0, color_texture.width as i32, color_texture.height as i32);
            for channel in 0..color_channel_count {
                color_texture.bind_to_framebuffer(color_channel_to_texture_layer(channel), channel);
            }
        }

        if let Some(ref depth_texture) = self.depth_texture_array {
            self.gl.viewport(0, 0, depth_texture.width as i32, depth_texture.height as i32);
            depth_texture.bind_to_depth_target(depth_layer);
        }

        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        self.clear(clear_color, clear_depth);

        render();

        self.gl.delete_framebuffer(Some(&id));
        Ok(())
    }

    fn clear(&self, clear_color: Option<&Vec4>, clear_depth: Option<f32>) {
        if let Some(color) = clear_color {
            if let Some(depth) = clear_depth {
                self.gl.clear_color(color.x, color.y, color.z, color.w);
                depth_write(&self.gl,true);
                self.gl.clear_depth(depth);
                self.gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
            }
            else {
                self.gl.clear_color(color.x, color.y, color.z, color.w);
                self.gl.clear(gl::consts::COLOR_BUFFER_BIT);
            }
        } else if let Some(depth) = clear_depth {
            self.gl.clear_depth(depth);
            depth_write(&self.gl, true);
            self.gl.clear(gl::consts::DEPTH_BUFFER_BIT);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_from_color(&self, width: usize, height: usize) -> Result<Vec<u8>, Error>
    {
        let color_texture = self.color_texture.as_ref().unwrap();

        let id = RenderTarget::new_framebuffer(&self.gl, 1)?;
        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, Some(&id));
        self.gl.viewport(0, 0, color_texture.width as i32, color_texture.height as i32);
        color_texture.bind_to_framebuffer(0);

        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        let mut pixels = vec![0u8; width * height * 3];
        self.gl.read_pixels(0, 0, width as u32, height as u32, gl::consts::RGB,
                            gl::consts::UNSIGNED_BYTE, &mut pixels);

        self.gl.delete_framebuffer(Some(&id));
        Ok(pixels)
    }

    /*#[cfg(not(target_arch = "wasm32"))]
    pub fn depths(&self, width: usize, height: usize, dst_data: &mut [f32])
    {
        self.read();
        self.gl.read_depths(0, 0, width as u32, height as u32,
                            gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, dst_data);
    }

    pub fn blit_color_and_depth_to(&self, target: &RenderTarget, target_color_texture: &Texture2D, target_depth_texture: &Texture2D)
    {
        self.read();
        target.write_to_color_and_depth(target_color_texture, target_depth_texture).unwrap();
        depth_write(&self.gl, true);
        self.gl.blit_framebuffer(0, 0, target_color_texture.width as u32, target_color_texture.height as u32,
                                 0, 0, target_color_texture.width as u32, target_color_texture.height as u32,
                                 gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
    }

    pub fn blit_color_to(&self, target: &RenderTarget, target_texture: &Texture2D)
    {
        self.read();
        target.write_to_color(target_texture).unwrap();
        self.gl.blit_framebuffer(0, 0, target_texture.width as u32, target_texture.height as u32,
                                 0, 0, target_texture.width as u32, target_texture.height as u32,
                                 gl::consts::COLOR_BUFFER_BIT, gl::consts::NEAREST);
    }

    pub fn blit_depth_to(&self, target: &RenderTarget, target_texture: &Texture2D)
    {
        self.read();
        target.write_to_depth(target_texture).unwrap();
        depth_write(&self.gl, true);
        self.gl.blit_framebuffer(0, 0, target_texture.width as u32, target_texture.height as u32,
                                 0, 0, target_texture.width as u32, target_texture.height as u32,
                                 gl::consts::DEPTH_BUFFER_BIT, gl::consts::NEAREST);
    }*/
}

/*impl Drop for RenderTarget {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(self.id.as_ref());
    }
}*/

// COMMON FUNCTIONS

#[cfg(all(not(target_arch = "wasm32"), feature = "image-io"))]
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