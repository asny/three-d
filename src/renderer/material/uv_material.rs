use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct UVMaterial {}

impl ForwardMaterial for UVMaterial {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        _vertex_colors: VertexColors,
    ) -> String {
        include_str!("shaders/uv_material.frag").to_string()
    }
    fn bind(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) -> Result<()> {
        Ok(())
    }
    fn render_states(&self, _vertex_colors: VertexColors) -> RenderStates {
        RenderStates::default()
    }
}
