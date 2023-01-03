use crate::core::*;

pub struct RenderTargetMultisample {
    context: Context,
    color: Option<Texture2DMultisample>,
    depth: Option<DepthTexture2DMultisample>,
}

impl RenderTargetMultisample {
    pub fn new<C: TextureDataType, D: DepthTextureDataType>(
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
            depth: Some(DepthTexture2DMultisample::new::<D>(
                context,
                width,
                height,
                number_of_samples,
            )),
        }
    }

    pub fn new_color<C: TextureDataType>(
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
        }
    }

    pub fn new_depth<D: DepthTextureDataType>(
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
        }
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

    pub fn as_render_target(&mut self) -> RenderTarget<'_> {
        if let Some(color) = &mut self.color {
            if let Some(depth) = &mut self.depth {
                RenderTarget::new(color.as_color_target(), depth.as_depth_target())
            } else {
                RenderTarget::new_color(color.as_color_target())
            }
        } else {
            RenderTarget::new_depth(self.depth.as_mut().unwrap().as_depth_target())
        }
    }
    pub fn resolve_color(&self) -> Option<Texture2D> {
        if let Some(source_color) = &self.color {
            let mut target_color = Texture2D::new_empty::<[u8; 4]>(
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
            Some(target_color)
        } else {
            None
        }
    }

    pub fn resolve_depth(&self) -> Option<DepthTexture2D> {
        if let Some(source_depth) = &self.depth {
            let mut target_depth = DepthTexture2D::new::<f32>(
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
            Some(target_depth)
        } else {
            None
        }
    }

    pub fn resolve(&self) -> (Option<Texture2D>, Option<DepthTexture2D>) {
        if let Some(source_color) = &self.color {
            if let Some(source_depth) = &self.depth {
                let mut target_color = Texture2D::new_empty::<[u8; 4]>(
                    &self.context,
                    source_color.width(),
                    source_color.height(),
                    Interpolation::Nearest,
                    Interpolation::Nearest,
                    None,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                let mut target_depth = DepthTexture2D::new::<f32>(
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
                (Some(target_color), Some(target_depth))
            } else {
                (self.resolve_color(), None)
            }
        } else {
            (None, self.resolve_depth())
        }
    }
}
