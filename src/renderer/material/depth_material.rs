use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct DepthMaterial {}

impl ForwardMaterial for DepthMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light], _use_vertex_colors: bool) -> String {
        "void main() {}".to_string()
    }
    fn bind(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) -> Result<()> {
        Ok(())
    }
    fn render_states(&self, _transparent: bool) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::DEPTH,
            ..Default::default()
        }
    }
}
