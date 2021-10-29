use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct PositionMaterial {
    pub render_states: RenderStates,
}

impl ForwardMaterial for PositionMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &Lights) -> String {
        include_str!("shaders/position_material.frag").to_string()
    }
    fn use_uniforms(
        &self,
        _program: &Program,
        _camera: &Camera,
        _lights: &Lights,
    ) -> ThreeDResult<()> {
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn is_transparent(&self) -> bool {
        false
    }
}
