use crate::core::*;
use crate::renderer::*;
use std::sync::Arc;

pub struct SkyboxMaterial {
    pub texture: Arc<TextureCubeMap>,
}

impl Material for SkyboxMaterial {
    fn id(&self) -> u16 {
        0b1u16 << 15 | 0b100u16
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

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes::NONE
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, _lights: &[&dyn Light]) {
        camera.tone_mapping.use_uniforms(program);
        camera.color_mapping.use_uniforms(program);
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
