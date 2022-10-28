use crate::renderer::*;

///
/// A simple anti-aliasing approach which smooths otherwise jagged edges (for example lines) but also
/// smooths the rest of the image.
///
#[derive(Clone, Default, Debug)]
pub struct FxaaEffect {}

impl FxaaEffect {
    ///
    /// Applies the FXAA effect to the given color texture.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn apply(&self, context: &Context, color_texture: ColorTexture) {
        apply_effect(
            context,
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
            Viewport::new_at_origo(color_texture.width(), color_texture.height()),
            |program| {
                color_texture.use_uniforms(program);
                let (w, h) = color_texture.resolution();
                program.use_uniform("resolution", vec2(w as f32, h as f32));
            },
        )
    }
}
