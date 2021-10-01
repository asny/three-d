use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct PickMaterial {
    pub min_distance: Option<f32>,
    pub max_distance: Option<f32>,
}

impl ForwardMaterial for PickMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light], _use_vertex_colors: bool) -> String {
        include_str!("shaders/pick_material.frag").to_string()
    }
    fn bind(&self, program: &Program, camera: &Camera, _lights: &[&dyn Light]) -> Result<()> {
        program.use_uniform_float("minDistance", &self.min_distance.unwrap_or(camera.z_near()))?;
        program.use_uniform_float("maxDistance", &self.max_distance.unwrap_or(camera.z_far()))?;
        Ok(())
    }
    fn render_states(&self, _transparent: bool) -> RenderStates {
        RenderStates::default()
    }
}
