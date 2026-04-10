use crate::core::*;
use crate::renderer::*;

pub struct WireframeMaterial {
    pub line_width: f32,
    pub line_color: Srgba,
}

impl Material for WireframeMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/wireframe_material.frag").to_owned()
    }

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::WireframeMaterial
    }

    fn use_uniforms(&self, program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_uniform("lineWidth", self.line_width);
        program.use_uniform("lineColor", Vec4::from(self.line_color));
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}
