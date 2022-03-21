use crate::core::render_target::*;

///
/// Adds additional functionality to write to and copy from both a [ColorTargetTexture2D] and
/// a [DepthTargetTexture2D] at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTarget<'a, 'b> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a Texture2D>,
    depth_texture: Option<&'b DepthTargetTexture2D>,
}

impl<'a, 'b> RenderTarget<'a, 'b> {
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
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture),
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
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Constructs a new render target that enables rendering into the given
    /// [ColorTargetTexture2D].
    ///
    pub fn new_color(context: &Context, color_texture: &'a mut Texture2D) -> ThreeDResult<Self> {
        Self::new_color_internal(context, color_texture)
    }

    pub(in crate::core) fn new_color_internal(
        context: &Context,
        color_texture: &'a Texture2D,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined at construction.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write(
        &self,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        clear(
            &self.context,
            &ClearState {
                red: self.color_texture.as_ref().and(clear_state.red),
                green: self.color_texture.as_ref().and(clear_state.green),
                blue: self.color_texture.as_ref().and(clear_state.blue),
                alpha: self.color_texture.as_ref().and(clear_state.alpha),
                depth: self.depth_texture.as_ref().and(clear_state.depth),
            },
        );
        render()?;
        if let Some(ref color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        Ok(())
    }

    ///
    /// Returns the values of the pixels in this texture inside the given viewport.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    pub fn read_color<T: TextureDataType>(&self, viewport: Viewport) -> ThreeDResult<Vec<T>> {
        if self.color_texture.is_none() {
            Err(CoreError::RenderTargetRead("color".to_string()))?;
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
            Ok(from_byte_slice(&pixels).to_vec())
        }
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
            self.context.bind_framebuffer(target, Some(self.id));
            if let Some(ref tex) = self.color_texture {
                self.context
                    .draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                tex.bind_as_color_target(0);
            }
            if let Some(ref tex) = self.depth_texture {
                tex.bind_as_depth_target();
            }
        }
        self.context.framebuffer_check()?;
        self.context.error_check()
    }
}

impl Drop for RenderTarget<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_framebuffer(self.id);
        }
    }
}
