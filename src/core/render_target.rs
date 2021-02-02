use crate::core::*;

pub struct Screen {}

impl Screen {
    pub fn write<F: FnOnce() -> Result<(), Error>>(gl: &Gl, clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: F) -> Result<(), Error>
    {
        gl.bind_framebuffer(consts::DRAW_FRAMEBUFFER, None);
        RenderTarget::clear(gl, clear_color, clear_depth);
        render()?;
        Ok(())
    }

    // TODO: Possible to change format
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_color(gl: &Gl, viewport: Viewport) -> Result<Vec<u8>, Error>
    {
        let mut pixels = vec![0u8; viewport.width * viewport.height * 3];
        gl.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        gl.read_pixels_with_u8_data(viewport.x as u32,
                                    viewport.y as u32,
                                    viewport.width as u32,
                                    viewport.height as u32,
                                    consts::RGB,
                                    consts::UNSIGNED_BYTE,
                                    &mut pixels);
        Ok(pixels)
    }

    // TODO: Possible to change format
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(gl: &Gl, viewport: Viewport) -> Result<Vec<f32>, Error>
    {
        let mut pixels = vec![0f32; viewport.width * viewport.height];
        gl.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        gl.read_pixels_with_f32_data(viewport.x as u32,
                                     viewport.y as u32,
                                     viewport.width as u32,
                                     viewport.height as u32,
                                     consts::DEPTH_COMPONENT,
                                     consts::FLOAT,
                                     &mut pixels);
        Ok(pixels)
    }
}

pub struct RenderTarget {}

impl RenderTarget
{
    pub fn write_to_color<F: FnOnce() -> Result<(), Error>>(gl: &Gl,
                          clear_color: Option<&Vec4>, color_texture: Option<&Texture2D>, render: F) -> Result<(), Error>
    {
        Self::write(gl, clear_color, None, color_texture, None, render)
    }

    pub fn write_to_depth<F: FnOnce() -> Result<(), Error>>(gl: &Gl,
                          clear_depth: Option<f32>, depth_texture: Option<&Texture2D>, render: F) -> Result<(), Error>
    {
        Self::write(gl,None, clear_depth, None, depth_texture, render)
    }

    pub fn write<F: FnOnce() -> Result<(), Error>>(gl: &Gl,
                 clear_color: Option<&Vec4>, clear_depth: Option<f32>,
                 color_texture: Option<&Texture2D>, depth_texture: Option<&Texture2D>,
                 render: F) -> Result<(), Error>
    {
        let id = RenderTarget::new_framebuffer(gl, if color_texture.is_some() {1} else {0})?;

        if let Some(color_texture) = color_texture {
            color_texture.bind_as_color_target(0);
        }

        if let Some(depth_texture) = depth_texture {
            depth_texture.bind_as_depth_target();
        }

        #[cfg(feature = "debug")]
        {
            gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        }
        RenderTarget::clear(gl, clear_color, clear_depth);

        render()?;

        gl.delete_framebuffer(Some(&id));

        if let Some(color_texture) = color_texture {
            color_texture.generate_mip_maps();
        }

        if let Some(depth_texture) = depth_texture {
            depth_texture.generate_mip_maps();
        }
        Ok(())
    }

    pub fn write_to_color_array<F: FnOnce() -> Result<(), Error>>(gl: &Gl, x: i32, y: i32, width: usize, height: usize,
                                clear_color: Option<&Vec4>,
                       color_texture_array: Option<&Texture2DArray>,
                                color_channel_count: usize, color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                                render: F) -> Result<(), Error>
    {
        Self::write_array(gl, x, y, width, height, clear_color, None, color_texture_array, None, color_channel_count, color_channel_to_texture_layer, 0, render)
    }

    pub fn write_to_depth_array<F: FnOnce() -> Result<(), Error>>(gl: &Gl, x: i32, y: i32, width: usize, height: usize, clear_depth: Option<f32>,
                       depth_texture_array: Option<&Texture2DArray>, depth_layer: usize,
                                render: F) -> Result<(), Error>
    {
        Self::write_array(gl, x, y, width, height,None, clear_depth, None, depth_texture_array, 0, &|i| {i}, depth_layer, render)
    }

    pub fn write_array<F: FnOnce() -> Result<(), Error>>(gl: &Gl, x: i32, y: i32, width: usize, height: usize,
                       clear_color: Option<&Vec4>, clear_depth: Option<f32>,
                       color_texture_array: Option<&Texture2DArray>,
                       depth_texture_array: Option<&Texture2DArray>,
                       color_channel_count: usize, color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                       depth_layer: usize, render: F) -> Result<(), Error>
    {
        gl.viewport(x, y, width, height);
        let id = RenderTarget::new_framebuffer(gl, color_channel_count)?;

        if let Some(color_texture) = color_texture_array {
            for channel in 0..color_channel_count {
                color_texture.bind_as_color_target(color_channel_to_texture_layer(channel), channel);
            }
        }

        if let Some(depth_texture) = depth_texture_array {
            depth_texture.bind_as_depth_target(depth_layer);
        }

        #[cfg(feature = "debug")]
        {
            gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        }
        RenderTarget::clear(gl, clear_color, clear_depth);

        render()?;

        gl.delete_framebuffer(Some(&id));
        if let Some(color_texture) = color_texture_array {
            color_texture.generate_mip_maps();
        }

        if let Some(depth_texture) = depth_texture_array {
            depth_texture.generate_mip_maps();
        }
        Ok(())
    }

    // TODO: Read color and depth from rendertarget to cpu
    /*#[cfg(not(target_arch = "wasm32"))]
    pub fn read_color(&self, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<u8>, Error>
    {
        let color_texture = self.color_texture.as_ref().unwrap();
        self.gl.viewport(x, y, width, height);
        let mut pixels = vec![0u8; width * height * 3];
        let id = RenderTarget::new_framebuffer(&self.gl, 1)?;
        color_texture.bind_as_color_target(0);
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        self.gl.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&id));
        self.gl.read_pixels(x as u32, y as u32, width as u32, height as u32, consts::RGB,
                            consts::UNSIGNED_BYTE, &mut pixels);
        self.gl.delete_framebuffer(Some(&id));
        Ok(pixels)
    }*/

    /*#[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<f32>, Error>
    {
        let depth_texture = self.depth_texture.as_ref().unwrap();
        self.gl.viewport(x, y, width, height);
        let mut pixels = vec![0f32; width * height];
        let id = RenderTarget::new_framebuffer(&self.gl, 0)?;
        depth_texture.bind_as_depth_target();
        self.gl.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        self.gl.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&id));
        self.gl.read_depths(x as u32, y as u32, width as u32, height as u32,
                        consts::DEPTH_COMPONENT, consts::FLOAT, &mut pixels);
        self.gl.delete_framebuffer(Some(&id));
        Ok(pixels)
    }*/

    fn new_framebuffer(gl: &Gl, no_color_channels: usize) -> Result<crate::context::Framebuffer, Error>
    {
        let id = gl.create_framebuffer()
            .ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )?;
        gl.bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&id));

        if no_color_channels > 0 {
            let mut draw_buffers = Vec::new();
            for i in 0..no_color_channels {
                draw_buffers.push(consts::COLOR_ATTACHMENT0 + i as u32);
            }
            gl.draw_buffers(&draw_buffers);
        }
        Ok(id)
    }

    fn clear(gl: &Gl, clear_color: Option<&Vec4>, clear_depth: Option<f32>) {
        if let Some(color) = clear_color {
            Program::set_color_mask(gl, ColorMask::default());
            if let Some(depth) = clear_depth {
                Program::set_depth(gl, None, true);
                gl.clear_color(color.x, color.y, color.z, color.w);
                gl.clear_depth(depth);
                gl.clear(consts::COLOR_BUFFER_BIT | consts::DEPTH_BUFFER_BIT);
            }
            else {
                gl.clear_color(color.x, color.y, color.z, color.w);
                gl.clear(consts::COLOR_BUFFER_BIT);
            }
        } else if let Some(depth) = clear_depth {
            Program::set_depth(gl, None, true);
            gl.clear_depth(depth);
            gl.clear(consts::DEPTH_BUFFER_BIT);
        }
    }

    /*pub fn blit_color_and_depth_to(&self, target: &RenderTarget, target_color_texture: &Texture2D, target_depth_texture: &Texture2D)
    {
        self.read();
        target.write_to_color_and_depth(target_color_texture, target_depth_texture).unwrap();
        depth_write(&self.gl, true);
        self.gl.blit_framebuffer(0, 0, target_color_texture.width as u32, target_color_texture.height as u32,
                                 0, 0, target_color_texture.width as u32, target_color_texture.height as u32,
                                 consts::COLOR_BUFFER_BIT | consts::DEPTH_BUFFER_BIT, consts::NEAREST);
    }

    pub fn blit_color_to(&self, target: &RenderTarget, target_texture: &Texture2D)
    {
        self.read();
        target.write_to_color(target_texture).unwrap();
        self.gl.blit_framebuffer(0, 0, target_texture.width as u32, target_texture.height as u32,
                                 0, 0, target_texture.width as u32, target_texture.height as u32,
                                 consts::COLOR_BUFFER_BIT, consts::NEAREST);
    }

    pub fn blit_depth_to(&self, target: &RenderTarget, target_texture: &Texture2D)
    {
        self.read();
        target.write_to_depth(target_texture).unwrap();
        depth_write(&self.gl, true);
        self.gl.blit_framebuffer(0, 0, target_texture.width as u32, target_texture.height as u32,
                                 0, 0, target_texture.width as u32, target_texture.height as u32,
                                 consts::DEPTH_BUFFER_BIT, consts::NEAREST);
    }*/
}