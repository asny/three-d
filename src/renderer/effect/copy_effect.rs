use crate::renderer::*;

///
/// Copies the content of a color and depth texture.
///
#[derive(Default)]
pub struct CopyEffect {}

impl CopyEffect {
    pub fn render(
        context: &Context,
        write_mask: WriteMask,
        viewport: Viewport,
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        context.apply_effect(
            &Self::fragment_shader_source(color_texture, depth_texture),
            RenderStates {
                depth_test: DepthTest::Always,
                write_mask,
                ..Default::default()
            },
            viewport,
            |program| {
                color_texture.map(|t| t.use_uniforms(program));
                depth_texture.map(|t| t.use_uniforms(program));
            },
        )
    }

    fn fragment_shader_source(
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        let color_source = color_texture.map(|t| t.fragment_shader_source());
        let depth_source = depth_texture.map(|t| t.fragment_shader_source());

        if let Some(color_source) = color_source {
            if let Some(depth_source) = depth_source {
                let source = "
                    in vec2 uvs;
                    layout (location = 0) out vec4 color;
                    void main()
                    {
                        color = sample_color(uvs);
                        gl_FragDepth = sample_depth(uvs);
                    }";
                format!("{}\n{}\n{}", color_source, depth_source, source)
            } else {
                let source = "
                    uniform sampler2D colorMap;
                    in vec2 uvs;
                    layout (location = 0) out vec4 color;
                    void main()
                    {
                        color = texture(colorMap, uvs);
                    }";
                format!("{}\n{}", color_source, source)
            }
        } else {
            if let Some(depth_source) = depth_source {
                let source = "
                    in vec2 uvs;
                    void main()
                    {
                        gl_FragDepth = sample_depth(uvs);
                    }";
                format!("{}\n{}", depth_source, source)
            } else {
                panic!("Must supply a color or depth texture to apply a copy effect")
            }
        }
    }
}
