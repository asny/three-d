use crate::core::*;

///
/// A multisampled render target for depth data.
/// To resolve the multisampled buffer to a depth texture, use [DepthTargetMultisample::resolve]. Use [DepthTargetMultisample::resolve_to] to resolve to another render target.
///
pub struct DepthTargetMultisample<D: DepthTextureDataType> {
    pub(crate) context: Context,
    depth: DepthTexture2DMultisample,
    _d: std::marker::PhantomData<D>,
}

impl<D: DepthTextureDataType> DepthTargetMultisample<D> {
    ///
    /// Constructs a new multisampled depth target with the given dimensions and number of samples.
    ///
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        #[cfg(debug_assertions)]
        super::multisample_sanity_check(context, number_of_samples);
        Self {
            context: context.clone(),
            depth: DepthTexture2DMultisample::new::<D>(context, width, height, number_of_samples),
            _d: std::marker::PhantomData,
        }
    }

    ///
    /// Clears the color and depth of this target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> &Self {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the color and depth of the part of this target that is inside the given scissor box.
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
    /// Writes whatever rendered in the `render` closure into this target.
    ///
    pub fn write(&self, render: impl FnOnce()) -> &Self {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this target defined by the scissor box.
    ///
    pub fn write_partially(&self, scissor_box: ScissorBox, render: impl FnOnce()) -> &Self {
        self.as_render_target().write_partially(scissor_box, render);
        self
    }

    /// The width of this target.
    pub fn width(&self) -> u32 {
        self.depth.width()
    }

    /// The height of this target.
    pub fn height(&self) -> u32 {
        self.depth.height()
    }

    /// The number of samples for each fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.depth.number_of_samples()
    }

    fn as_render_target(&self) -> RenderTarget<'_> {
        DepthTarget::new_texture_2d_multisample(&self.context, &self.depth).as_render_target()
    }

    ///
    /// Resolves the depth buffer into the given depth target.
    ///
    pub fn resolve_to(&self, target: &DepthTarget<'_>) {
        self.as_render_target().blit_to(&target.as_render_target());
    }

    ///
    /// Resolves the depth buffer to a depth texture.
    ///
    pub fn resolve(&self) -> DepthTexture2D {
        let mut depth_texture = DepthTexture2D::new::<D>(
            &self.context,
            self.width(),
            self.height(),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        self.resolve_to(&depth_texture.as_depth_target());
        depth_texture
    }
}
