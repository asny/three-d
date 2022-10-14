use crate::renderer::*;

///
/// Copies the content of a color and depth texture.
/// Only copies the channels given by the write mask.
///
pub struct CopyEffect {
    pub write_mask: WriteMask,
}

impl EffectMaterial for CopyEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) -> String {
        if color_texture.is_none() && depth_texture.is_none() {
            panic!("Must supply a color or depth texture to apply a copy effect")
        }

        if color_texture.is_some() && depth_texture.is_some() {
            "
            uniform sampler2D colorMap;
            uniform sampler2D depthMap;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, uv);
                gl_FragDepth = texture(depthMap, uv).r;
            }"
        } else if color_texture.is_some() {
            "
            uniform sampler2D colorMap;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, uv);
            }"
        } else {
            "
            uniform sampler2D depthMap;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                gl_FragDepth = texture(depthMap, uv).r;
            }"
        }
        .to_owned()
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) {
        if let Some(tex) = color_texture {
            program.use_texture("colorMap", tex);
        }
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
