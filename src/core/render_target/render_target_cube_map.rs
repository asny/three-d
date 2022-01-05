use crate::context::consts;
use crate::core::render_target::*;
use crate::core::*;

///
/// Adds additional functionality to write to a [TextureCubeMap] and
/// a [DepthTargetTextureCubeMap] at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTargetCubeMap<'a, 'b, T: TextureDataType> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a mut TextureCubeMap<T>>,
    depth_texture: Option<&'b mut DepthTargetTextureCubeMap>,
}

impl<'a, 'b, T: TextureDataType> RenderTargetCubeMap<'a, 'b, T> {
    ///
    /// Constructs a new render target cube map that enables rendering into the given
    /// [TextureCubeMap] and [DepthTargetTextureCubeMap] textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a mut TextureCubeMap<T>,
        depth_texture: &'b mut DepthTargetTextureCubeMap,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture),
        })
    }

    pub fn new_color(
        context: &Context,
        color_texture: &'a mut TextureCubeMap<T>,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

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

    pub fn write_to_mip_level(
        &self,
        side: CubeMapSide,
        mip_level: u32,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        self.context
            .bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&self.id));
        if let Some(ref color_texture) = self.color_texture {
            self.context.draw_buffers(&[consts::COLOR_ATTACHMENT0]);
            color_texture.bind_as_color_target(side, 0, mip_level);
        }
        if let Some(ref depth_texture) = self.depth_texture {
            depth_texture.bind_as_depth_target(side);
        }
        #[cfg(feature = "debug")]
        check(&self.context)?;

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
        Ok(())
    }

    pub fn width(&self) -> u32 {
        self.color_texture
            .as_ref()
            .map(|t| t.width())
            .unwrap_or_else(|| self.depth_texture.as_ref().unwrap().width())
    }

    pub fn height(&self) -> u32 {
        self.color_texture
            .as_ref()
            .map(|t| t.height())
            .unwrap_or_else(|| self.depth_texture.as_ref().unwrap().height())
    }
}

impl<T: TextureDataType> Drop for RenderTargetCubeMap<'_, '_, T> {
    fn drop(&mut self) {
        self.context.delete_framebuffer(Some(&self.id));
    }
}
