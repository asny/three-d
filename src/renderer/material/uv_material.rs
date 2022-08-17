use crate::core::*;
use crate::renderer::*;

///
/// Render the object with colors that reflect its uv coordinates which primarily is used for debug purposes.
/// The u coordinate maps to the red channel and the v coordinate to the green channel.
///
#[derive(Default, Clone)]
pub struct UVMaterial {
    /// Render states.
    pub render_states: RenderStates,
}

impl FromCpuMaterial for UVMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for UVMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/uv_material.frag").to_string()
    }
    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {}
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
