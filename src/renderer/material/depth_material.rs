use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct DepthMaterial {}

impl ForwardMaterial for DepthMaterial {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        _vertex_colors: VertexColors,
    ) -> String {
        "void main() {}".to_string()
    }
    fn bind(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) -> Result<()> {
        Ok(())
    }
    fn render_states(&self, _vertex_colors: VertexColors) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::DEPTH,
            ..Default::default()
        }
    }
}
