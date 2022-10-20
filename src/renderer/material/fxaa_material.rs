use crate::renderer::*;

///
/// A simple anti-aliasing approach which smooths otherwise jagged edges (for example lines) but also
/// smooths the rest of the image.
///
pub struct FxaaMaterial {}

impl PostMaterial for FxaaMaterial {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        color_texture: ColorTexture,
        _depth_texture: DepthTexture,
    ) -> String {
        format!(
            "{}\n{}",
            color_texture
                .fragment_shader_source()
                .expect("Must supply a color texture to apply a FXAA effect"),
            include_str!("shaders/fxaa_material.frag")
        )
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: ColorTexture,
        _depth_texture: DepthTexture,
    ) {
        color_texture.use_uniforms(program);
        let (w, h) = color_texture
            .resolution()
            .expect("Must supply a color texture to apply a FXAA effect");
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
