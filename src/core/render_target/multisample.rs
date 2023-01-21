use crate::core::*;

///
/// A multisampled render target for color and depth data. Use this if you want to avoid aliasing, ie. jagged edges, when rendering to a [RenderTarget].
///
/// After rendering into this target, it needs to be resolved to a non-multisample texture to be able to sample it in a shader.
/// To do this, use the [RenderTargetMultisample::resolve], [RenderTargetMultisample::resolve_to], [RenderTargetMultisample::resolve_color_to]
/// or [RenderTargetMultisample::resolve_depth_to] methods.
///
/// Also see [ColorTargetMultisample] and [DepthTargetMultisample].
///
pub struct RenderTargetMultisample<C: TextureDataType, D: DepthTextureDataType> {
    pub(crate) context: Context,
    color: Texture2DMultisample,
    depth: DepthTexture2DMultisample,
    _c: std::marker::PhantomData<C>,
    _d: std::marker::PhantomData<D>,
}

impl<C: TextureDataType, D: DepthTextureDataType> RenderTargetMultisample<C, D> {
    ///
    /// Constructs a new multisample render target with the given dimensions and number of samples.
    /// The number of samples must be larger than 0, less than or equal to the maximum number of samples supported by the hardware and power of two.
    ///
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        #[cfg(debug_assertions)]
        super::multisample_sanity_check(context, number_of_samples);
        Self {
            context: context.clone(),
            color: Texture2DMultisample::new::<C>(context, width, height, number_of_samples),
            depth: DepthTexture2DMultisample::new::<D>(context, width, height, number_of_samples),
            _c: std::marker::PhantomData,
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
        self.as_render_target()
            .clear_partially(scissor_box, clear_state);
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
        self.color.width()
    }

    /// The height of this target.
    pub fn height(&self) -> u32 {
        self.color.height()
    }

    /// The number of samples for each fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.color.number_of_samples()
    }

    pub(super) fn as_render_target(&self) -> RenderTarget<'_> {
        RenderTarget::new(
            ColorTarget::new_texture_2d_multisample(&self.context, &self.color),
            DepthTarget::new_texture_2d_multisample(&self.context, &self.depth),
        )
    }

    ///
    /// Resolves the color of the multisample render target into the given non-multisample color target.
    /// The target must have the same width, height and [TextureDataType] as the color part of this target.
    ///
    pub fn resolve_color_to(&self, target: &ColorTarget<'_>) {
        ColorTarget::new_texture_2d_multisample(&self.context, &self.color)
            .as_render_target()
            .blit_to(&target.as_render_target());
    }

    ///
    /// Resolves the depth of the multisample render target into the given non-multisample depth target.
    /// The target must have the same width, height and [DepthTextureDataType] as the depth part of this target.
    ///
    pub fn resolve_depth_to(&self, target: &DepthTarget<'_>) {
        DepthTarget::new_texture_2d_multisample(&self.context, &self.depth)
            .as_render_target()
            .blit_to(&target.as_render_target());
    }

    ///
    /// Resolves the multisample render target into the given non-multisample render target.
    /// The target must have the same width, height, [TextureDataType] and [DepthTextureDataType] as this target.
    /// If the given render target is the screen render target, it must be non-multisampled or have the same number of samples as this target.
    ///
    pub fn resolve_to(&self, target: &RenderTarget<'_>) {
        self.as_render_target().blit_to(target);
    }

    ///
    /// Resolves the color of the multisample render target to a default non-multisample [Texture2D].
    /// Use [RenderTargetMultisample::resolve_color_to] to resolve to a custom non-multisample texture.
    ///
    pub fn resolve_color(&self) -> Texture2D {
        let mut color_texture = Texture2D::new_empty::<C>(
            &self.context,
            self.color.width(),
            self.color.height(),
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        self.resolve_color_to(&color_texture.as_color_target(None));
        color_texture
    }

    ///
    /// Resolves the depth of the multisample render target to a default non-multisample [DepthTexture2D].
    /// Use [RenderTargetMultisample::resolve_depth_to] to resolve to a custom non-multisample texture.
    ///
    pub fn resolve_depth(&self) -> DepthTexture2D {
        let mut depth_texture = DepthTexture2D::new::<D>(
            &self.context,
            self.width(),
            self.height(),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        self.resolve_depth_to(&depth_texture.as_depth_target());
        depth_texture
    }

    ///
    /// Resolves the multisample render target to default non-multisample [Texture2D] and [DepthTexture2D].
    /// Use [RenderTargetMultisample::resolve_to] to resolve to custom non-multisample textures.
    ///
    pub fn resolve(&self) -> (Texture2D, DepthTexture2D) {
        let mut color_texture = Texture2D::new_empty::<C>(
            &self.context,
            self.color.width(),
            self.color.height(),
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let mut depth_texture = DepthTexture2D::new::<D>(
            &self.context,
            self.width(),
            self.height(),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        self.resolve_to(&RenderTarget::new(
            color_texture.as_color_target(None),
            depth_texture.as_depth_target(),
        ));
        (color_texture, depth_texture)
    }
}
