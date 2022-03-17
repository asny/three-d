use crate::core::render_target::*;

///
/// Adds additional functionality to write to a [TextureCubeMap] and
/// a [DepthTargetTextureCubeMap] at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTargetCubeMap<'a, 'b> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a mut TextureCubeMap>,
    depth_texture: Option<&'b mut DepthTargetTextureCubeMap>,
}

impl<'a, 'b> RenderTargetCubeMap<'a, 'b> {
    ///
    /// Constructs a new render target cube map that enables rendering into the given
    /// [DepthTargetTextureCubeMap].
    ///
    pub fn new_depth(
        context: &Context,
        depth_texture: &'b mut DepthTargetTextureCubeMap,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Constructs a new render target cube map that enables rendering into the given
    /// [TextureCubeMap] and [DepthTargetTextureCubeMap] textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a mut TextureCubeMap,
        depth_texture: &'b mut DepthTargetTextureCubeMap,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Constructs a new render target cube map that enables rendering into the given
    /// [TextureCubeMap].
    ///
    pub fn new_color(
        context: &Context,
        color_texture: &'a mut TextureCubeMap,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures at the given side of the cube map render target.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write(
        &self,
        side: CubeMapSide,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        self.write_to_mip_level(side, 0, clear_state, render)?;
        if let Some(ref color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        Ok(())
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures at the given side and at the given mip level of the cube map render target.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write_to_mip_level(
        &self,
        side: CubeMapSide,
        mip_level: u32,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        unsafe {
            self.context
                .bind_framebuffer(crate::context::DRAW_FRAMEBUFFER, Some(self.id));
            if let Some(ref color_texture) = self.color_texture {
                self.context
                    .draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                color_texture.bind_as_color_target(side, 0, mip_level);
            }
        }
        if let Some(ref depth_texture) = self.depth_texture {
            depth_texture.bind_as_depth_target(side);
        }
        self.context.framebuffer_check()?;

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
        self.context.error_check()
    }
}

impl Drop for RenderTargetCubeMap<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_framebuffer(self.id);
        }
    }
}
