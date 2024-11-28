use crate::core::*;
use crate::renderer::*;
use std::sync::Arc;

pub struct SkyboxMaterial {
    pub texture: Arc<TextureCubeMap>,
}

impl Material for SkyboxMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::SkyboxMaterial
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}{}{}",
            include_str!("../../core/shared.frag"),
            ToneMapping::fragment_shader_source(),
            ColorMapping::fragment_shader_source(),
            include_str!("shaders/skybox_material.frag")
        )
    }

    fn use_uniforms(&self, program: &Program, viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        viewer.tone_mapping().use_uniforms(program);
        viewer.color_mapping().use_uniforms(program);
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
