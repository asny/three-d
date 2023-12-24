use crate::renderer::*;

///
/// Renders a full screen quad with the content of the color and/or depth textures.
/// The difference from [CopyEffect] is that this effect also applies any mapping set in the [Camera].
///
#[derive(Clone, Debug, Default)]
pub struct ScreenEffect {
    /// Defines which channels (red, green, blue, alpha and depth) to render into.
    pub write_mask: WriteMask,
    /// Defines which type of blending to use when writing the color to the render target.
    pub blend: Blend,
}

impl Effect for ScreenEffect {
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
            ColorMapping::fragment_shader_source(),
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
        0b1u16 << 14
            | 0b1u16 << 13
            | 0b1u16 << 11
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
        if let Some(color_texture) = color_texture {
            camera.tone_mapping.use_uniforms(program);
            camera.color_mapping.use_uniforms(program);
            color_texture.use_uniforms(program);
        }
        if let Some(depth_texture) = depth_texture {
            depth_texture.use_uniforms(program);
        }
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            cull: Cull::Back,
            write_mask: self.write_mask,
            blend: self.blend,
        }
    }
}
