use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct DepthMaterial {
    pub min_distance: f32,
    pub max_distance: f32,
    pub render_states: RenderStates,
}

impl DepthMaterial {
    pub fn new(camera: &Camera) -> Self {
        Self {
            min_distance: camera.z_near(),
            max_distance: camera.z_far(),
            render_states: RenderStates::default(),
        }
    }
}

impl ForwardMaterial for DepthMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool) -> String {
        include_str!("shaders/depth_material.frag").to_string()
    }
    fn use_uniforms(&self, program: &Program) -> Result<()> {
        program.use_uniform_float("minDistance", &self.min_distance)?;
        program.use_uniform_float("maxDistance", &self.max_distance)?;
        Ok(())
    }
    fn render_states(&self, _transparent: bool) -> RenderStates {
        self.render_states
    }
    fn is_transparent(&self) -> bool {
        false
    }
}

impl ForwardMaterial for &DepthMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool) -> String {
        (*self).fragment_shader_source(use_vertex_colors)
    }
    fn use_uniforms(&self, program: &Program) -> Result<()> {
        (*self).use_uniforms(program)
    }
    fn render_states(&self, transparent: bool) -> RenderStates {
        (*self).render_states(transparent)
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}
