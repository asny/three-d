use crate::core::render_target::*;

pub enum ColorRenderTarget<'a> {
    None,
    Texture2D(&'a mut Texture2D),
    Texture2DArray(&'a mut Texture2DArray, &'a [u32]),
    TextureCubeMap {
        texture: &'a mut TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    },
}

impl<'a> ColorRenderTarget<'a> {
    fn generate_mip_maps(&self) {
        match self {
            Self::Texture2D(tex) => tex.generate_mip_maps(),
            Self::Texture2DArray(tex, _) => tex.generate_mip_maps(),
            Self::TextureCubeMap {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            Self::None => {}
        }
    }

    fn bind(&self, context: &Context) {
        match self {
            Self::Texture2D(tex) => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                tex.bind_as_color_target(0);
            },
            Self::Texture2DArray(tex, layers) => unsafe {
                context.draw_buffers(
                    &(0..layers.len())
                        .map(|i| crate::context::COLOR_ATTACHMENT0 + i as u32)
                        .collect::<Vec<u32>>(),
                );
                for channel in 0..layers.len() {
                    tex.bind_as_color_target(layers[channel], channel as u32);
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
            Self::None => {}
        }
    }

    fn clear_state(&self, clear_state: &mut ClearState) {
        match self {
            Self::None => {
                clear_state.red = None;
                clear_state.green = None;
                clear_state.blue = None;
                clear_state.alpha = None;
            }
            _ => {}
        }
    }
}

pub enum DepthRenderTarget<'a> {
    None,
    Texture2D(&'a mut DepthTargetTexture2D),
    Texture2DArray(&'a mut DepthTargetTexture2DArray, u32),
    TextureCubeMap(&'a mut DepthTargetTextureCubeMap, CubeMapSide),
}

impl<'a> DepthRenderTarget<'a> {
    fn bind(&self, context: &Context) {
        match self {
            Self::Texture2D(tex) => unsafe {
                tex.bind_as_depth_target();
            },
            Self::Texture2DArray(tex, layer) => unsafe {
                tex.bind_as_depth_target(*layer);
            },
            Self::TextureCubeMap(tex, side) => unsafe {
                tex.bind_as_depth_target(*side);
            },
            Self::None => {}
        }
    }

    fn clear_state(&self, clear_state: &mut ClearState) {
        match self {
            Self::None => clear_state.depth = None,
            _ => {}
        }
    }
}

///
/// Adds additional functionality to write to and copy from both a [ColorTargetTexture2D] and
/// a [DepthTargetTexture2D] at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTarget<'a, 'b> {
    context: Context,
    id: Option<crate::context::Framebuffer>,
    color_target: ColorRenderTarget<'a>,
    depth_target: DepthRenderTarget<'b>,
}

impl<'a, 'b> RenderTarget<'a, 'b> {
    ///
    /// Returns the screen render target for this context.
    /// Write to this render target to draw something on the screen.
    ///
    pub fn screen(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: None,
            color_target: ColorRenderTarget::None,
            depth_target: DepthRenderTarget::None,
        })
    }

    ///
    /// Constructs a new render target that enables rendering into the given
    /// [DepthTargetTexture2D].
    ///
    pub fn new_depth(
        context: &Context,
        depth_texture: &'b mut DepthTargetTexture2D,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: Some(new_framebuffer(context)?),
            color_target: ColorRenderTarget::None,
            depth_target: DepthRenderTarget::Texture2D(depth_texture),
        })
    }

    pub fn new_(
        context: &Context,
        color_target: ColorRenderTarget<'a>,
        depth_target: DepthRenderTarget<'b>,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: Some(new_framebuffer(context)?),
            color_target,
            depth_target,
        })
    }

    ///
    /// Constructs a new render target that enables rendering into the given
    /// [ColorTargetTexture2D] and [DepthTargetTexture2D] textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a mut Texture2D,
        depth_texture: &'b mut DepthTargetTexture2D,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: Some(new_framebuffer(context)?),
            color_target: ColorRenderTarget::Texture2D(color_texture),
            depth_target: DepthRenderTarget::Texture2D(depth_texture),
        })
    }

    ///
    /// Constructs a new render target that enables rendering into the given
    /// [ColorTargetTexture2D].
    ///
    pub fn new_color(context: &Context, color_texture: &'a mut Texture2D) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: Some(new_framebuffer(context)?),
            color_target: ColorRenderTarget::Texture2D(color_texture),
            depth_target: DepthRenderTarget::None,
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined at construction.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write(
        &self,
        mut clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        if self.id.is_some() {
            self.color_target.clear_state(&mut clear_state);
            self.depth_target.clear_state(&mut clear_state);
        }
        clear(&self.context, &clear_state);
        render()?;
        self.color_target.generate_mip_maps();
        self.context.error_check()
    }

    ///
    /// Returns the values of the pixels in this texture inside the given viewport.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_color<T: TextureDataType>(&self, viewport: Viewport) -> ThreeDResult<Vec<T>> {
        if let ColorRenderTarget::None = self.color_target {
            if self.id.is_some() {
                Err(CoreError::RenderTargetRead("color".to_string()))?;
            }
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut data_size = std::mem::size_of::<T>();
        // On web, the format needs to be RGBA if the data type is byte.
        if data_size / T::size() as usize == 1 {
            data_size *= 4 / T::size() as usize
        }
        let mut pixels = vec![0u8; viewport.width as usize * viewport.height as usize * data_size];
        unsafe {
            self.context.read_pixels(
                viewport.x as i32,
                viewport.y as i32,
                viewport.width as i32,
                viewport.height as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelPackData::Slice(&mut pixels),
            );
            self.context.error_check()?;
            Ok(from_byte_slice(&pixels).to_vec())
        }
    }

    ///
    /// Returns the depth values from the screen as a list of 32-bit floats.
    /// Only available on desktop.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self, viewport: Viewport) -> ThreeDResult<Vec<f32>> {
        if let DepthRenderTarget::None = self.depth_target {
            if self.id.is_some() {
                Err(CoreError::RenderTargetRead("depth".to_string()))?;
            }
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut pixels = vec![0u8; viewport.width as usize * viewport.height as usize * 4];
        unsafe {
            self.context.read_pixels(
                viewport.x as i32,
                viewport.y as i32,
                viewport.width as i32,
                viewport.height as i32,
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
    ) -> ThreeDResult<()> {
        self.write(ClearState::none(), || {
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
    ) -> ThreeDResult<()> {
        self.write(ClearState::none(), || {
            copy_from_array(
                &self.context,
                color_texture,
                depth_texture,
                viewport,
                write_mask,
            )
        })
    }

    pub(in crate::core) fn bind(&self, target: u32) -> ThreeDResult<()> {
        unsafe {
            self.context.bind_framebuffer(target, self.id);
        }
        self.color_target.bind(&self.context);
        self.depth_target.bind(&self.context);
        self.context.framebuffer_check()?;
        self.context.error_check()
    }
}

impl Drop for RenderTarget<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            if let Some(id) = self.id {
                self.context.delete_framebuffer(id);
            }
        }
    }
}
