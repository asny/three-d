use crate::renderer::*;

///
/// A simple anti-aliasing approach which smooths otherwise jagged edges (for example lines) but also
/// smooths the rest of the image.
///
pub struct FxaaEffect {
    context: Context,
}

impl FxaaEffect {
    ///
    /// Constructs a new FXAA effect.
    ///
    pub fn new(context: &Context) -> Self {
        Self {
            context: context.clone(),
        }
    }

    pub fn render(&self, viewport: Viewport, color_texture: ColorTexture) {
        self.context.apply_effect(
            &format!(
                "{}\n{}",
                color_texture.fragment_shader_source(),
                include_str!("shaders/fxaa_effect.frag")
            ),
            RenderStates {
                write_mask: WriteMask::COLOR,
                depth_test: DepthTest::Always,
                cull: Cull::Back,
                ..Default::default()
            },
            viewport,
            |program| {
                color_texture.use_uniforms(program);
                let (w, h) = color_texture.resolution();
                program.use_uniform("resolution", vec2(w as f32, h as f32));
            },
        )
    }
}
