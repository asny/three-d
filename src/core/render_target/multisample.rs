use crate::core::*;
use crate::render_target::multisample_sanity_check;

///
/// A render target that supports multisample anti-aliasing. This is a combination of the functionality of [ColorTargetMultisample] and [DepthTargetMultisample].
/// To resolve the multisampled buffer to a color texture, use [RenderTargetMultisample::resolve_color]. The variants [RenderTargetMultisample::resolve_depth], and [RenderTargetMultisample::resolve] resolve the buffer to a depth texture or a tuple of color and depth textures, respectively.
/// It is possible to resolve to another render target via [RenderTargetMultisample::resolve_to] (and its variants), which will give greater control over the resolution.
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
    ///
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        number_of_samples: u32,
    ) -> Self {
        multisample_sanity_check(context, number_of_samples);
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
    /// Resolve the color buffer into the given color target.
    ///
    pub fn resolve_color_to(&self, target: &ColorTarget<'_>) {
        ColorTarget::new_texture_2d_multisample(&self.context, &self.color)
            .as_render_target()
            .blit_to(&target.as_render_target());
    }

    ///
    /// Resolve the depth buffer into the given depth target.
    ///
    pub fn resolve_depth_to(&self, target: &DepthTarget<'_>) {
        DepthTarget::new_texture_2d_multisample(&self.context, &self.depth)
            .as_render_target()
            .blit_to(&target.as_render_target());
    }

    ///
    /// Resolve both color and depth buffers into the given render target.
    ///
    pub fn resolve_to(&self, target: &RenderTarget<'_>) {
        self.as_render_target().blit_to(target);
    }

    ///
    /// Resolve the color buffer to a color texture.
    ///
    pub fn resolve_color(&self) -> Texture2D {
        let mut color_texture = Texture2D::new_empty::<C>(
            &self.context,
            self.color.width(),
            self.color.height(),
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        self.resolve_color_to(&color_texture.as_color_target(None));
        color_texture
    }

    ///
    /// Resolve the depth buffer to a depth texture.
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
    /// Resolve the color and depth buffers into a color texture and a depth texture.
    ///
    pub fn resolve(&self) -> (Texture2D, DepthTexture2D) {
        let mut color_texture = Texture2D::new_empty::<C>(
            &self.context,
            self.color.width(),
            self.color.height(),
            Interpolation::Nearest,
            Interpolation::Nearest,
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
