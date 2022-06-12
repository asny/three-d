use crate::core::*;

///
/// A simple anti-aliasing approach which smooths otherwise jagged edges (for example lines) but also
/// smooths the rest of the image.
///
pub struct FXAAEffect {
    image_effect: ImageEffect,
}

impl FXAAEffect {
    ///
    /// Creates a new FXAA effect.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            image_effect: ImageEffect::new(context, include_str!("shaders/fxaa.frag"))?,
        })
    }

    ///
    /// Applies the FXAA effect to the image in the given texture and writes the result to the given viewport of the current render target.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn apply(&self, viewport: Viewport, texture: &Texture2D) {
        let render_states = RenderStates {
            write_mask: WriteMask::COLOR,
            depth_test: DepthTest::Always,
            cull: Cull::Back,
            ..Default::default()
        };

        self.image_effect.use_texture("colorMap", &texture);
        self.image_effect.use_uniform(
            "resolution",
            vec2(texture.width() as f32, texture.height() as f32),
        );

        self.image_effect.apply(render_states, viewport);
    }
}
