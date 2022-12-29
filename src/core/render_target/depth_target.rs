use super::*;

///
/// Adds additional functionality to clear, read from and write to a texture.
/// Use the `as_depth_target` function directly on the texture structs (for example [DepthTexture2D]) to construct a depth target.
/// Combine this together with a [ColorTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
/// A depth target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the texture.
///
#[derive(Clone)]
pub struct DepthTarget<'a> {
    pub(crate) context: Context,
    target: DepthTexture<'a>,
}

impl<'a> DepthTarget<'a> {
    pub(in crate::core) fn new_texture2d(context: &Context, texture: &'a DepthTexture2D) -> Self {
        Self {
            context: context.clone(),
            target: DepthTexture::Single(texture),
        }
    }

    pub(in crate::core) fn new_texture_cube_map(
        context: &Context,
        texture: &'a DepthTextureCubeMap,
        side: CubeMapSide,
    ) -> Self {
        Self {
            context: context.clone(),
            target: DepthTexture::CubeMap { texture, side },
        }
    }

    pub(in crate::core) fn new_texture_2d_array(
        context: &Context,
        texture: &'a DepthTexture2DArray,
        layer: u32,
    ) -> Self {
        Self {
            context: context.clone(),
            target: DepthTexture::Array { texture, layer },
        }
    }

    pub(in crate::core) fn new_texture_2d_multisample(
        context: &Context,
        texture: &'a DepthTexture2DMultisample,
    ) -> Self {
        Self {
            context: context.clone(),
            target: DepthTexture::Multisample(texture),
        }
    }

    ///
    /// Clears the depth of this depth target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> &Self {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the depth of the part of this depth target that is inside the given scissor box.
    ///
    pub fn clear_partially(&self, scissor_box: ScissorBox, clear_state: ClearState) -> &Self {
        self.as_render_target().clear_partially(
            scissor_box,
            ClearState {
                depth: clear_state.depth,
                ..ClearState::none()
            },
        );
        self
    }

    ///
    /// Writes whatever rendered in the `render` closure into this depth target.
    ///
    pub fn write(&self, render: impl FnOnce()) -> &Self {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this depth target defined by the scissor box.
    ///
    pub fn write_partially(&self, scissor_box: ScissorBox, render: impl FnOnce()) -> &Self {
        self.as_render_target().write_partially(scissor_box, render);
        self
    }

    ///
    /// Returns the depth values in this depth target.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read(&self) -> Vec<f32> {
        self.read_partially(self.scissor_box())
    }

    ///
    /// Returns the depth values in this depth target inside the given scissor box.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_partially(&self, scissor_box: ScissorBox) -> Vec<f32> {
        self.as_render_target().read_depth_partially(scissor_box)
    }

    ///
    /// Copies the content of the depth texture
    /// to the part of this depth target specified by the [Viewport].
    ///
    pub fn copy_from(&self, depth_texture: DepthTexture, viewport: Viewport) -> &Self {
        self.copy_partially_from(self.scissor_box(), depth_texture, viewport)
    }

    ///
    /// Copies the content of the depth texture as limited by the [ScissorBox]
    /// to the part of this depth target specified by the [Viewport].
    ///
    pub fn copy_partially_from(
        &self,
        scissor_box: ScissorBox,
        depth_texture: DepthTexture,
        viewport: Viewport,
    ) -> &Self {
        self.as_render_target()
            .copy_partially_from_depth(scissor_box, depth_texture, viewport);
        self
    }

    pub(crate) fn as_render_target(&self) -> RenderTarget<'a> {
        RenderTarget::new_depth(self.clone())
    }

    ///
    /// Returns the width of the depth target in texels, which is simply the width of the underlying texture.
    ///
    pub fn width(&self) -> u32 {
        match &self.target {
            DepthTexture::Single(texture) => texture.width(),
            DepthTexture::Array { texture, .. } => texture.width(),
            DepthTexture::CubeMap { texture, .. } => texture.width(),
            DepthTexture::Multisample(texture) => texture.width(),
        }
    }

    ///
    /// Returns the height of the depth target in texels, which is simply the height of the underlying texture.
    ///
    pub fn height(&self) -> u32 {
        match &self.target {
            DepthTexture::Single(texture) => texture.height(),
            DepthTexture::Array { texture, .. } => texture.height(),
            DepthTexture::CubeMap { texture, .. } => texture.height(),
            DepthTexture::Multisample(texture) => texture.height(),
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
            DepthTexture::Single(texture) => {
                texture.bind_as_depth_target();
            }
            DepthTexture::Array { texture, layer } => {
                texture.bind_as_depth_target(*layer);
            }
            DepthTexture::CubeMap { texture, side } => {
                texture.bind_as_depth_target(*side);
            }
            DepthTexture::Multisample(texture) => {
                texture.bind_as_depth_target();
            }
        }
    }
}
