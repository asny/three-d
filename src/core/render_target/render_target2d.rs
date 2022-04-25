use crate::core::render_target::*;

///
/// Adds additional functionality to clear, read from and write to a texture.
/// Use the `as_color_target` function directly on the texture structs (for example [Texture2D]) to construct a color target.
/// Combine this together with a [DepthTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
/// A color target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the texture.
///
/// **Note:** [DepthTest] is disabled if not also writing to a [DepthTarget].
///
#[derive(Clone)]
pub struct ColorTarget<'a> {
    context: Context,
    target: CT<'a>,
}

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
}

impl<'a> ColorTarget<'a> {
    pub(in crate::core) fn new_texture2d(
        context: &Context,
        texture: &'a Texture2D,
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            target: CT::Texture2D { texture, mip_level },
        }
    }

    pub(in crate::core) fn new_texture_cube_map(
        context: &Context,
        texture: &'a TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            target: CT::TextureCubeMap {
                texture,
                side,
                mip_level,
            },
        }
    }

    pub(in crate::core) fn new_texture_2d_array(
        context: &Context,
        texture: &'a Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            target: CT::Texture2DArray {
                texture,
                layers,
                mip_level,
            },
        }
    }

    ///
    /// Clears the color of this color target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the color of the part of this color target that is inside the given scissor box.
    ///
    pub fn clear_partially(
        &self,
        scissor_box: ScissorBox,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?.clear_partially(
            scissor_box,
            ClearState {
                depth: None,
                ..clear_state
            },
        )?;
        Ok(self)
    }

    ///
    /// Writes whatever rendered in the `render` closure into this color target.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this color target defined by the scissor box.
    ///
    pub fn write_partially(
        &self,
        scissor_box: ScissorBox,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?
            .write_partially(scissor_box, render)?;
        Ok(self)
    }

    ///
    /// Returns the colors of the pixels in this color target.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read<T: TextureDataType>(&self) -> ThreeDResult<Vec<T>> {
        self.read_partially(self.scissor_box())
    }

    ///
    /// Returns the colors of the pixels in this color target inside the given scissor box.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_partially<T: TextureDataType>(
        &self,
        scissor_box: ScissorBox,
    ) -> ThreeDResult<Vec<T>> {
        self.as_render_target()?.read_color_partially(scissor_box)
    }

    ///
    /// Returns the width of the color target in texels.
    /// If using the zero mip level of the underlying texture, then this is simply the width of that texture, otherwise it is the width of the given mip level.
    ///
    pub fn width(&self) -> u32 {
        match self.target {
            CT::Texture2D { texture, mip_level } => size_with_mip(texture.width(), mip_level),
            CT::Texture2DArray {
                texture, mip_level, ..
            } => size_with_mip(texture.width(), mip_level),
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => size_with_mip(texture.width(), mip_level),
        }
    }

    ///
    /// Returns the height of the color target in texels.
    /// If using the zero mip level of the underlying texture, then this is simply the height of that texture, otherwise it is the height of the given mip level.
    ///
    pub fn height(&self) -> u32 {
        match self.target {
            CT::Texture2D { texture, mip_level } => size_with_mip(texture.height(), mip_level),
            CT::Texture2DArray {
                texture, mip_level, ..
            } => size_with_mip(texture.height(), mip_level),
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => size_with_mip(texture.height(), mip_level),
        }
    }

    ///
    /// Returns the scissor box that encloses the entire target.
    ///
    pub fn scissor_box(&self) -> ScissorBox {
        ScissorBox::new_at_origo(self.width(), self.height())
    }

    pub(in crate::core) fn as_render_target(&self) -> ThreeDResult<RenderTarget<'a>> {
        RenderTarget::new_color(self.clone())
    }

    fn generate_mip_maps(&self) {
        match self.target {
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
        }
    }

    fn bind(&self, context: &Context) {
        match self.target {
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
        }
    }
}

///
/// Adds additional functionality to clear, read from and write to a texture.
/// Use the `as_depth_target` function directly on the texture structs (for example [DepthTargetTexture2D]) to construct a depth target.
/// Combine this together with a [ColorTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
/// A depth target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the texture.
///
#[derive(Clone)]
pub struct DepthTarget<'a> {
    context: Context,
    target: DT<'a>,
}

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
}

impl<'a> DepthTarget<'a> {
    pub(in crate::core) fn new_texture2d(
        context: &Context,
        texture: &'a DepthTargetTexture2D,
    ) -> Self {
        Self {
            context: context.clone(),
            target: DT::Texture2D { texture },
        }
    }

    pub(in crate::core) fn new_texture_cube_map(
        context: &Context,
        texture: &'a DepthTargetTextureCubeMap,
        side: CubeMapSide,
    ) -> Self {
        Self {
            context: context.clone(),
            target: DT::TextureCubeMap { texture, side },
        }
    }

    pub(in crate::core) fn new_texture_2d_array(
        context: &Context,
        texture: &'a DepthTargetTexture2DArray,
        layer: u32,
    ) -> Self {
        Self {
            context: context.clone(),
            target: DT::Texture2DArray { texture, layer },
        }
    }

    ///
    /// Clears the depth of this depth target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the depth of the part of this depth target that is inside the given scissor box.
    ///
    pub fn clear_partially(
        &self,
        scissor_box: ScissorBox,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?.clear_partially(
            scissor_box,
            ClearState {
                depth: clear_state.depth,
                ..ClearState::none()
            },
        )?;
        Ok(self)
    }

    ///
    /// Writes whatever rendered in the `render` closure into this depth target.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this depth target defined by the scissor box.
    ///
    pub fn write_partially(
        &self,
        scissor_box: ScissorBox,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?
            .write_partially(scissor_box, render)?;
        Ok(self)
    }

    ///
    /// Returns the depth values in this depth target.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read(&self) -> ThreeDResult<Vec<f32>> {
        self.read_partially(self.scissor_box())
    }

    ///
    /// Returns the depth values in this depth target inside the given scissor box.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_partially(&self, scissor_box: ScissorBox) -> ThreeDResult<Vec<f32>> {
        self.as_render_target()?.read_depth_partially(scissor_box)
    }

    fn as_render_target(&self) -> ThreeDResult<RenderTarget<'a>> {
        RenderTarget::new_depth(self.clone())
    }

    ///
    /// Returns the width of the depth target in texels, which is simply the width of the underlying texture.
    ///
    pub fn width(&self) -> u32 {
        match &self.target {
            DT::Texture2D { texture, .. } => texture.width(),
            DT::Texture2DArray { texture, .. } => texture.width(),
            DT::TextureCubeMap { texture, .. } => texture.width(),
        }
    }

    ///
    /// Returns the height of the depth target in texels, which is simply the height of the underlying texture.
    ///
    pub fn height(&self) -> u32 {
        match &self.target {
            DT::Texture2D { texture, .. } => texture.height(),
            DT::Texture2DArray { texture, .. } => texture.height(),
            DT::TextureCubeMap { texture, .. } => texture.height(),
        }
    }

    ///
    /// Returns the scissor box that encloses the entire target.
    ///
    pub fn scissor_box(&self) -> ScissorBox {
        ScissorBox::new_at_origo(self.width(), self.height())
    }

    fn bind(&self) {
        match &self.target {
            DT::Texture2D { texture } => {
                texture.bind_as_depth_target();
            }
            DT::Texture2DArray { texture, layer } => {
                texture.bind_as_depth_target(*layer);
            }
            DT::TextureCubeMap { texture, side } => {
                texture.bind_as_depth_target(*side);
            }
        }
    }
}

use crate::context::Framebuffer;
///
/// Adds additional functionality to clear, read from and write to the screen (see [RenderTarget::screen]) or a color texture and
/// a depth texture at the same time (see [RenderTarget::new]).
/// If you only want to perform an operation on either a color texture or depth texture, see [ColorTarget] and [DepthTarget] respectively.
/// A render target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the textures.
///
pub struct RenderTarget<'a> {
    id: Option<Framebuffer>,
    color: Option<ColorTarget<'a>>,
    depth: Option<DepthTarget<'a>>,
    context: Context,
    width: u32,
    height: u32,
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
            color: None,
            depth: None,
            width,
            height,
        }
    }

    ///
    /// Constructs a new render target that enables rendering into the given [ColorTarget] and [DepthTarget].
    ///
    pub fn new(color: ColorTarget<'a>, depth: DepthTarget<'a>) -> ThreeDResult<Self> {
        let width = color.width();
        let height = color.height();
        Ok(Self {
            context: color.context.clone(),
            id: Some(new_framebuffer(&color.context)?),
            color: Some(color),
            depth: Some(depth),
            width,
            height,
        })
    }

    ///
    /// Clears the color and depth of this render target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the color and depth of the part of this render target that is inside the given scissor box.
    ///
    pub fn clear_partially(
        &self,
        scissor_box: ScissorBox,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.context.set_scissor(scissor_box);
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        clear_state.apply(&self.context);
        self.context.error_check()?;
        Ok(self)
    }

    ///
    /// Writes whatever rendered in the `render` closure into this render target.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this render target defined by the scissor box.
    ///
    pub fn write_partially(
        &self,
        scissor_box: ScissorBox,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.context.set_scissor(scissor_box);
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        render()?;
        if let Some(ref color) = self.color {
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
        self.read_color_partially(self.scissor_box())
    }

    ///
    /// Returns the colors of the pixels in this render target inside the given scissor box.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_color_partially<T: TextureDataType>(
        &self,
        scissor_box: ScissorBox,
    ) -> ThreeDResult<Vec<T>> {
        if self.id.is_some() && self.color.is_none() {
            Err(CoreError::RenderTargetRead("color".to_string()))?;
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut data_size = std::mem::size_of::<T>();
        // On web, the format needs to be RGBA if the data type is byte.
        if data_size / T::size() as usize == 1 {
            data_size *= 4 / T::size() as usize
        }
        let mut bytes =
            vec![0u8; scissor_box.width as usize * scissor_box.height as usize * data_size];
        unsafe {
            self.context.read_pixels(
                scissor_box.x as i32,
                scissor_box.y as i32,
                scissor_box.width as i32,
                scissor_box.height as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelPackData::Slice(&mut bytes),
            );
            self.context.error_check()?;
        }
        let mut pixels = from_byte_slice(&bytes).to_vec();
        flip_y(
            &mut pixels,
            scissor_box.width as usize,
            scissor_box.height as usize,
        );
        Ok(pixels)
    }

    ///
    /// Returns the depth values in this render target.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self) -> ThreeDResult<Vec<f32>> {
        self.read_depth_partially(self.scissor_box())
    }

    ///
    /// Returns the depth values in this render target inside the given scissor box.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth_partially(&self, scissor_box: ScissorBox) -> ThreeDResult<Vec<f32>> {
        if self.id.is_some() && self.depth.is_none() {
            Err(CoreError::RenderTargetRead("depth".to_string()))?;
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut pixels = vec![0u8; scissor_box.width as usize * scissor_box.height as usize * 4];
        unsafe {
            self.context.read_pixels(
                scissor_box.x as i32,
                scissor_box.y as i32,
                scissor_box.width as i32,
                scissor_box.height as i32,
                crate::context::DEPTH_COMPONENT,
                crate::context::FLOAT,
                crate::context::PixelPackData::Slice(&mut pixels),
            );
        }
        self.context.error_check()?;
        Ok(from_byte_slice(&pixels).to_vec())
    }

    ///
    /// Copies the content of the color and depth texture to the specified scissor box of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from(
        &self,
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
        scissor_box: ScissorBox,
        write_mask: WriteMask,
    ) -> ThreeDResult<&Self> {
        self.write(|| {
            copy_from(
                &self.context,
                color_texture,
                depth_texture,
                scissor_box.into(),
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
        scissor_box: ScissorBox,
        write_mask: WriteMask,
    ) -> ThreeDResult<&Self> {
        self.write(|| {
            copy_from_array(
                &self.context,
                color_texture,
                depth_texture,
                scissor_box.into(),
                write_mask,
            )
        })
    }

    ///
    /// Returns the scissor box that encloses the entire target.
    ///
    pub fn scissor_box(&self) -> ScissorBox {
        ScissorBox::new_at_origo(self.width, self.height)
    }

    fn new_color(color: ColorTarget<'a>) -> ThreeDResult<Self> {
        let width = color.width();
        let height = color.height();
        Ok(Self {
            context: color.context.clone(),
            id: Some(new_framebuffer(&color.context)?),
            color: Some(color),
            depth: None,
            width,
            height,
        })
    }

    fn new_depth(depth: DepthTarget<'a>) -> ThreeDResult<Self> {
        let width = depth.width();
        let height = depth.height();
        Ok(Self {
            context: depth.context.clone(),
            id: Some(new_framebuffer(&depth.context)?),
            depth: Some(depth),
            color: None,
            width,
            height,
        })
    }

    fn bind(&self, target: u32) -> ThreeDResult<()> {
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
