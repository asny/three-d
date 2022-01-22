use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct DepthMaterial {
    pub min_distance: Option<f32>,
    pub max_distance: Option<f32>,
    pub render_states: RenderStates,
}

impl Material for DepthMaterial {
    fn fragment_shader_source(
        &self,
        _use_vertex_colors: bool,
        _lights: impl std::iter::IntoIterator<Item = impl Light>,
    ) -> String {
        include_str!("shaders/depth_material.frag").to_string()
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: impl std::iter::IntoIterator<Item = impl Light>,
    ) -> ThreeDResult<()> {
        program.use_uniform_float("minDistance", &self.min_distance.unwrap_or(camera.z_near()))?;
        program.use_uniform_float("maxDistance", &self.max_distance.unwrap_or(camera.z_far()))?;
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn is_transparent(&self) -> bool {
        false
    }
}
