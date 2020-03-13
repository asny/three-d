use crate::core::*;

pub struct RenderTarget {
    gl: Gl,
    pub color_texture: Option<Texture2D>,
    pub depth_texture: Option<Texture2D>,
    pub color_texture_array: Option<Texture2DArray>,
    pub depth_texture_array: Option<Texture2DArray>
}

impl RenderTarget
{
    pub fn new(gl: &Gl, width: usize, height: usize, color_layers: usize, depth_layers: usize) -> Result<RenderTarget, Error>
    {
        let color_texture = if color_layers == 1 && depth_layers <= 1 {
            Some(Texture2D::new_empty(gl, width, height, Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::RGBA8)?)
        } else {None};
        let depth_texture = if color_layers <= 1 && depth_layers == 1 {
            Some(Texture2D::new_empty(gl, width, height, Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F)?)
        } else {None};
        let color_texture_array = if depth_layers > 1 && color_layers == 1 || color_layers > 1 { Some(Texture2DArray::new_as_color_targets(gl, width, height, color_layers)?)} else {None};
        let depth_texture_array = if color_layers > 1 && depth_layers == 1 || depth_layers > 1 { Some(Texture2DArray::new_as_depth_targets(gl, width, height, depth_layers)?)} else {None};
        Ok(RenderTarget { gl: gl.clone(), color_texture, depth_texture, color_texture_array, depth_texture_array })
    }

    pub fn write_to_screen(gl: &Gl, x: i32, y: i32, width: usize, height: usize,
                          clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: &dyn Fn()) -> Result<(), Error>
    {
        gl.viewport(x, y, width, height);
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, None);
        RenderTarget::clear(gl, clear_color, clear_depth);
        render();
        Ok(())
    }

    pub fn write_to_color(&self, x: i32, y: i32, width: usize, height: usize,
                          clear_color: Option<&Vec4>, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write(x, y, width, height, clear_color, None, render)
    }

    pub fn write_to_depth(&self, x: i32, y: i32, width: usize, height: usize,
                          clear_depth: Option<f32>, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write(x, y, width, height, None, clear_depth, render)
    }

    pub fn write(&self, x: i32, y: i32, width: usize, height: usize,
                                    clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: &dyn Fn()) -> Result<(), Error>
    {
        self.gl.viewport(x, y, width, height);
        let id = RenderTarget::new_framebuffer(&self.gl, if self.color_texture.is_some() {1} else {0})?;

        if let Some(ref color_texture) = self.color_texture {
            color_texture.bind_as_color_target(0);
        }

        if let Some(ref depth_texture) = self.depth_texture {
            depth_texture.bind_as_depth_target();
        }

        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        RenderTarget::clear(&self.gl, clear_color, clear_depth);

        render();

        self.gl.delete_framebuffer(Some(&id));
        Ok(())
    }

    pub fn write_to_color_array(&self, x: i32, y: i32, width: usize, height: usize,
                                clear_color: Option<&Vec4>, color_channel_count: usize,
        color_channel_to_texture_layer: &dyn Fn(usize) -> usize, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write_array(x, y, width, height, clear_color, None, color_channel_count, color_channel_to_texture_layer, 0, render)
    }

    pub fn write_to_depth_array(&self, x: i32, y: i32, width: usize, height: usize,
                                clear_depth: Option<f32>, depth_layer: usize, render: &dyn Fn()) -> Result<(), Error>
    {
        self.write_array(x, y, width, height,None, clear_depth, 0, &|i| {i}, depth_layer, render)
    }

    pub fn write_array(&self, x: i32, y: i32, width: usize, height: usize,
                                                clear_color: Option<&Vec4>, clear_depth: Option<f32>, color_channel_count: usize,
                                                color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                                                depth_layer: usize, render: &dyn Fn()) -> Result<(), Error>
    {
        self.gl.viewport(x, y, width, height);
        let id = RenderTarget::new_framebuffer(&self.gl, color_channel_count)?;

        if let Some(ref color_texture) = self.color_texture_array {
            for channel in 0..color_channel_count {
                color_texture.bind_as_color_target(color_channel_to_texture_layer(channel), channel);
            }
        }

        if let Some(ref depth_texture) = self.depth_texture_array {
            depth_texture.bind_as_depth_target(depth_layer);
        }

        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        RenderTarget::clear(&self.gl, clear_color, clear_depth);

        render();

        self.gl.delete_framebuffer(Some(&id));
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_color(&self, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<u8>, Error>
    {
        let color_texture = self.color_texture.as_ref().unwrap();
        self.gl.viewport(x, y, width, height);
        let mut pixels = vec![0u8; width * height * 3];
        let id = RenderTarget::new_framebuffer(&self.gl, 1)?;
        color_texture.bind_as_color_target(0);
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, Some(&id));
        self.gl.read_pixels(x as u32, y as u32, width as u32, height as u32, gl::consts::RGB,
                            gl::consts::UNSIGNED_BYTE, &mut pixels);
        self.gl.delete_framebuffer(Some(&id));
        Ok(pixels)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_color_from_screen(gl: &Gl, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<u8>, Error>
    {
        gl.viewport(x, y, width, height);
        let mut pixels = vec![0u8; width * height * 3];
        gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, None);
        gl.read_pixels(x as u32, y as u32, width as u32, height as u32, gl::consts::RGB,
                            gl::consts::UNSIGNED_BYTE, &mut pixels);
        Ok(pixels)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<f32>, Error>
    {
        let depth_texture = self.depth_texture.as_ref().unwrap();
        self.gl.viewport(x, y, width, height);
        let mut pixels = vec![0f32; width * height];
        let id = RenderTarget::new_framebuffer(&self.gl, 0)?;
        depth_texture.bind_as_depth_target();
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        self.gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, Some(&id));
        self.gl.read_depths(x as u32, y as u32, width as u32, height as u32,
                        gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, &mut pixels);
        self.gl.delete_framebuffer(Some(&id));
        Ok(pixels)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth_from_screen(gl: &Gl, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<f32>, Error>
    {
        gl.viewport(x, y, width, height);
        let mut pixels = vec![0f32; width * height];
        gl.bind_framebuffer(gl::consts::READ_FRAMEBUFFER, None);
        gl.read_depths(x as u32, y as u32, width as u32, height as u32,
                        gl::consts::DEPTH_COMPONENT, gl::consts::FLOAT, &mut pixels);
        Ok(pixels)
    }

    fn new_framebuffer(gl: &Gl, no_color_channels: usize) -> Result<gl::Framebuffer, Error>
    {
        let id = gl.create_framebuffer()
            .ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )?;
        gl.bind_framebuffer(gl::consts::DRAW_FRAMEBUFFER, Some(&id));

        if no_color_channels > 0 {
            let mut draw_buffers = Vec::new();
            for i in 0..no_color_channels {
                draw_buffers.push(gl::consts::COLOR_ATTACHMENT0 + i as u32);
            }
            gl.draw_buffers(&draw_buffers);
        }
        Ok(id)
    }

    fn clear(gl: &Gl, clear_color: Option<&Vec4>, clear_depth: Option<f32>) {
        if let Some(color) = clear_color {
            if let Some(depth) = clear_depth {
                gl.clear_color(color.x, color.y, color.z, color.w);
                depth_write(gl,true);
                gl.clear_depth(depth);
                gl.clear(gl::consts::COLOR_BUFFER_BIT | gl::consts::DEPTH_BUFFER_BIT);
            }
            else {
                gl.clear_color(color.x, color.y, color.z, color.w);
                gl.clear(gl::consts::COLOR_BUFFER_BIT);
            }
        } else if let Some(depth) = clear_depth {
            gl.clear_depth(depth);
            depth_write(gl, true);
            gl.clear(gl::consts::DEPTH_BUFFER_BIT);
        }
    }

    /*pub fn blit_color_and_depth_to(&self, target: &RenderTarget, target_color_texture: &Texture2D, target_depth_texture: &Texture2D)
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

#[cfg(all(not(target_arch = "wasm32"), feature = "image-io"))]
pub fn save_screenshot(path: &str, gl: &Gl, width: usize, height: usize) -> Result<(), Error>
{
    let pixels = RenderTarget::read_color_from_screen(gl, 0, 0,width, height)?;
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