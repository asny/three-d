use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct DepthMaterial {}

impl ForwardMaterial for DepthMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        "void main() {}".to_string()
    }
    fn bind(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) -> Result<()> {
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::DEPTH,
            ..Default::default()
        }
    }
}
