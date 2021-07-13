use crate::context::{consts, Context, DataType};
use crate::core::*;
use crate::definition::*;
use crate::math::*;
use crate::ImageEffect;

///
/// Defines which channels (red, green, blue, alpha and depth) to clear when starting to write to a
/// [render target](crate::RenderTarget) or the [screen](crate::Screen).
/// If `None` then the channel is not cleared and if `Some(value)` the channel is cleared to that value (the value must be between 0 and 1).
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClearState {
    /// Defines the clear value for the red channel.
    pub red: Option<f32>,
    /// Defines the clear value for the green channel.
    pub green: Option<f32>,
    /// Defines the clear value for the blue channel.
    pub blue: Option<f32>,
    /// Defines the clear value for the alpha channel.
    pub alpha: Option<f32>,
    /// Defines the clear value for the depth channel. A value of 1 means a depth value equal to the far plane and 0 means a depth value equal to the near plane.
    pub depth: Option<f32>,
}

impl ClearState {
    ///
    /// Nothing will be cleared.
    ///
    pub const fn none() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
            alpha: None,
            depth: None,
        }
    }

    ///
    /// The depth will be cleared to the given value.
    ///
    pub const fn depth(depth: f32) -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
            alpha: None,
            depth: Some(depth),
        }
    }

    ///
    /// The color channels (red, green, blue and alpha) will be cleared to the given values.
    ///
    pub const fn color(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red: Some(red),
            green: Some(green),
            blue: Some(blue),
            alpha: Some(alpha),
            depth: None,
        }
    }

    ///
    /// Both the color channels (red, green, blue and alpha) and depth will be cleared to the given values.
    ///
    pub const fn color_and_depth(red: f32, green: f32, blue: f32, alpha: f32, depth: f32) -> Self {
        Self {
            red: Some(red),
            green: Some(green),
            blue: Some(blue),
            alpha: Some(alpha),
            depth: Some(depth),
        }
    }
}

impl Default for ClearState {
    fn default() -> Self {
        Self::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0)
    }
}

///
/// The screen render target which is essential to get something on the screen (see the [write function](Screen::write)).
///
pub struct Screen {}

impl Screen {
    ///
    /// Call this function and make a render call (for example on some [object](crate::object))
    /// in the `render` closure to render something to the screen.
    /// Before writing, the screen is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        context: &Context,
        clear_state: ClearState,
        render: F,
    ) -> Result<(), Error> {
        context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, None);
        clear(context, &clear_state);
        render()?;
        Ok(())
    }

    ///
    /// Returns the RGBA color values from the screen as a list of bytes (one byte for each color channel).
    ///
    pub fn read_color(context: &Context, viewport: Viewport) -> Result<Vec<u8>, Error> {
        let mut pixels = vec![0u8; viewport.width as usize * viewport.height as usize * 4];
        context.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        context.read_pixels_with_u8_data(
            viewport.x as u32,
            viewport.y as u32,
            viewport.width,
            viewport.height,
            consts::RGBA,
            DataType::UnsignedByte,
            &mut pixels,
        );
        Ok(pixels)
    }

    ///
    /// Returns the depth values from the screen as a list of 32-bit floats.
    /// Only available on desktop.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(context: &Context, viewport: Viewport) -> Result<Vec<f32>, Error> {
        let mut pixels = vec![0f32; viewport.width as usize * viewport.height as usize];
        context.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        context.read_pixels_with_f32_data(
            viewport.x as u32,
            viewport.y as u32,
            viewport.width,
            viewport.height,
            consts::DEPTH_COMPONENT,
            DataType::Float,
            &mut pixels,
        );
        Ok(pixels)
    }
}

///
/// The destination of applying a copy.
///
pub enum CopyDestination<'a, 'b, 'c, 'd, T: TextureDataType> {
    /// Copies to the [Screen](crate::Screen).
    Screen,
    /// Copies to a [ColorTargetTexture2D](crate::ColorTargetTexture2D).
    ColorTexture(&'d ColorTargetTexture2D<T>),
    /// Copies to a [DepthTargetTexture2D](crate::DepthTargetTexture2D).
    DepthTexture(&'d DepthTargetTexture2D),
    /// Copies to a [RenderTarget](crate::RenderTarget).
    RenderTarget(&'c RenderTarget<'a, 'b, T>),
}

///
/// Adds additional functionality to write to and copy from both a [ColorTargetTexture2D](crate::ColorTargetTexture2D) and
/// a [DepthTargetTexture2D](crate::DepthTargetTexture2D) at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTarget<'a, 'b, T: TextureDataType> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a ColorTargetTexture2D<T>>,
    depth_texture: Option<&'b DepthTargetTexture2D>,
}

impl<'a, 'b, T: TextureDataType> RenderTarget<'a, 'b, T> {
    ///
    /// Constructs a new render target that enables rendering into the given
    /// [color](crate::ColorTargetTexture2D) and [depth](DepthTargetTexture2D) textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a ColorTargetTexture2D<T>,
        depth_texture: &'b DepthTargetTexture2D,
    ) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined at construction.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        clear_state: ClearState,
        render: F,
    ) -> Result<(), Error> {
        self.bind(consts::DRAW_FRAMEBUFFER)?;
        clear(
            &self.context,
            &ClearState {
                red: self.color_texture.and(clear_state.red),
                green: self.color_texture.and(clear_state.green),
                blue: self.color_texture.and(clear_state.blue),
                alpha: self.color_texture.and(clear_state.alpha),
                depth: self.depth_texture.and(clear_state.depth),
            },
        );
        render()?;
        if let Some(color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        Ok(())
    }

    ///
    /// Copies the content of the color and depth textures in this render target to the specified viewport of the specified [destination](crate::CopyDestination).
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_to(
        &self,
        destination: CopyDestination<T>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> Result<(), Error> {
        let copy = || {
            let effect = get_copy_effect(&self.context)?;
            if let Some(tex) = self.color_texture {
                effect.use_texture("colorMap", tex)?;
            }
            if let Some(tex) = self.depth_texture {
                effect.use_texture("depthMap", tex)?;
            }
            effect.apply(
                RenderStates {
                    depth_test: DepthTestType::Always,
                    write_mask,
                    ..Default::default()
                },
                viewport,
            )?;
            Ok(())
        };
        match destination {
            CopyDestination::RenderTarget(other) => {
                other.write(ClearState::none(), copy)?;
            }
            CopyDestination::Screen => {
                Screen::write(&self.context, ClearState::none(), copy)?;
            }
            CopyDestination::ColorTexture(tex) => {
                if self.color_texture.is_none() {
                    Err(Error::RenderTargetError {
                        message: "Cannot copy color from a depth texture.".to_owned(),
                    })?;
                }
                tex.write(ClearState::none(), copy)?;
            }
            CopyDestination::DepthTexture(tex) => {
                if self.depth_texture.is_none() {
                    Err(Error::RenderTargetError {
                        message: "Cannot copy depth from a color texture.".to_owned(),
                    })?;
                }
                tex.write(None, copy)?;
            }
        }
        Ok(())
    }

    pub(super) fn new_color(
        context: &Context,
        color_texture: &'a ColorTargetTexture2D<T>,
    ) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

    pub(super) fn new_depth(
        context: &Context,
        depth_texture: &'b DepthTargetTexture2D,
    ) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture),
        })
    }

    pub(super) fn bind(&self, target: u32) -> Result<(), Error> {
        self.context.bind_framebuffer(target, Some(&self.id));
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

impl<T: TextureDataType> Drop for RenderTarget<'_, '_, T> {
    fn drop(&mut self) {
        self.context.delete_framebuffer(Some(&self.id));
    }
}

///
/// Adds additional functionality to write to and copy from both a [ColorTargetTexture2DArray](crate::ColorTargetTexture2DArray) and
/// a [DepthTargetTexture2DArray](crate::DepthTargetTexture2DArray) at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTargetArray<'a, 'b, T: TextureDataType> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a ColorTargetTexture2DArray<T>>,
    depth_texture: Option<&'b DepthTargetTexture2DArray>,
}

impl<'a, 'b, T: TextureDataType> RenderTargetArray<'a, 'b, T> {
    ///
    /// Constructs a new render target array that enables rendering into the given
    /// [color](crate::ColorTargetTexture2DArray) and [depth](DepthTargetTexture2DArray) array textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a ColorTargetTexture2DArray<T>,
        depth_texture: &'b DepthTargetTexture2DArray,
    ) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture),
        })
    }

    pub(super) fn new_color(
        context: &Context,
        color_texture: &'a ColorTargetTexture2DArray<T>,
    ) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

    pub(super) fn new_depth(
        context: &Context,
        depth_texture: &'b DepthTargetTexture2DArray,
    ) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined at construction
    /// and defined by the input parameters `color_layers` and `depth_layer`.
    /// Output at location *i* defined in the fragment shader is written to the color texture layer at the *ith* index in `color_layers`.
    /// The depth is written to the depth texture defined by `depth_layer`.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        color_layers: &[u32],
        depth_layer: u32,
        clear_state: ClearState,
        render: F,
    ) -> Result<(), Error> {
        self.bind(Some(color_layers), Some(depth_layer))?;
        clear(
            &self.context,
            &ClearState {
                red: self.color_texture.and(clear_state.red),
                green: self.color_texture.and(clear_state.green),
                blue: self.color_texture.and(clear_state.blue),
                alpha: self.color_texture.and(clear_state.alpha),
                depth: self.depth_texture.and(clear_state.depth),
            },
        );
        render()?;
        if let Some(color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        Ok(())
    }

    ///
    /// Copies the content of the specified color and depth layers in this render target to the given viewport of the given [destination](crate::CopyDestination).
    /// Only copies the channels specified by the write mask.
    ///
    pub fn copy_to(
        &self,
        color_layer: u32,
        depth_layer: u32,
        destination: CopyDestination<T>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> Result<(), Error> {
        let copy = || {
            let effect = get_copy_array_effect(&self.context)?;
            if let Some(tex) = self.color_texture {
                effect.use_texture_array("colorMap", tex)?;
                effect.use_uniform_int("colorLayer", &(color_layer as i32))?;
            }
            if let Some(tex) = self.depth_texture {
                effect.use_texture_array("depthMap", tex)?;
                effect.use_uniform_int("depthLayer", &(depth_layer as i32))?;
            }
            effect.apply(
                RenderStates {
                    depth_test: DepthTestType::Always,
                    write_mask,
                    ..Default::default()
                },
                viewport,
            )?;
            Ok(())
        };
        match destination {
            CopyDestination::RenderTarget(other) => {
                other.write(ClearState::none(), copy)?;
            }
            CopyDestination::Screen => {
                Screen::write(&self.context, ClearState::none(), copy)?;
            }
            CopyDestination::ColorTexture(tex) => {
                if self.color_texture.is_none() {
                    Err(Error::RenderTargetError {
                        message: "Cannot copy color from a depth texture.".to_owned(),
                    })?;
                }
                tex.write(ClearState::none(), copy)?;
            }
            CopyDestination::DepthTexture(tex) => {
                if self.depth_texture.is_none() {
                    Err(Error::RenderTargetError {
                        message: "Cannot copy depth from a color texture.".to_owned(),
                    })?;
                }
                tex.write(None, copy)?;
            }
        }
        Ok(())
    }

    fn bind(&self, color_layers: Option<&[u32]>, depth_layer: Option<u32>) -> Result<(), Error> {
        self.context
            .bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&self.id));
        if let Some(color_texture) = self.color_texture {
            if let Some(color_layers) = color_layers {
                self.context.draw_buffers(
                    &(0..color_layers.len())
                        .map(|i| consts::COLOR_ATTACHMENT0 + i as u32)
                        .collect::<Vec<u32>>(),
                );
                for channel in 0..color_layers.len() {
                    color_texture.bind_as_color_target(color_layers[channel], channel as u32);
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

impl<T: TextureDataType> Drop for RenderTargetArray<'_, '_, T> {
    fn drop(&mut self) {
        self.context.delete_framebuffer(Some(&self.id));
    }
}

fn new_framebuffer(context: &Context) -> Result<crate::context::Framebuffer, Error> {
    Ok(context
        .create_framebuffer()
        .ok_or_else(|| Error::RenderTargetError {
            message: "Failed to create framebuffer".to_string(),
        })?)
}

#[cfg(feature = "debug")]
fn check(context: &Context) -> Result<(), Error> {
    context.check_framebuffer_status().or_else(|status| {
        Err(Error::RenderTargetError {
            message: format!("Failed to create frame buffer: {}", status),
        })
    })
}

fn clear(context: &Context, clear_state: &ClearState) {
    Program::set_write_mask(
        context,
        WriteMask {
            red: clear_state.red.is_some(),
            green: clear_state.green.is_some(),
            blue: clear_state.blue.is_some(),
            alpha: clear_state.alpha.is_some(),
            depth: clear_state.depth.is_some(),
        },
    );
    let clear_color = clear_state.red.is_some()
        || clear_state.green.is_some()
        || clear_state.blue.is_some()
        || clear_state.alpha.is_some();
    if clear_color {
        context.clear_color(
            clear_state.red.unwrap_or(0.0),
            clear_state.green.unwrap_or(0.0),
            clear_state.blue.unwrap_or(0.0),
            clear_state.alpha.unwrap_or(1.0),
        );
    }
    if let Some(depth) = clear_state.depth {
        context.clear_depth(depth);
    }
    context.clear(if clear_color && clear_state.depth.is_some() {
        consts::COLOR_BUFFER_BIT | consts::DEPTH_BUFFER_BIT
    } else {
        if clear_color {
            consts::COLOR_BUFFER_BIT
        } else {
            consts::DEPTH_BUFFER_BIT
        }
    });
}

fn get_copy_effect(context: &Context) -> Result<&ImageEffect, Error> {
    unsafe {
        static mut COPY_EFFECT: Option<ImageEffect> = None;
        if COPY_EFFECT.is_none() {
            COPY_EFFECT = Some(ImageEffect::new(
                context,
                &"
                uniform sampler2D colorMap;
                uniform sampler2D depthMap;
                in vec2 uv;
                layout (location = 0) out vec4 color;
                void main()
                {
                    color = texture(colorMap, uv);
                    gl_FragDepth = texture(depthMap, uv).r;
                }",
            )?);
        }
        Ok(COPY_EFFECT.as_ref().unwrap())
    }
}

fn get_copy_array_effect(context: &Context) -> Result<&ImageEffect, Error> {
    unsafe {
        static mut COPY_EFFECT: Option<ImageEffect> = None;
        if COPY_EFFECT.is_none() {
            COPY_EFFECT = Some(ImageEffect::new(
                context,
                &"
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
                }",
            )?);
        }
        Ok(COPY_EFFECT.as_ref().unwrap())
    }
}
