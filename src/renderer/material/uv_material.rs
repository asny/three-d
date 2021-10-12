use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct UVMaterial {
    pub render_states: RenderStates,
}

impl ForwardMaterial for UVMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool) -> String {
        include_str!("shaders/uv_material.frag").to_string()
    }
    fn use_uniforms(&self, _program: &Program, _camera: &Camera) -> Result<()> {
        Ok(())
    }
    fn render_states(&self, _transparent: bool) -> RenderStates {
        self.render_states
    }
    fn is_transparent(&self) -> bool {
        false
    }
}

impl ForwardMaterial for &UVMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool) -> String {
        (*self).fragment_shader_source(use_vertex_colors)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera) -> Result<()> {
        (*self).use_uniforms(program, camera)
    }
    fn render_states(&self, transparent: bool) -> RenderStates {
        (*self).render_states(transparent)
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}
