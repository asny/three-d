use crate::core::*;

///
/// A multisample render target for color data. Use this if you want to avoid aliasing, ie. jagged edges, when rendering to a [ColorTarget].
///
/// After rendering into this target, it needs to be resolved to a non-multisample texture to be able to sample it in a shader.
/// To do this, use the [ColorTargetMultisample::resolve] or [ColorTargetMultisample::resolve_to] methods.
///
/// Also see [RenderTargetMultisample] and [DepthTargetMultisample].
///
pub struct ColorTargetMultisample<C: TextureDataType> {
    pub(crate) context: Context,
    color: Texture2DMultisample,
    _c: std::marker::PhantomData<C>,
}

impl<C: TextureDataType> ColorTargetMultisample<C> {
    ///
    /// Constructs a new multisample color target with the given dimensions and number of samples.
    /// The number of samples must be larger than 0, less than or equal to the maximum number of samples supported by the hardware and power of two.
    ///
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        #[cfg(debug_assertions)]
        super::multisample_sanity_check(context, number_of_samples);
        Self {
            context: context.clone(),
            color: Texture2DMultisample::new::<C>(context, width, height, number_of_samples),
            _c: std::marker::PhantomData,
        }
    }

    ///
    /// Clears the color of this target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> &Self {
        self.clear_partially(
            ScissorBox::new_at_origo(self.width(), self.height()),
            clear_state,
        )
    }

    ///
    /// Clears the color of the part of this target that is inside the given scissor box.
    ///
    pub fn clear_partially(&self, scissor_box: ScissorBox, clear_state: ClearState) -> &Self {
        self.as_render_target().clear_partially(
            scissor_box,
            ClearState {
                depth: None,
                ..clear_state
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

    fn as_render_target(&self) -> RenderTarget<'_> {
        ColorTarget::new_texture_2d_multisample(&self.context, &self.color).as_render_target()
    }

    ///
    /// Resolves the multisample color target into the given non-multisample color target.
    /// The target must have the same width, height and [TextureDataType] as this target.
    ///
    pub fn resolve_to(&self, target: &ColorTarget<'_>) {
        self.as_render_target().blit_to(&target.as_render_target());
    }

    ///
    /// Resolves the multisample color target to a default non-multisample [Texture2D].
    /// Use [ColorTargetMultisample::resolve_to] to resolve to a custom non-multisample texture.
    ///
    pub fn resolve(&self) -> Texture2D {
        let mut color_texture = Texture2D::new_empty::<C>(
            &self.context,
            self.width(),
            self.height(),
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        self.resolve_to(&color_texture.as_color_target(None));
        color_texture
    }
}
