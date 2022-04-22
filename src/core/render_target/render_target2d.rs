use crate::core::render_target::*;

#[derive(Clone)]
pub struct ColorTarget<'a>(CT<'a>);

#[derive(Clone)]
enum CT<'a> {
    Texture2D {
        texture: &'a Texture2D,
        mip_level: Option<u32>,
    },
    Texture2DArray {
        texture: &'a Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    },
    TextureCubeMap {
        texture: &'a TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    },
    Screen {
        width: u32,
        height: u32,
        context: Context,
    },
}

impl<'a> ColorTarget<'a> {
    fn screen(context: &Context, width: u32, height: u32) -> Self {
        ColorTarget(CT::Screen {
            context: context.clone(),
            width,
            height,
        })
    }

    pub(in crate::core) fn new_texture2d(texture: &'a Texture2D, mip_level: Option<u32>) -> Self {
        ColorTarget(CT::Texture2D { texture, mip_level })
    }

    pub(in crate::core) fn new_texture_cube_map(
        texture: &'a TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget(CT::TextureCubeMap {
            texture,
            side,
            mip_level,
        })
    }

    pub(in crate::core) fn new_texture_2d_array(
        texture: &'a Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget(CT::Texture2DArray {
            texture,
            layers,
            mip_level,
        })
    }

    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_in_viewport(self.viewport(), clear_state)
    }

    pub fn clear_in_viewport(
        &self,
        viewport: Viewport,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?.clear_in_viewport(
            viewport,
            ClearState {
                depth: None,
                ..clear_state
            },
        )?;
        Ok(self)
    }

    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_in_viewport(self.viewport(), render)
    }

    pub fn write_in_viewport(
        &self,
        viewport: Viewport,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?
            .write_in_viewport(viewport, render)?;
        Ok(self)
    }

    pub fn read<T: TextureDataType>(&self) -> ThreeDResult<Vec<T>> {
        self.read_in_viewport(self.viewport())
    }

    pub fn read_in_viewport<T: TextureDataType>(&self, viewport: Viewport) -> ThreeDResult<Vec<T>> {
        self.as_render_target()?.read_color_in_viewport(viewport)
    }

    pub fn width(&self) -> u32 {
        match self.0 {
            CT::Texture2D { texture, mip_level } => size_with_mip(texture.width(), mip_level),
            CT::Texture2DArray {
                texture, mip_level, ..
            } => size_with_mip(texture.width(), mip_level),
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => size_with_mip(texture.width(), mip_level),
            CT::Screen { width, .. } => width,
        }
    }

    pub fn height(&self) -> u32 {
        match self.0 {
            CT::Texture2D { texture, mip_level } => size_with_mip(texture.height(), mip_level),
            CT::Texture2DArray {
                texture, mip_level, ..
            } => size_with_mip(texture.height(), mip_level),
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => size_with_mip(texture.height(), mip_level),
            CT::Screen { height, .. } => height,
        }
    }

    pub fn viewport(&self) -> Viewport {
        Viewport::new_at_origo(self.width(), self.height())
    }

    pub(in crate::core) fn as_render_target(&self) -> ThreeDResult<RenderTarget<'a>> {
        let context = match &self.0 {
            CT::Texture2D { texture, .. } => &texture.context,
            CT::Texture2DArray { texture, .. } => &texture.context,
            CT::TextureCubeMap { texture, .. } => &texture.context,
            CT::Screen { context, .. } => context,
        };
        RenderTarget::new_color(context, self.clone())
    }

    fn generate_mip_maps(&self) {
        match self.0 {
            CT::Texture2D { texture, mip_level } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            CT::Texture2DArray {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            _ => {}
        }
    }

    fn bind(&self, context: &Context) {
        match self.0 {
            CT::Texture2D { texture, mip_level } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(0, mip_level.unwrap_or(0));
            },
            CT::Texture2DArray {
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
            CT::TextureCubeMap {
                texture,
                side,
                mip_level,
            } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(side, 0, mip_level.unwrap_or(0));
            },
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct DepthTarget<'a>(DT<'a>);

#[derive(Clone)]
enum DT<'a> {
    Texture2D {
        texture: &'a DepthTargetTexture2D,
    },
    Texture2DArray {
        texture: &'a DepthTargetTexture2DArray,
        layer: u32,
    },
    TextureCubeMap {
        texture: &'a DepthTargetTextureCubeMap,
        side: CubeMapSide,
    },
    Screen {
        width: u32,
        height: u32,
        context: Context,
    },
}

impl<'a> DepthTarget<'a> {
    fn screen(context: &Context, width: u32, height: u32) -> Self {
        Self(DT::Screen {
            context: context.clone(),
            width,
            height,
        })
    }

    pub(in crate::core) fn new_texture2d(texture: &'a DepthTargetTexture2D) -> Self {
        Self(DT::Texture2D { texture })
    }

    pub(in crate::core) fn new_texture_cube_map(
        texture: &'a DepthTargetTextureCubeMap,
        side: CubeMapSide,
    ) -> Self {
        Self(DT::TextureCubeMap { texture, side })
    }

    pub(in crate::core) fn new_texture_2d_array(
        texture: &'a DepthTargetTexture2DArray,
        layer: u32,
    ) -> Self {
        Self(DT::Texture2DArray { texture, layer })
    }

    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_in_viewport(self.viewport(), clear_state)
    }

    pub fn clear_in_viewport(
        &self,
        viewport: Viewport,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?.clear_in_viewport(
            viewport,
            ClearState {
                depth: clear_state.depth,
                ..ClearState::none()
            },
        )?;
        Ok(self)
    }

    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_in_viewport(self.viewport(), render)
    }

    pub fn write_in_viewport(
        &self,
        viewport: Viewport,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?
            .write_in_viewport(viewport, render)?;
        Ok(self)
    }

    ///
    /// Returns the depth values from the render target as a list of 32-bit floats.
    /// Not available on web.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read(&self) -> ThreeDResult<Vec<f32>> {
        self.read_in_viewport(self.viewport())
    }

    ///
    /// Returns the depth values from the given viewport of the render target as a list of 32-bit floats.
    /// Not available on web.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_in_viewport(&self, viewport: Viewport) -> ThreeDResult<Vec<f32>> {
        self.as_render_target()?.read_depth_in_viewport(viewport)
    }

    fn as_render_target(&self) -> ThreeDResult<RenderTarget<'a>> {
        let context = match &self.0 {
            DT::Texture2D { texture, .. } => &texture.context,
            DT::Texture2DArray { texture, .. } => &texture.context,
            DT::TextureCubeMap { texture, .. } => &texture.context,
            DT::Screen { context, .. } => context,
        };
        RenderTarget::new_depth(context, self.clone())
    }

    pub fn width(&self) -> u32 {
        match &self.0 {
            DT::Texture2D { texture, .. } => texture.width(),
            DT::Texture2DArray { texture, .. } => texture.width(),
            DT::TextureCubeMap { texture, .. } => texture.width(),
            DT::Screen { width, .. } => *width,
        }
    }

    pub fn height(&self) -> u32 {
        match &self.0 {
            DT::Texture2D { texture, .. } => texture.height(),
            DT::Texture2DArray { texture, .. } => texture.height(),
            DT::TextureCubeMap { texture, .. } => texture.height(),
            DT::Screen { height, .. } => *height,
        }
    }

    pub fn viewport(&self) -> Viewport {
        Viewport::new_at_origo(self.width(), self.height())
    }

    fn bind(&self) {
        match &self.0 {
            DT::Texture2D { texture } => {
                texture.bind_as_depth_target();
            }
            DT::Texture2DArray { texture, layer } => {
                texture.bind_as_depth_target(*layer);
            }
            DT::TextureCubeMap { texture, side } => {
                texture.bind_as_depth_target(*side);
            }
            _ => {}
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
    id: Option<Framebuffer>,
    color: Option<ColorTarget<'a>>,
    depth: Option<DepthTarget<'a>>,
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
            id: None,
            color: Some(ColorTarget::screen(context, width, height)),
            depth: Some(DepthTarget::screen(context, width, height)),
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
            id: Some(new_framebuffer(context)?),
            color: Some(color),
            depth: Some(depth),
        })
    }

    fn new_color(context: &Context, color: ColorTarget<'a>) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: Some(new_framebuffer(context)?),
            color: Some(color),
            depth: None,
        })
    }

    fn new_depth(context: &Context, depth: DepthTarget<'a>) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: Some(new_framebuffer(context)?),
            depth: Some(depth),
            color: None,
        })
    }

    fn color(&self) -> &ColorTarget {
        self.color.as_ref().unwrap()
    }

    fn depth(&self) -> &DepthTarget {
        self.depth.as_ref().unwrap()
    }

    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_in_viewport(self.color().viewport(), clear_state)
    }

    pub fn clear_in_viewport(
        &self,
        viewport: Viewport,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.context.set_scissor(viewport);
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        clear_state.apply(&self.context);
        self.context.error_check()?;
        Ok(self)
    }

    ///
    /// Renders whatever rendered in the `render` closure into this render target.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_in_viewport(self.color().viewport(), render)
    }

    pub fn write_in_viewport(
        &self,
        viewport: Viewport,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.context.set_scissor(viewport);
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        render()?;
        if let Some(ref color) = self.color {
            color.generate_mip_maps();
        }
        self.context.error_check()?;
        Ok(self)
    }

    pub fn read_color<T: TextureDataType>(&self) -> ThreeDResult<Vec<T>> {
        self.read_color_in_viewport(self.color().viewport())
    }

    ///
    /// Returns the colors of the pixels in this render target inside the given viewport.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_color_in_viewport<T: TextureDataType>(
        &self,
        viewport: Viewport,
    ) -> ThreeDResult<Vec<T>> {
        if self.color.is_none() {
            Err(CoreError::RenderTargetRead("color".to_string()))?;
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut data_size = std::mem::size_of::<T>();
        // On web, the format needs to be RGBA if the data type is byte.
        if data_size / T::size() as usize == 1 {
            data_size *= 4 / T::size() as usize
        }
        let mut bytes = vec![0u8; viewport.width as usize * viewport.height as usize * data_size];
        unsafe {
            self.context.read_pixels(
                viewport.x as i32,
                viewport.y as i32,
                viewport.width as i32,
                viewport.height as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelPackData::Slice(&mut bytes),
            );
            self.context.error_check()?;
        }
        let mut pixels = from_byte_slice(&bytes).to_vec();
        flip_y(
            &mut pixels,
            viewport.width as usize,
            viewport.height as usize,
        );
        Ok(pixels)
    }

    pub fn read_depth(&self) -> ThreeDResult<Vec<f32>> {
        self.read_depth_in_viewport(self.depth().viewport())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth_in_viewport(&self, viewport: Viewport) -> ThreeDResult<Vec<f32>> {
        if self.depth.is_none() {
            Err(CoreError::RenderTargetRead("depth".to_string()))?;
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

    pub(in crate::core) fn bind(&self, target: u32) -> ThreeDResult<()> {
        unsafe {
            self.context.bind_framebuffer(target, self.id);
        }
        if let Some(ref color) = self.color {
            color.bind(&self.context);
        }
        if let Some(ref depth) = self.depth {
            depth.bind();
        }
        self.context.framebuffer_check()?;
        self.context.error_check()
    }
}

impl Drop for RenderTarget<'_> {
    fn drop(&mut self) {
        unsafe {
            if let Some(id) = self.id {
                self.context.delete_framebuffer(id);
            }
        }
    }
}

fn size_with_mip(size: u32, mip: Option<u32>) -> u32 {
    if let Some(mip) = mip {
        size / 2u32.pow(mip)
    } else {
        size
    }
}
