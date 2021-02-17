use crate::core::*;
use crate::context::{Context, consts};
use crate::ImageEffect;

pub struct Screen {}

impl Screen {
    pub fn write<F: FnOnce() -> Result<(), Error>>(context: &Context, clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: F) -> Result<(), Error>
    {
        context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, None);
        clear(context, clear_color, clear_depth);
        render()?;
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

pub struct RenderTarget<'a, 'b> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a Texture2D>,
    depth_texture: Option<&'b Texture2D>,
}

impl<'a, 'b> RenderTarget<'a, 'b>
{
    pub fn new(context: &Context, color_texture: &'a Texture2D, depth_texture: &'b Texture2D) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture)
        })
    }

    pub fn new_color(context: &Context, color_texture: &'a Texture2D) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None
        })
    }

    pub fn new_depth(context: &Context, depth_texture: &'b Texture2D) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture)
        })
    }

    pub fn write<F: FnOnce() -> Result<(), Error>>(&self, clear_color: Option<&Vec4>, clear_depth: Option<f32>, render: F) -> Result<(), Error>
    {
        self.bind()?;
        clear(&self.context,
              self.color_texture.and(clear_color),
              self.depth_texture.and(clear_depth));
        render()?;
        if let Some(color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        if let Some(depth_texture) = self.depth_texture {
            depth_texture.generate_mip_maps();
        }
        Ok(())
    }

    pub fn copy_to_screen(&self, _filter: Interpolation, viewport: Viewport) -> Result<(), Error>
    {
        let effect = get_copy_effect(&self.context)?;
        Screen::write(&self.context, None, None,|| {
            if let Some(tex) = self.color_texture {
                effect.program().use_texture(tex, "colorMap")?;
            }
            if let Some(tex) = self.depth_texture {
                effect.program().use_texture(tex, "depthMap")?;
            }
            effect.apply(RenderStates {cull: CullType::Back, depth_test: DepthTestType::Always, depth_mask: self.depth_texture.is_some(),
                color_mask: if self.color_texture.is_some() {ColorMask::enabled()} else {ColorMask::disabled()}, ..Default::default()}, viewport)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy_to(&self, other: &Self, filter: Interpolation) -> Result<(), Error>
    {
        let color = self.color_texture.is_some() && other.color_texture.is_some();
        let depth = self.depth_texture.is_some() && other.depth_texture.is_some();
        if color {
            Program::set_color_mask(&self.context, ColorMask::enabled());
        }
        if depth {
            Program::set_depth(&self.context, None, true);
        }
        let mask = if depth && color {consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT} else {
            if depth { consts::DEPTH_BUFFER_BIT } else {consts::COLOR_BUFFER_BIT}};
        let (source_width, source_height) = if let Some(tex) = self.color_texture {(tex.width, tex.height)}
            else {(self.depth_texture.as_ref().unwrap().width, self.depth_texture.as_ref().unwrap().height)};
        let (target_width, target_height) = if let Some(tex) = other.color_texture {(tex.width, tex.height)}
            else {(other.depth_texture.as_ref().unwrap().width, other.depth_texture.as_ref().unwrap().height)};

        self.bind()?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None, None, || {
            self.context.blit_framebuffer(0, 0, source_width as u32, source_height as u32,
                                          0, 0, target_width as u32, target_height as u32,
                                          mask, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    fn bind(&self) -> Result<(), Error> {
        self.context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&self.id));
        if let Some(tex) = self.color_texture {
            self.context.draw_buffers(&[consts::COLOR_ATTACHMENT0]);
            tex.bind_as_color_target(0);
        }
        if let Some(tex) = self.depth_texture {
            tex.bind_as_depth_target();
        }
        #[cfg(feature = "debug")]
            check(&self.context)?;
        Ok(())
    }
}

impl Drop for RenderTarget<'_, '_> {
    fn drop(&mut self) {
        self.context.delete_framebuffer(Some(&self.id));
    }
}


pub struct RenderTargetArray<'a, 'b> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a Texture2DArray>,
    depth_texture: Option<&'b Texture2DArray>,
}

impl<'a, 'b> RenderTargetArray<'a, 'b>
{
    pub fn new(context: &Context, color_texture: &'a Texture2DArray, depth_texture: &'b Texture2DArray) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture)
        })
    }

    pub fn new_color(context: &Context, color_texture: &'a Texture2DArray) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None
        })
    }

    pub fn new_depth(context: &Context, depth_texture: &'b Texture2DArray) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture)
        })
    }

    pub fn write<F: FnOnce() -> Result<(), Error>>(&self, clear_color: Option<&Vec4>, clear_depth: Option<f32>, color_layers: &[usize], depth_layer: usize, render: F) -> Result<(), Error>
    {
        self.bind(color_layers, depth_layer)?;
        clear(&self.context,
              self.color_texture.and(clear_color),
              self.depth_texture.and(clear_depth));
        render()?;
        if let Some(color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        if let Some(depth_texture) = self.depth_texture {
            depth_texture.generate_mip_maps();
        }
        Ok(())
    }

    pub fn copy_to(&self, other: &RenderTarget, color_layer: usize, depth_layer: usize, filter: Interpolation) -> Result<(), Error>
    {
        let color = self.color_texture.is_some() && other.color_texture.is_some();
        let depth = self.depth_texture.is_some() && other.depth_texture.is_some();
        if color {
            Program::set_color_mask(&self.context, ColorMask::enabled());
        }
        if depth {
            Program::set_depth(&self.context, None, true);
        }
        let mask = if depth && color {consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT} else {
            if depth { consts::DEPTH_BUFFER_BIT } else {consts::COLOR_BUFFER_BIT}};
        let (source_width, source_height) = if let Some(tex) = self.color_texture {(tex.width, tex.height)}
        else {(self.depth_texture.as_ref().unwrap().width, self.depth_texture.as_ref().unwrap().height)};
        let (target_width, target_height) = if let Some(tex) = other.color_texture {(tex.width, tex.height)}
        else {(other.depth_texture.as_ref().unwrap().width, other.depth_texture.as_ref().unwrap().height)};

        self.bind(&[color_layer], depth_layer)?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None, None, || {
            self.context.blit_framebuffer(0, 0, source_width as u32, source_height as u32,
                                          0, 0, target_width as u32, target_height as u32,
                                          mask, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    fn bind(&self, color_layers: &[usize], depth_layer: usize) -> Result<(), Error> {
        self.context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&self.id));
        if let Some(color_texture) = self.color_texture {
            self.context.draw_buffers(&(0..color_layers.len()).map(|i| consts::COLOR_ATTACHMENT0 + i as u32).collect::<Vec<u32>>());
            for channel in 0..color_layers.len() {
                color_texture.bind_as_color_target(color_layers[channel], channel);
            }
        }
        if let Some(depth_texture) = self.depth_texture {
            depth_texture.bind_as_depth_target(depth_layer);
        }
        #[cfg(feature = "debug")]
            check(&self.context)?;
        Ok(())
    }
}

impl Drop for RenderTargetArray<'_, '_> {
    fn drop(&mut self) {
        self.context.delete_framebuffer(Some(&self.id));
    }
}


/*pub struct RenderTargetOld {}

impl RenderTargetOld
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
        Self::render(context, if color_texture.is_some() {1} else {0}, |_| {
            if let Some(color_texture) = color_texture {
                color_texture.bind_as_color_target(0);
            }
            if let Some(depth_texture) = depth_texture {
                depth_texture.bind_as_depth_target();
            }
            #[cfg(feature = "debug")]
            Self::check(context)?;
            Self::clear(context, clear_color, clear_depth);
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

    pub fn read<F: FnOnce() -> Result<(), Error>>(context: &Context, color_texture: Option<&Texture2D>, depth_texture: Option<&Texture2D>,
                                                  render: F) -> Result<(), Error>
    {
        Self::render(context, if color_texture.is_some() {1} else {0}, |id| {
            if let Some(tex) = color_texture {
                tex.bind_as_color_target(0);
            }
            if let Some(tex) = depth_texture {
                tex.bind_as_depth_target();
            }
            #[cfg(feature = "debug")]
            Self::check(context)?;
            context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(id));
            render()?;
            Ok(())
        })?;
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
        let mask = Self::copy_init(context, color_texture.is_some(), depth_texture.is_some())?;
        Self::read(context,
                           color_texture.map(|(tex, _)| tex),
                           depth_texture.map(|(tex, _)| tex), || {
            Self::write(context, None, None,
                                color_texture.map(|(_, tex)| tex),
                                depth_texture.map(|(_, tex)| tex), || {
                let (source_width, source_height) = if let Some((tex, _)) = color_texture {(tex.width, tex.height)} else {(depth_texture.as_ref().unwrap().0.width, depth_texture.as_ref().unwrap().0.height)};
                let (target_width, target_height) = if let Some((_, tex)) = color_texture {(tex.width, tex.height)} else {(depth_texture.as_ref().unwrap().1.width, depth_texture.as_ref().unwrap().1.height)};
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
        Self::render(context, color_channel_count, |_| {
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
            Self::clear(context, clear_color, clear_depth);
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

    pub fn read_array<F: FnOnce() -> Result<(), Error>>(context: &Context,
                                                        color_texture_array: Option<&Texture2DArray>,
                                                        depth_texture_array: Option<&Texture2DArray>,
                                                        color_channel_count: usize, color_channel_to_texture_layer: &dyn Fn(usize) -> usize,
                                                        depth_layer: usize, render: F) -> Result<(), Error>
    {
        Self::render(context, color_channel_count, |id| {
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
            context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(id));
            render()?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy_from_array(context: &Context,
                           color_texture: Option<(&Texture2DArray, &Texture2D)>,
                           depth_texture: Option<(&Texture2DArray, &Texture2D)>,
                           color_layer: usize,
                           depth_layer: usize,
                           filter: Interpolation) -> Result<(), Error>
    {
        let mask = Self::copy_init(context, color_texture.is_some(), depth_texture.is_some())?;
        Self::read_array(context,
                                 color_texture.map(|(tex, _)| tex),
                                 depth_texture.map(|(tex, _)| tex),
                                 if color_texture.is_some() {1} else {0},
                                 &|_| {color_layer}, depth_layer, || {
                Self::write(context, None, None,
                                    color_texture.map(|(_, tex)| tex),
                                    depth_texture.map(|(_, tex)| tex), || {
                        let (source_width, source_height) = if let Some((tex, _)) = color_texture {(tex.width, tex.height)} else {(depth_texture.as_ref().unwrap().0.width, depth_texture.as_ref().unwrap().0.height)};
                        let (target_width, target_height) = if let Some((_, tex)) = color_texture {(tex.width, tex.height)} else {(depth_texture.as_ref().unwrap().1.width, depth_texture.as_ref().unwrap().1.height)};
                        context.blit_framebuffer(0, 0, source_width as u32, source_height as u32,
                                                 0, 0, target_width as u32, target_height as u32,
                                                 mask, filter as u32);
                        Ok(())
                    })?;
                Ok(())
            })?;
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

    fn copy_init(context: &Context, color_texture_is_some: bool, depth_texture_is_some: bool) -> Result<u32, Error> {
        if !color_texture_is_some && !depth_texture_is_some {
            Err(Error::FailedToCopyFramebuffer {message: "A copy operation must copy either color or depth or both.".to_owned()})?;
        }
        if color_texture_is_some {
            Program::set_color_mask(context, ColorMask::enabled());
        }
        if depth_texture_is_some {
            Program::set_depth(context, None, true);
        }
        let mask = if depth_texture_is_some && color_texture_is_some {consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT} else {
            if depth_texture_is_some { consts::DEPTH_BUFFER_BIT } else {consts::COLOR_BUFFER_BIT}};
        Ok(mask as u32)
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
}*/

fn new_framebuffer(context: &Context) -> Result<crate::context::Framebuffer, Error> {
    Ok(context.create_framebuffer()
        .ok_or_else(|| Error::FailedToCreateFramebuffer {message: "Failed to create framebuffer".to_string()} )?)
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