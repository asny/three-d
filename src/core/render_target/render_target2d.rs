use crate::core::render_target::*;

///
/// Adds additional functionality to write to and copy from both a [ColorTargetTexture2D] and
/// a [DepthTargetTexture2D] at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTarget<'a, 'b, T: TextureDataType> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a mut Texture2D<T>>,
    depth_texture: Option<&'b mut DepthTargetTexture2D>,
}

impl<'a, 'b> RenderTarget<'a, 'b, u8> {
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
}

impl<'a, 'b, T: TextureDataType> RenderTarget<'a, 'b, T> {
    ///
    /// Constructs a new render target that enables rendering into the given
    /// [ColorTargetTexture2D] and [DepthTargetTexture2D] textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a mut Texture2D<T>,
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
    pub fn new_color(context: &Context, color_texture: &'a mut Texture2D<T>) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

    pub(crate) fn new_depth_internal(
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
    /// Renders whatever rendered in the `render` closure into the textures defined at construction.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write(
        &self,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        self.bind(consts::DRAW_FRAMEBUFFER)?;
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
    /// Copies the content of the color and depth texture to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from<U: TextureDataType>(
        &self,
        color_texture: Option<&Texture2D<U>>,
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
    pub fn copy_from_array<U: TextureDataType>(
        &self,
        color_texture: Option<(&Texture2DArray<U>, u32)>,
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
        self.context.bind_framebuffer(target, Some(&self.id));
        if let Some(ref tex) = self.color_texture {
            self.context.draw_buffers(&[consts::COLOR_ATTACHMENT0]);
            tex.bind_as_color_target(0);
        }
        if let Some(ref tex) = self.depth_texture {
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
