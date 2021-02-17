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

    pub fn copy_to_screen(&self, viewport: Viewport) -> Result<(), Error>
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

    pub fn copy(&self, other: &Self, filter: Interpolation) -> Result<(), Error>
    {
        if self.color_texture.is_none() || self.depth_texture.is_none() || other.color_texture.is_none() || other.depth_texture.is_none() {
            Err(Error::FailedToCopyFromRenderTarget {message: "Cannot copy depth and color when the render target does not have a color and depth texture.".to_owned()})?;
        }
        Program::set_color_mask(&self.context, ColorMask::enabled());
        Program::set_depth(&self.context, None, true);
        self.bind()?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None, None, || {
            self.context.blit_framebuffer(0, 0, self.color_texture.unwrap().width as u32, self.color_texture.unwrap().height as u32,
                                          0, 0, other.color_texture.unwrap().width as u32, other.color_texture.unwrap().height as u32,
                                          consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy_color(&self, other: &Self, filter: Interpolation) -> Result<(), Error>
    {
        if self.color_texture.is_none() || other.color_texture.is_none() {
            Err(Error::FailedToCopyFromRenderTarget {message: "Cannot copy color when the render target does not have a color texture.".to_owned()})?;
        }
        Program::set_color_mask(&self.context, ColorMask::enabled());
        self.bind()?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None, None, || {
            self.context.blit_framebuffer(0, 0, self.color_texture.unwrap().width as u32, self.color_texture.unwrap().height as u32,
                                          0, 0, other.color_texture.unwrap().width as u32, other.color_texture.unwrap().height as u32,
                                          consts::COLOR_BUFFER_BIT, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy_depth(&self, other: &Self, filter: Interpolation) -> Result<(), Error>
    {
        if self.depth_texture.is_none() || other.depth_texture.is_none() {
            Err(Error::FailedToCopyFromRenderTarget {message: "Cannot copy depth when the render target does not have a depth texture.".to_owned()})?;
        }
        Program::set_depth(&self.context, None, true);
        self.bind()?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None,None, || {
            self.context.blit_framebuffer(0, 0, self.depth_texture.unwrap().width as u32, self.depth_texture.unwrap().height as u32,
                                          0, 0, other.depth_texture.unwrap().width as u32, other.depth_texture.unwrap().height as u32,
                                          consts::DEPTH_BUFFER_BIT, filter as u32);
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
        self.bind(Some(color_layers), Some(depth_layer))?;
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

    pub fn copy_to_screen(&self, color_layer: usize, depth_layer: usize, viewport: Viewport) -> Result<(), Error>
    {
        let effect = get_copy_array_effect(&self.context)?;
        Screen::write(&self.context, None, None,|| {
            if let Some(tex) = self.color_texture {
                effect.program().use_texture(tex, "colorMap")?;
                effect.program().add_uniform_int("colorLayer", &(color_layer as i32))?;
            }
            if let Some(tex) = self.depth_texture {
                effect.program().use_texture(tex, "depthMap")?;
                effect.program().add_uniform_int("depthLayer", &(depth_layer as i32))?;
            }
            effect.apply(RenderStates {cull: CullType::Back, depth_test: DepthTestType::Always, depth_mask: self.depth_texture.is_some(),
                color_mask: if self.color_texture.is_some() {ColorMask::enabled()} else {ColorMask::disabled()}, ..Default::default()}, viewport)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy(&self, color_layer: usize, depth_layer: usize, other: &RenderTarget, filter: Interpolation) -> Result<(), Error>
    {
        if self.color_texture.is_none() || self.depth_texture.is_none() || other.color_texture.is_none() || other.depth_texture.is_none() {
            Err(Error::FailedToCopyFromRenderTarget {message: "Cannot copy depth and color when the render target does not have a color and depth texture.".to_owned()})?;
        }
        Program::set_color_mask(&self.context, ColorMask::enabled());
        Program::set_depth(&self.context, None, true);
        self.bind(Some(&[color_layer]), Some(depth_layer))?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None, None, || {
            self.context.blit_framebuffer(0, 0, self.color_texture.unwrap().width as u32, self.color_texture.unwrap().height as u32,
                                          0, 0, other.color_texture.unwrap().width as u32, other.color_texture.unwrap().height as u32,
                                          consts::DEPTH_BUFFER_BIT | consts::COLOR_BUFFER_BIT, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy_color(&self, color_layer: usize, other: &RenderTarget, filter: Interpolation) -> Result<(), Error>
    {
        if self.color_texture.is_none() || other.color_texture.is_none() {
            Err(Error::FailedToCopyFromRenderTarget {message: "Cannot copy color when the render target does not have a color texture.".to_owned()})?;
        }
        Program::set_color_mask(&self.context, ColorMask::enabled());
        self.bind(Some(&[color_layer]), None)?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None,None, || {
            self.context.blit_framebuffer(0, 0, self.color_texture.unwrap().width as u32, self.color_texture.unwrap().height as u32,
                                          0, 0, other.color_texture.unwrap().width as u32, other.color_texture.unwrap().height as u32,
                                          consts::COLOR_BUFFER_BIT, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    pub fn copy_depth(&self, depth_layer: usize, other: &RenderTarget, filter: Interpolation) -> Result<(), Error>
    {
        if self.depth_texture.is_none() || other.depth_texture.is_none() {
            Err(Error::FailedToCopyFromRenderTarget {message: "Cannot copy depth when the render target does not have a depth texture.".to_owned()})?;
        }
        Program::set_depth(&self.context, None, true);
        self.bind(None, Some(depth_layer))?;
        self.context.bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&self.id));
        other.write(None, None, || {
            self.context.blit_framebuffer(0, 0, self.depth_texture.unwrap().width as u32, self.depth_texture.unwrap().height as u32,
                                          0, 0, other.depth_texture.unwrap().width as u32, other.depth_texture.unwrap().height as u32,
                                          consts::DEPTH_BUFFER_BIT, filter as u32);
            Ok(())
        })?;
        Ok(())
    }

    fn bind(&self, color_layers: Option<&[usize]>, depth_layer: Option<usize>) -> Result<(), Error> {
        self.context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&self.id));
        if let Some(color_texture) = self.color_texture {
            if let Some(color_layers) = color_layers {
                self.context.draw_buffers(&(0..color_layers.len()).map(|i| consts::COLOR_ATTACHMENT0 + i as u32).collect::<Vec<u32>>());
                for channel in 0..color_layers.len() {
                    color_texture.bind_as_color_target(color_layers[channel], channel);
                }
            }
        }
        if let Some(depth_texture) = self.depth_texture {
            if let Some(depth_layer) = depth_layer {
                depth_texture.bind_as_depth_target(depth_layer);
            }
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

fn get_copy_array_effect(context: &Context) -> Result<&ImageEffect, Error>
{
    unsafe {
        static mut COPY_EFFECT: Option<ImageEffect> = None;
        if COPY_EFFECT.is_none() {
            COPY_EFFECT = Some(ImageEffect::new(context, &"
                uniform sampler2DArray colorMap;
                uniform sampler2DArray depthMap;
                uniform int colorLayer;
                uniform int depthLayer;
                in vec2 uv;
                layout (location = 0) out vec4 color;
                void main()
                {
                    color = texture(colorMap, vec3(uv, colorLayer));
                    gl_FragDepth = texture(depthMap, vec3(uv, depthLayer)).r;
                }")?);
        }
        Ok(COPY_EFFECT.as_ref().unwrap())
    }
}