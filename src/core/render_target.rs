use crate::core::*;
use crate::context::{Context, consts};
use crate::ImageEffect;

pub struct Screen {}

impl Screen {
    pub fn write<F: FnOnce() -> Result<(), Error>>(context: &Context, clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: F) -> Result<(), Error>
    {
        context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, None);
        RenderTarget::clear(context, clear_color, clear_depth);
        render()?;
        Ok(())
    }

    pub fn copy_from(context: &Context, color_texture: Option<&Texture2D>, depth_texture: Option<&Texture2D>, _filter: Interpolation, viewport: Viewport) -> Result<(), Error>
    {
        if color_texture.is_none() && depth_texture.is_none() {
            Err(Error::FailedToCopyFramebuffer {message: "A copy operation must copy either color or depth or both.".to_owned()})?
        }
        /*let id_source = RenderTarget::new_framebuffer(context, if color_texture.is_some() {1} else {0})?;

        let mut source_width = 0;
        let mut source_height = 0;

        if let Some(source) = color_texture {
            Program::set_color_mask(context, ColorMask::enabled());
            source.bind_as_color_target(0);
            source_width = source.width;
            source_height = source.height;
        }

        if let Some(source) = depth_texture {
            Program::set_depth(context, None, true);
            source.bind_as_depth_target();
            source_width = source.width;
            source_height = source.height;
        }

        #[cfg(feature = "debug")]
        {
            context.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;
        }

        context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&id_source));
        context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, None);

        let mask = if depth_texture.is_some() && color_texture.is_some() {consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT} else {
            if depth_texture.is_some() { consts::DEPTH_BUFFER_BIT } else {consts::COLOR_BUFFER_BIT}};
        context.blit_framebuffer(0, 0, source_width as u32, source_height as u32,
                                 viewport.x as u32, viewport.y as u32, viewport.width as u32, viewport.height as u32,
                                 mask, filter as u32);

        context.delete_framebuffer(Some(&id_source));*/

        let effect = get_copy_effect(context)?;
        Self::write(context, None, None, || {
            if let Some(tex) = color_texture {
                effect.program().use_texture(tex, "colorMap")?;
            }
            if let Some(tex) = depth_texture {
                effect.program().use_texture(tex, "depthMap")?;
            }
            effect.apply(RenderStates {cull: CullType::Back, depth_test: DepthTestType::Always, depth_mask: depth_texture.is_some(),
                color_mask: if color_texture.is_some() {ColorMask::enabled()} else {ColorMask::disabled()}, ..Default::default()}, viewport)?;
            Ok(())
        })?;
        Ok(())
    }

    // TODO: Possible to change format
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_color(context: &Context, viewport: Viewport) -> Result<Vec<u8>, Error>
    {
        let mut pixels = vec![0u8; viewport.width * viewport.height * 3];
        context.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        context.read_pixels_with_u8_data(viewport.x as u32,
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
    pub fn read_depth(context: &Context, viewport: Viewport) -> Result<Vec<f32>, Error>
    {
        let mut pixels = vec![0f32; viewport.width * viewport.height];
        context.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        context.read_pixels_with_f32_data(viewport.x as u32,
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
    pub fn write_to_color<F: FnOnce() -> Result<(), Error>>(context: &Context,
                                                            clear_color: Option<&Vec4>, color_texture: Option<&Texture2D>, render: F) -> Result<(), Error>
    {
        Self::write(context, clear_color, None, color_texture, None, render)
    }

    pub fn write_to_depth<F: FnOnce() -> Result<(), Error>>(context: &Context,
                                                            clear_depth: Option<f32>, depth_texture: Option<&Texture2D>, render: F) -> Result<(), Error>
    {
        Self::write(context,None, clear_depth, None, depth_texture, render)
    }

    pub fn write<F: FnOnce() -> Result<(), Error>>(context: &Context,
                                                   clear_color: Option<&Vec4>, clear_depth: Option<f32>,
                                                   color_texture: Option<&Texture2D>, depth_texture: Option<&Texture2D>,
                                                   render: F) -> Result<(), Error>
    {
        RenderTarget::render(context, if color_texture.is_some() {1} else {0}, |_| {
            if let Some(color_texture) = color_texture {
                color_texture.bind_as_color_target(0);
            }
            if let Some(depth_texture) = depth_texture {
                depth_texture.bind_as_depth_target();
            }
            #[cfg(feature = "debug")]
            Self::check(context)?;
            RenderTarget::clear(context, clear_color, clear_depth);
            render()?;
            Ok(())
        })?;

        if let Some(color_texture) = color_texture {
            color_texture.generate_mip_maps();
        }
        if let Some(depth_texture) = depth_texture {
            depth_texture.generate_mip_maps();
        }
        Ok(())
    }

    pub fn copy_color(context: &Context, source_texture: &Texture2D, target_texture: &Texture2D, filter: Interpolation) -> Result<(), Error>
    {
        Self::copy(context, Some((source_texture, target_texture)), None, filter)
    }

    pub fn copy_depth(context: &Context, source_texture: &Texture2D, target_texture: &Texture2D, filter: Interpolation) -> Result<(), Error>
    {
        Self::copy(context, None, Some((source_texture, target_texture)), filter)
    }

    pub fn copy(context: &Context, color_texture: Option<(&Texture2D, &Texture2D)>, depth_texture: Option<(&Texture2D, &Texture2D)>,
                filter: Interpolation) -> Result<(), Error>
    {
        if color_texture.is_none() && depth_texture.is_none() {
            Err(Error::FailedToCopyFramebuffer {message: "A copy operation must copy either color or depth or both.".to_owned()})?
        }

        RenderTarget::render(context, if color_texture.is_some() {1} else {0}, |id| {
            if let Some((source, _)) = color_texture {
                Program::set_color_mask(context, ColorMask::enabled());
                source.bind_as_color_target(0);
            }
            if let Some((source, _)) = depth_texture {
                Program::set_depth(context, None, true);
                source.bind_as_depth_target();
            }

            context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(id));
            #[cfg(feature = "debug")]
            Self::check(context)?;

            RenderTarget::render(context, if color_texture.is_some() {1} else {0}, |_| {
                if let Some((_, target)) = color_texture {
                    target.bind_as_color_target(0);
                }

                if let Some((_, target)) = depth_texture {
                    target.bind_as_depth_target();
                }
                #[cfg(feature = "debug")]
                Self::check(context)?;

                let (source_width, source_height) = if let Some((tex, _)) = color_texture {(tex.width, tex.height)} else {(depth_texture.as_ref().unwrap().0.width, depth_texture.as_ref().unwrap().0.height)};
                let (target_width, target_height) = if let Some((_, tex)) = color_texture {(tex.width, tex.height)} else {(depth_texture.as_ref().unwrap().1.width, depth_texture.as_ref().unwrap().1.height)};
                let mask = if depth_texture.is_some() && color_texture.is_some() {consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT} else {
                    if depth_texture.is_some() { consts::DEPTH_BUFFER_BIT } else {consts::COLOR_BUFFER_BIT}};
                context.blit_framebuffer(0, 0, source_width as u32, source_height as u32,
                                         0, 0, target_width as u32, target_height as u32,
                                         mask, filter as u32);
                Ok(())
            })?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn write_to_color_array<F: FnOnce() -> Result<(), Error>>(context: &Context,
                                                                  clear_color: Option<&Vec4>,
                                                                  color_texture_array: Option<&Texture2DArray>,
                                                                  color_channel_count: usize, color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                                                                  render: F) -> Result<(), Error>
    {
        Self::write_array(context, clear_color, None, color_texture_array, None, color_channel_count, color_channel_to_texture_layer, 0, render)
    }

    pub fn write_to_depth_array<F: FnOnce() -> Result<(), Error>>(context: &Context, clear_depth: Option<f32>,
                                                                  depth_texture_array: Option<&Texture2DArray>, depth_layer: usize,
                                                                  render: F) -> Result<(), Error>
    {
        Self::write_array(context, None, clear_depth, None, depth_texture_array, 0, &|i| {i}, depth_layer, render)
    }

    pub fn write_array<F: FnOnce() -> Result<(), Error>>(context: &Context,
                                                         clear_color: Option<&Vec4>, clear_depth: Option<f32>,
                                                         color_texture_array: Option<&Texture2DArray>,
                                                         depth_texture_array: Option<&Texture2DArray>,
                                                         color_channel_count: usize, color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                                                         depth_layer: usize, render: F) -> Result<(), Error>
    {
        RenderTarget::render(context, color_channel_count, |_| {
            if let Some(color_texture) = color_texture_array {
                for channel in 0..color_channel_count {
                    color_texture.bind_as_color_target(color_channel_to_texture_layer(channel), channel);
                }
            }
            if let Some(depth_texture) = depth_texture_array {
                depth_texture.bind_as_depth_target(depth_layer);
            }
            #[cfg(feature = "debug")]
            Self::check(context)?;
            RenderTarget::clear(context, clear_color, clear_depth);
            render()?;
            Ok(())
        })?;

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
        self.context.viewport(x, y, width, height);
        let mut pixels = vec![0u8; width * height * 3];
        let id = RenderTarget::new_framebuffer(&self.context, 1)?;
        color_texture.bind_as_color_target(0);
        self.context.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&id));
        self.context.read_pixels(x as u32, y as u32, width as u32, height as u32, consts::RGB,
                            consts::UNSIGNED_BYTE, &mut pixels);
        self.context.delete_framebuffer(Some(&id));
        Ok(pixels)
    }*/

    /*#[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self, x: i32, y: i32, width: usize, height: usize) -> Result<Vec<f32>, Error>
    {
        let depth_texture = self.depth_texture.as_ref().unwrap();
        self.context.viewport(x, y, width, height);
        let mut pixels = vec![0f32; width * height];
        let id = RenderTarget::new_framebuffer(&self.context, 0)?;
        depth_texture.bind_as_depth_target();
        self.context.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))?;

        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&id));
        self.context.read_depths(x as u32, y as u32, width as u32, height as u32,
                        consts::DEPTH_COMPONENT, consts::FLOAT, &mut pixels);
        self.context.delete_framebuffer(Some(&id));
        Ok(pixels)
    }*/

    fn render<F: FnOnce(&crate::context::Framebuffer) -> Result<(), Error>>(context: &Context, no_color_channels: usize, callback: F) -> Result<(), Error>
    {
        let id = context.create_framebuffer()
            .ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )?;
        context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&id));

        if no_color_channels > 0 {
            let mut draw_buffers = Vec::new();
            for i in 0..no_color_channels {
                draw_buffers.push(consts::COLOR_ATTACHMENT0 + i as u32);
            }
            context.draw_buffers(&draw_buffers);
        }

        callback(&id)?;

        context.delete_framebuffer(Some(&id));
        Ok(())
    }

    #[cfg(feature = "debug")]
    fn check(context: &Context) -> Result<(), Error> {
        context.check_framebuffer_status().or_else(|message| Err(Error::FailedToCreateFramebuffer {message}))
    }

    fn clear(context: &Context, clear_color: Option<&Vec4>, clear_depth: Option<f32>) {
        if let Some(color) = clear_color {
            Program::set_color_mask(context, ColorMask::enabled());
            if let Some(depth) = clear_depth {
                Program::set_depth(context, None, true);
                context.clear_color(color.x, color.y, color.z, color.w);
                context.clear_depth(depth);
                context.clear(consts::COLOR_BUFFER_BIT | consts::DEPTH_BUFFER_BIT);
            }
            else {
                context.clear_color(color.x, color.y, color.z, color.w);
                context.clear(consts::COLOR_BUFFER_BIT);
            }
        } else if let Some(depth) = clear_depth {
            Program::set_depth(context, None, true);
            context.clear_depth(depth);
            context.clear(consts::DEPTH_BUFFER_BIT);
        }
    }

    /*pub fn blit_color_and_depth_to(&self, target: &RenderTarget, target_color_texture: &Texture2D, target_depth_texture: &Texture2D)
    {
        self.read();
        target.write_to_color_and_depth(target_color_texture, target_depth_texture).unwrap();
        depth_write(&self.context, true);
        self.context.blit_framebuffer(0, 0, target_color_texture.width as u32, target_color_texture.height as u32,
                                 0, 0, target_color_texture.width as u32, target_color_texture.height as u32,
                                 consts::COLOR_BUFFER_BIT | consts::DEPTH_BUFFER_BIT, consts::NEAREST);
    }

    pub fn blit_color_to(&self, target: &RenderTarget, target_texture: &Texture2D)
    {
        self.read();
        target.write_to_color(target_texture).unwrap();
        self.context.blit_framebuffer(0, 0, target_texture.width as u32, target_texture.height as u32,
                                 0, 0, target_texture.width as u32, target_texture.height as u32,
                                 consts::COLOR_BUFFER_BIT, consts::NEAREST);
    }

    pub fn blit_depth_to(&self, target: &RenderTarget, target_texture: &Texture2D)
    {
        self.read();
        target.write_to_depth(target_texture).unwrap();
        depth_write(&self.context, true);
        self.context.blit_framebuffer(0, 0, target_texture.width as u32, target_texture.height as u32,
                                 0, 0, target_texture.width as u32, target_texture.height as u32,
                                 consts::DEPTH_BUFFER_BIT, consts::NEAREST);
    }*/
}

fn get_copy_effect(context: &Context) -> Result<&ImageEffect, Error>
{
    unsafe {
        static mut COPY_EFFECT: Option<ImageEffect> = None;
        if COPY_EFFECT.is_none() {
            COPY_EFFECT = Some(ImageEffect::new(context, &"
                uniform sampler2D colorMap;
                uniform sampler2D depthMap;
                in vec2 uv;
                layout (location = 0) out vec4 color;
                void main()
                {
                    color = texture(colorMap, uv);
                    gl_FragDepth = texture(depthMap, uv).r;
                }")?);
        }
        Ok(COPY_EFFECT.as_ref().unwrap())
    }
}