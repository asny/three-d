use crate::core::*;
use crate::renderer::*;
use std::sync::Arc;

pub struct SkyboxMaterial {
    pub texture: Arc<TextureCubeMap>,
}

impl Material for SkyboxMaterial {
    fn fragment_shader_source(
        &self,
        _provided_attributes: FragmentAttributes,
        _lights: &[&dyn Light],
    ) -> String {
        format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/skybox_material.frag")
        )
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("isHDR", i32::from(self.texture.is_hdr()));
        program.use_texture_cube("texture0", &self.texture);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::LessOrEqual,
            cull: Cull::Front,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
