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
        depth_texture: Option<&DepthTargetTexture2D>,
    ) -> String {
        let color_source = color_texture.fragment_shader_source();

        if color_source.is_none() && depth_texture.is_none() {
            panic!("Must supply a color or depth texture to apply a copy effect")
        }

        if let Some(color_source) = color_source {
            if depth_texture.is_some() {
                let source = "
                    uniform sampler2D depthMap;
                    in vec2 uvs;
                    layout (location = 0) out vec4 color;
                    void main()
                    {
                        color = sample_color(uvs);
                        gl_FragDepth = texture(depthMap, uvs).r;
                    }";
                format!("{}\n{}", color_source, source)
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
            "
            uniform sampler2D depthMap;
            in vec2 uvs;
            void main()
            {
                gl_FragDepth = texture(depthMap, uvs).r;
            }"
            .to_owned()
        }
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) {
        color_texture.use_uniforms(program);
        if let Some(tex) = depth_texture {
            program.use_depth_texture("depthMap", tex);
        }
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: self.write_mask,
            ..Default::default()
        }
    }
}
