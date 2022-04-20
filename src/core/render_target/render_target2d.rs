use crate::core::render_target::*;

pub enum ColorTarget<'a> {
    Texture2D {
        texture: &'a mut Texture2D,
        mip_level: Option<u32>,
    },
    Texture2DArray {
        texture: &'a mut Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    },
    TextureCubeMap {
        texture: &'a mut TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    },
}

impl<'a> ColorTarget<'a> {
    fn width(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.width(),
            Self::Texture2DArray { texture, .. } => texture.width(),
            Self::TextureCubeMap { texture, .. } => texture.width(),
        }
    }

    fn height(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.height(),
            Self::Texture2DArray { texture, .. } => texture.height(),
            Self::TextureCubeMap { texture, .. } => texture.height(),
        }
    }

    fn generate_mip_maps(&self) {
        match self {
            Self::Texture2D { texture, mip_level } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            Self::Texture2DArray {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            Self::TextureCubeMap {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
        }
    }

    fn bind(&self, context: &Context) {
        match self {
            Self::Texture2D { texture, mip_level } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(0, mip_level.unwrap_or(0));
            },
            Self::Texture2DArray {
                texture,
                layers,
                mip_level,
            } => unsafe {
                context.draw_buffers(
                    &(0..layers.len())
                        .map(|i| crate::context::COLOR_ATTACHMENT0 + i as u32)
                        .collect::<Vec<u32>>(),
                );
                for channel in 0..layers.len() {
                    texture.bind_as_color_target(
                        layers[channel],
                        channel as u32,
                        mip_level.unwrap_or(0),
                    );
                }
            },
            Self::TextureCubeMap {
                texture,
                side,
                mip_level,
            } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(*side, 0, mip_level.unwrap_or(0));
            },
        }
    }
}

pub enum DepthTarget<'a> {
    Texture2D {
        texture: &'a mut DepthTargetTexture2D,
    },
    Texture2DArray {
        texture: &'a mut DepthTargetTexture2DArray,
        layer: u32,
    },
    TextureCubeMap {
        texture: &'a mut DepthTargetTextureCubeMap,
        side: CubeMapSide,
    },
}

impl<'a> DepthTarget<'a> {
    fn width(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.width(),
            Self::Texture2DArray { texture, .. } => texture.width(),
            Self::TextureCubeMap { texture, .. } => texture.width(),
        }
    }

    fn height(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.height(),
            Self::Texture2DArray { texture, .. } => texture.height(),
            Self::TextureCubeMap { texture, .. } => texture.height(),
        }
    }

    fn bind(&self) {
        match self {
            Self::Texture2D { texture } => {
                texture.bind_as_depth_target();
            }
            Self::Texture2DArray { texture, layer } => {
                texture.bind_as_depth_target(*layer);
            }
            Self::TextureCubeMap { texture, side } => {
                texture.bind_as_depth_target(*side);
            }
        }
    }
}

enum Target<'a> {
    Screen {
        width: u32,
        height: u32,
    },
    Color {
        id: Framebuffer,
        color: ColorTarget<'a>,
    },
    Depth {
        id: Framebuffer,
        depth: DepthTarget<'a>,
    },
    ColorAndDepth {
        id: Framebuffer,
        color: ColorTarget<'a>,
        depth: DepthTarget<'a>,
    },
}

impl<'a> Target<'a> {
    pub(in crate::core) fn bind(&self, context: &Context, target: u32) -> ThreeDResult<()> {
        match self {
            Target::ColorAndDepth { color, depth, id } => unsafe {
                context.bind_framebuffer(target, Some(*id));
                color.bind(context);
                depth.bind();
            },
            Target::Color { color, id } => unsafe {
                context.bind_framebuffer(target, Some(*id));
                color.bind(context);
            },
            Target::Depth { depth, id } => unsafe {
                context.bind_framebuffer(target, Some(*id));
                depth.bind();
            },
            Target::Screen { .. } => unsafe {
                context.bind_framebuffer(target, None);
            },
        };
        context.framebuffer_check()?;
        context.error_check()
    }

    fn width(&self) -> u32 {
        match self {
            Self::ColorAndDepth { color, .. } => color.width(),
            Self::Color { color, .. } => color.width(),
            Self::Depth { depth, .. } => depth.width(),
            Self::Screen { width, .. } => *width,
        }
    }

    fn height(&self) -> u32 {
        match self {
            Self::ColorAndDepth { color, .. } => color.height(),
            Self::Color { color, .. } => color.height(),
            Self::Depth { depth, .. } => depth.height(),
            Self::Screen { height, .. } => *height,
        }
    }
}

use crate::context::Framebuffer;
///
/// Adds additional functionality to read from and write to the screen (see [RenderTarget::screen]) or a color texture and
/// a depth texture (see [RenderTarget::new]) at the same time.
/// If you only want to perform an operation on either a color texture or depth texture,
/// use the `render_target` function directly on the texture structs (for example [Texture2D]) or the [RenderTarget::new_color] and [RenderTarget::new_depth] functions.
/// A render target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the textures.
///
pub struct RenderTarget<'a> {
    target: Target<'a>,
    context: Context,
}

impl<'a> RenderTarget<'a> {
    ///
    /// Returns the screen render target for this context.
    /// Write to this render target to draw something on the screen.
    ///
    pub fn screen(context: &Context, width: u32, height: u32) -> Self {
        Self {
            context: context.clone(),
            target: Target::Screen { width, height },
        }
    }

    ///
    /// Constructs a new render target that enables rendering into the given [ColorTarget] and [DepthTarget].
    ///
    pub fn new(
        context: &Context,
        color: ColorTarget<'a>,
        depth: DepthTarget<'a>,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            target: Target::ColorAndDepth {
                id: new_framebuffer(context)?,
                color,
                depth,
            },
        })
    }

    pub fn new_color(context: &Context, color: ColorTarget<'a>) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            target: Target::Color {
                id: new_framebuffer(context)?,
                color,
            },
        })
    }

    pub fn new_depth(context: &Context, depth: DepthTarget<'a>) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            target: Target::Depth {
                id: new_framebuffer(context)?,
                depth,
            },
        })
    }

    pub fn clear(&self, color: Color, depth: f32) -> ThreeDResult<&Self> {
        self.clear_area(self.area(), color, depth)
    }

    pub fn clear_area(&self, area: Viewport, color: Color, depth: f32) -> ThreeDResult<&Self> {
        self.clear_internal(area, Some(color), Some(depth))?;
        Ok(self)
    }

    pub fn clear_color(&self, color: Color) -> ThreeDResult<&Self> {
        self.clear_color_area(self.area(), color)
    }

    pub fn clear_color_area(&self, area: Viewport, color: Color) -> ThreeDResult<&Self> {
        self.clear_internal(area, Some(color), None)?;
        Ok(self)
    }

    pub fn clear_depth(&self, depth: f32) -> ThreeDResult<&Self> {
        self.clear_depth_area(self.area(), depth)
    }

    pub fn clear_depth_area(&self, area: Viewport, depth: f32) -> ThreeDResult<&Self> {
        self.clear_internal(area, None, Some(depth))?;
        Ok(self)
    }

    #[allow(deprecated)]
    fn clear_internal(
        &self,
        area: Viewport,
        color: Option<Color>,
        depth: Option<f32>,
    ) -> ThreeDResult<()> {
        let clear_state = if let Some(color) = color {
            if let Some(depth) = depth {
                match self.target {
                    Target::Depth { .. } => ClearState::depth(depth),
                    Target::Color { .. } => ClearState::color(
                        color.r as f32 / 255.0,
                        color.g as f32 / 255.0,
                        color.b as f32 / 255.0,
                        color.a as f32 / 255.0,
                    ),
                    _ => ClearState::color_and_depth(
                        color.r as f32 / 255.0,
                        color.g as f32 / 255.0,
                        color.b as f32 / 255.0,
                        color.a as f32 / 255.0,
                        depth,
                    ),
                }
            } else {
                match self.target {
                    Target::Depth { .. } => ClearState::none(),
                    _ => ClearState::color(
                        color.r as f32 / 255.0,
                        color.g as f32 / 255.0,
                        color.b as f32 / 255.0,
                        color.a as f32 / 255.0,
                    ),
                }
            }
        } else {
            match self.target {
                Target::Color { .. } => ClearState::none(),
                _ => ClearState::depth(depth.unwrap()),
            }
        };
        set_scissor(&self.context, area);
        self.target
            .bind(&self.context, crate::context::DRAW_FRAMEBUFFER)?;
        clear(&self.context, &clear_state);
        self.context.error_check()?;
        Ok(())
    }

    #[allow(deprecated)]
    pub(in crate::core) fn clear_deprecated(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        set_scissor(&self.context, self.area());
        self.target
            .bind(&self.context, crate::context::DRAW_FRAMEBUFFER)?;
        clear(&self.context, &clear_state);
        self.context.error_check()?;
        Ok(self)
    }

    ///
    /// Renders whatever rendered in the `render` closure into this render target.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_to_viewport(self.area(), render)
    }

    pub fn write_to_viewport(
        &self,
        area: Viewport,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        set_scissor(&self.context, area);
        self.target
            .bind(&self.context, crate::context::DRAW_FRAMEBUFFER)?;
        render()?;
        if let Target::Color { ref color, .. } = self.target {
            color.generate_mip_maps();
        } else if let Target::ColorAndDepth { ref color, .. } = self.target {
            color.generate_mip_maps();
        }
        self.context.error_check()?;
        Ok(self)
    }

    ///
    /// Returns the colors of the pixels in this render target.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_color<T: TextureDataType>(&self) -> ThreeDResult<Vec<T>> {
        self.read_color_area(self.area())
    }

    ///
    /// Returns the colors of the pixels in this render target inside the given viewport.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_color_area<T: TextureDataType>(&self, area: Viewport) -> ThreeDResult<Vec<T>> {
        if let Target::Depth { .. } = self.target {
            Err(CoreError::RenderTargetRead("color".to_string()))?;
        }
        self.target
            .bind(&self.context, crate::context::DRAW_FRAMEBUFFER)?;
        self.target
            .bind(&self.context, crate::context::READ_FRAMEBUFFER)?;
        let mut data_size = std::mem::size_of::<T>();
        // On web, the format needs to be RGBA if the data type is byte.
        if data_size / T::size() as usize == 1 {
            data_size *= 4 / T::size() as usize
        }
        let mut bytes = vec![0u8; area.width as usize * area.height as usize * data_size];
        unsafe {
            self.context.read_pixels(
                area.x as i32,
                area.y as i32,
                area.width as i32,
                area.height as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelPackData::Slice(&mut bytes),
            );
            self.context.error_check()?;
        }
        let mut pixels = from_byte_slice(&bytes).to_vec();
        flip_y(&mut pixels, area.width as usize, area.height as usize);
        Ok(pixels)
    }

    ///
    /// Returns the depth values from the render target as a list of 32-bit floats.
    /// Not available on web.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self) -> ThreeDResult<Vec<f32>> {
        self.read_depth_area(self.area())
    }

    ///
    /// Returns the depth values from the given area of the render target as a list of 32-bit floats.
    /// Not available on web.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth_area(&self, area: Viewport) -> ThreeDResult<Vec<f32>> {
        if let Target::Color { .. } = self.target {
            Err(CoreError::RenderTargetRead("depth".to_string()))?;
        }
        self.target
            .bind(&self.context, crate::context::DRAW_FRAMEBUFFER)?;
        self.target
            .bind(&self.context, crate::context::READ_FRAMEBUFFER)?;
        let mut pixels = vec![0u8; area.width as usize * area.height as usize * 4];
        unsafe {
            self.context.read_pixels(
                area.x as i32,
                area.y as i32,
                area.width as i32,
                area.height as i32,
                crate::context::DEPTH_COMPONENT,
                crate::context::FLOAT,
                crate::context::PixelPackData::Slice(&mut pixels),
            );
        }
        self.context.error_check()?;
        Ok(from_byte_slice(&pixels).to_vec())
    }

    ///
    /// Copies the content of the color and depth texture to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from(
        &self,
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<&Self> {
        self.write(|| {
            copy_from(
                &self.context,
                color_texture,
                depth_texture,
                viewport,
                write_mask,
            )
        })
    }

    ///
    /// Copies the content of the given layers of the color and depth array textures to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from_array(
        &self,
        color_texture: Option<(&Texture2DArray, u32)>,
        depth_texture: Option<(&DepthTargetTexture2DArray, u32)>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<&Self> {
        self.write(|| {
            copy_from_array(
                &self.context,
                color_texture,
                depth_texture,
                viewport,
                write_mask,
            )
        })
    }

    pub fn area(&self) -> Viewport {
        Viewport::new_at_origo(self.target.width(), self.target.height())
    }
}

impl Drop for RenderTarget<'_> {
    fn drop(&mut self) {
        unsafe {
            match self.target {
                Target::ColorAndDepth { id, .. } => self.context.delete_framebuffer(id),
                Target::Color { id, .. } => self.context.delete_framebuffer(id),
                Target::Depth { id, .. } => self.context.delete_framebuffer(id),
                Target::Screen { .. } => {}
            }
        }
    }
}
