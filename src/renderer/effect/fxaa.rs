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
    #[deprecated = "use `apply_screen_effect` instead"]
    pub fn apply(&self, context: &Context, color_texture: ColorTexture) {
        apply_screen_effect(
            context,
            self,
            &camera2d(Viewport::new_at_origo(
                color_texture.width(),
                color_texture.height(),
            )),
            &[],
            Some(color_texture),
            None,
        );
    }
}

impl Effect for FxaaEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) -> String {
        let color_texture =
            color_texture.expect("Must supply a color texture to apply a fxaa effect");
        format!(
            "{}\n{}",
            color_texture.fragment_shader_source(),
            include_str!("shaders/fxaa_effect.frag")
        )
    }

    fn id(&self) -> u16 {
        0b11u16 << 14 | 0b10u16
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) {
        let color_texture =
            color_texture.expect("Must supply a color texture to apply a fxaa effect");
        let w = color_texture.width();
        let h = color_texture.height();
        color_texture.use_uniforms(program);
        program.use_uniform("resolution", vec2(w as f32, h as f32));
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            depth_test: DepthTest::Always,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}
