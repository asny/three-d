use crate::renderer::*;

#[derive(Clone, Debug, Default)]
pub struct CopyEffect {}

impl Effect for CopyEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) -> String {
        let color_texture =
            color_texture.expect("Must supply a color texture to apply a copy effect");
        format!(
            "
            {}
            {}
            {}

            in vec2 uvs;

            layout (location = 0) out vec4 outColor;

            void main()
            {{
                outColor = sample_color(uvs);
                outColor.rgb = tone_mapping(outColor.rgb);
                outColor.rgb = color_mapping(outColor.rgb);
            }}

        ",
            color_texture.fragment_shader_source(),
            ToneMapping::fragment_shader_source(),
            ColorSpace::fragment_shader_source(),
        )
    }

    fn id(&self) -> u16 {
        0b11u16 << 14 | 0b101u16
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
        camera: &Camera,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) {
        camera.tone_mapping.use_uniforms(program);
        camera.target_color_space.use_uniforms(program);
        let color_texture =
            color_texture.expect("Must supply a color texture to apply a copy effect");
        color_texture.use_uniforms(program);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}
