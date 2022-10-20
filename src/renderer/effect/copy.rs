use crate::renderer::*;

///
/// Copies the content of a color and depth texture.
/// Only copies the channels given by the write mask.
///
pub struct CopyEffect {
    pub write_mask: WriteMask,
}

impl PostMaterial for CopyEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        let color_source = color_texture.fragment_shader_source();
        let depth_source = depth_texture.fragment_shader_source();

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

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        color_texture.use_uniforms(program);
        depth_texture.use_uniforms(program);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: self.write_mask,
            ..Default::default()
        }
    }
}
