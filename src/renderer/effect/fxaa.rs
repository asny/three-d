use crate::renderer::*;

///
/// A simple anti-aliasing approach which smooths otherwise jagged edges (for example lines) but also
/// smooths the rest of the image.
///
pub struct FXAAEffect {
    context: Context,
}

impl FXAAEffect {
    ///
    /// Creates a new FXAA effect.
    ///
    pub fn new(context: &Context) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

impl EffectMaterial for FXAAEffect {
    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        include_str!("shaders/fxaa.frag").to_owned()
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        _depth_texture: Option<&DepthTargetTexture2D>,
    ) {
        let texture = color_texture.expect("Must supply a color texture to apply a fog effect");
        program.use_texture("colorMap", texture);
        program.use_uniform(
            "resolution",
            vec2(texture.width() as f32, texture.height() as f32),
        );
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
