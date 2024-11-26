use crate::core::*;
use crate::renderer::*;

///
/// Render the object with colors that reflect its position which primarily is used for debug purposes.
/// The x coordinate maps to the red channel, y to green and z to blue.
///
#[derive(Default, Clone)]
pub struct PositionMaterial {
    /// Render states.
    pub render_states: RenderStates,
}

impl FromCpuMaterial for PositionMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for PositionMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::PositionMaterial
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/position_material.frag").to_string()
    }

    fn use_uniforms(&self, _program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {}

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
