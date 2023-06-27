use crate::renderer::*;

#[derive(Clone, Debug, Default)]
pub struct CopyEffect {}

impl Effect for CopyEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        format!(
            "{}{}{}{}

            in vec2 uvs;
            layout (location = 0) out vec4 outColor;

            void main()
            {{
                {}
                {}
            }}

        ",
            color_texture
                .map(|t| t.fragment_shader_source())
                .unwrap_or("".to_string()),
            depth_texture
                .map(|t| t.fragment_shader_source())
                .unwrap_or("".to_string()),
            ToneMapping::fragment_shader_source(),
            ColorSpace::fragment_shader_source(),
            color_texture
                .map(|_| "
                    outColor = sample_color(uvs);
                    outColor.rgb = tone_mapping(outColor.rgb);
                    outColor.rgb = color_mapping(outColor.rgb);"
                    .to_string())
                .unwrap_or("".to_string()),
            depth_texture
                .map(|_| "gl_FragDepth = sample_depth(uvs);".to_string())
                .unwrap_or("".to_string()),
        )
    }

    fn id(&self, color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16 {
        0b11u16 << 14
            | 0b100u16
            | color_texture.map(|t| t.id()).unwrap_or(0u16)
            | depth_texture.map(|t| t.id()).unwrap_or(0u16)
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
        depth_texture: Option<DepthTexture>,
    ) {
        camera.tone_mapping.use_uniforms(program);
        camera.target_color_space.use_uniforms(program);
        if let Some(color_texture) = color_texture {
            color_texture.use_uniforms(program);
        }
        if let Some(depth_texture) = depth_texture {
            depth_texture.use_uniforms(program);
        }
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}
