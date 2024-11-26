use crate::renderer::*;

///
/// Copies the content of the color and/or depth texture by rendering a quad with those textures applied.
/// The difference from [ScreenEffect] is that this effect does not apply any tone and color mapping specified in the [Viewer].
///
#[derive(Clone, Debug, Default)]
pub struct CopyEffect {
    /// Defines which channels (red, green, blue, alpha and depth) to copy.
    pub write_mask: WriteMask,
    /// Defines which type of blending to use when writing the copied color to the render target.
    pub blend: Blend,
}

impl Effect for CopyEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        format!(
            "{}{}

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
            color_texture
                .map(|_| "
                    outColor = sample_color(uvs);"
                    .to_string())
                .unwrap_or("".to_string()),
            depth_texture
                .map(|_| "gl_FragDepth = sample_depth(uvs);".to_string())
                .unwrap_or("".to_string()),
        )
    }

    fn id(
        &self,
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> EffectMaterialId {
        EffectMaterialId::CopyEffect(color_texture, depth_texture)
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _viewer: &dyn Viewer,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        if let Some(color_texture) = color_texture {
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
