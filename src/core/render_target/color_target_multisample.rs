use crate::core::*;

pub struct ColorTargetMultisample<C: TextureDataType> {
    target: RenderTargetMultisample<C, f32>,
    pub(crate) context: Context,
}

impl<C: TextureDataType + Default> ColorTargetMultisample<C> {
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        Self {
            target: RenderTargetMultisample::new_color(context, width, height, number_of_samples),
            context: context.clone(),
        }
    }

    /// The width of this target.
    pub fn width(&self) -> u32 {
        self.target.width()
    }

    /// The height of this target.
    pub fn height(&self) -> u32 {
        self.target.height()
    }

    /// The number of samples for each fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.target.number_of_samples()
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
        self.write_partially(
            ScissorBox::new_at_origo(self.width(), self.height()),
            render,
        )
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this target defined by the scissor box.
    ///
    pub fn write_partially(&self, scissor_box: ScissorBox, render: impl FnOnce()) -> &Self {
        self.as_render_target().write_partially(scissor_box, render);
        self
    }

    fn as_render_target(&self) -> RenderTarget<'_> {
        self.target.as_render_target()
    }

    pub fn resolve(&self) -> Texture2D {
        self.target.resolve_color()
    }
}
