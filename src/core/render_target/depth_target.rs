use super::*;

///
/// Adds additional functionality to clear, read from and write to a texture.
/// Use the `as_depth_target` function directly on the texture structs (for example [DepthTargetTexture2D]) to construct a depth target.
/// Combine this together with a [ColorTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
/// A depth target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the texture.
///
#[derive(Clone)]
pub struct DepthTarget<'a> {
    pub(crate) context: Context,
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

    pub(crate) fn as_render_target(&self) -> ThreeDResult<RenderTarget<'a>> {
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

    pub(super) fn bind(&self) {
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
