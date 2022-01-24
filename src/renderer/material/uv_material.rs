use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct UVMaterial {
    pub render_states: RenderStates,
}

impl Material for UVMaterial {
    fn fragment_shader_source(
        &self,
        _use_vertex_colors: bool,
        _lights: &mut dyn std::iter::Iterator<Item = &dyn Light>,
    ) -> String {
        include_str!("shaders/uv_material.frag").to_string()
    }
    fn use_uniforms(
        &self,
        _program: &Program,
        _camera: &Camera,
        _lights: &mut dyn std::iter::Iterator<Item = &dyn Light>,
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
