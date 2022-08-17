use crate::core::*;
use crate::renderer::*;

///
/// Used for rendering the distance from the camera to the object with this material in each pixel.
/// Can be used for debug purposes but is also used to create shadow maps from light sources.
///
#[derive(Default, Clone)]
pub struct DepthMaterial {
    /// The minimum distance from the camera to any object. If None, then the near plane of the camera is used.
    pub min_distance: Option<f32>,
    /// The maximum distance from the camera to any object. If None, then the far plane of the camera is used.
    pub max_distance: Option<f32>,
    /// Render states.
    pub render_states: RenderStates,
}

impl FromCpuMaterial for DepthMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for DepthMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/depth_material.frag").to_string()
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("minDistance", &self.min_distance.unwrap_or(camera.z_near()));
        program.use_uniform("maxDistance", &self.max_distance.unwrap_or(camera.z_far()));
        program.use_uniform("eye", camera.position());
    }
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
