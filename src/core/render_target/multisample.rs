use crate::core::*;

pub struct RenderTargetMultisample<C: TextureDataType, D: DepthTextureDataType> {
    pub(crate) context: Context,
    color: Option<Texture2DMultisample>,
    depth: Option<DepthTexture2DMultisample>,
    _c: C,
    _d: D,
}

impl<C: TextureDataType + Default, D: DepthTextureDataType + Default>
    RenderTargetMultisample<C, D>
{
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        Self {
            context: context.clone(),
            color: Some(Texture2DMultisample::new::<C>(
                context,
                width,
                height,
                number_of_samples,
            )),
            depth: Some(DepthTexture2DMultisample::new::<D>(
                context,
                width,
                height,
                number_of_samples,
            )),
            _c: C::default(),
            _d: D::default(),
        }
    }

    pub(in crate::core) fn new_color(
        context: &Context,
        width: u32,
        height: u32,
        number_of_samples: u32,
    ) -> Self {
        Self {
            context: context.clone(),
            color: Some(Texture2DMultisample::new::<C>(
                context,
                width,
                height,
                number_of_samples,
            )),
            depth: None,
            _c: C::default(),
            _d: D::default(),
        }
    }

    pub(in crate::core) fn new_depth(
        context: &Context,
        width: u32,
        height: u32,
        number_of_samples: u32,
    ) -> Self {
        Self {
            context: context.clone(),
            color: None,
            depth: Some(DepthTexture2DMultisample::new::<D>(
                context,
                width,
                height,
                number_of_samples,
            )),
            _c: C::default(),
            _d: D::default(),
        }
    }

    ///
    /// Clears the color and depth of this target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> &Self {
        self.clear_partially(
            ScissorBox::new_at_origo(self.width(), self.height()),
            clear_state,
        )
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

    /// The width of this target.
    pub fn width(&self) -> u32 {
        self.color
            .as_ref()
            .map(|t| t.width())
            .unwrap_or_else(|| self.depth.as_ref().unwrap().width())
    }

    /// The height of this target.
    pub fn height(&self) -> u32 {
        self.color
            .as_ref()
            .map(|t| t.height())
            .unwrap_or_else(|| self.depth.as_ref().unwrap().height())
    }

    /// The number of samples for each fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.color
            .as_ref()
            .map(|t| t.number_of_samples())
            .unwrap_or_else(|| self.depth.as_ref().unwrap().number_of_samples())
    }

    pub(super) fn as_render_target(&self) -> RenderTarget<'_> {
        if let Some(color) = &self.color {
            if let Some(depth) = &self.depth {
                RenderTarget::new(color.as_color_target(), depth.as_depth_target())
            } else {
                RenderTarget::new_color(color.as_color_target())
            }
        } else {
            RenderTarget::new_depth(self.depth.as_ref().unwrap().as_depth_target())
        }
    }

    pub fn resolve_color(&self) -> Texture2D {
        if let Some(source_color) = &self.color {
            let mut target_color = Texture2D::new_empty::<C>(
                &self.context,
                source_color.width(),
                source_color.height(),
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            source_color
                .as_color_target()
                .as_render_target()
                .blit_to(&target_color.as_color_target(None).as_render_target());
            target_color
        } else {
            panic!("Cannot resolve color on a multisampled depth target")
        }
    }

    pub fn resolve_depth(&self) -> DepthTexture2D {
        if let Some(source_depth) = &self.depth {
            let mut target_depth = DepthTexture2D::new::<D>(
                &self.context,
                source_depth.width(),
                source_depth.height(),
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            source_depth
                .as_depth_target()
                .as_render_target()
                .blit_to(&target_depth.as_depth_target().as_render_target());
            target_depth
        } else {
            panic!("Cannot resolve depth on a multisampled color target")
        }
    }

    pub fn resolve(&self) -> (Texture2D, DepthTexture2D) {
        if let Some(source_color) = &self.color {
            if let Some(source_depth) = &self.depth {
                let mut target_color = Texture2D::new_empty::<C>(
                    &self.context,
                    source_color.width(),
                    source_color.height(),
                    Interpolation::Nearest,
                    Interpolation::Nearest,
                    None,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                let mut target_depth = DepthTexture2D::new::<D>(
                    &self.context,
                    source_depth.width(),
                    source_depth.height(),
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                RenderTarget::new(
                    source_color.as_color_target(),
                    source_depth.as_depth_target(),
                )
                .blit_to(&RenderTarget::new(
                    target_color.as_color_target(None),
                    target_depth.as_depth_target(),
                ));
                (target_color, target_depth)
            } else {
                panic!(
                    "Cannot resolve both color and depth on a multisampled color or depth target"
                )
            }
        } else {
            panic!("Cannot resolve both color and depth on a multisampled color or depth target")
        }
    }
}
