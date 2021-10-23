use crate::core::*;
use crate::renderer::*;

///
/// A light which shines equally on all parts of any surface.
///
#[derive(Clone, Debug)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}

impl Light for AmbientLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "
            uniform vec3 ambientColor{};
            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                return occlusion * ambientColor{} * mix(surface_color, vec3(0.0), metallic);
            }}
        
        ", i, i, i)
    }
    fn use_uniforms(&self, program: &Program, _camera: &Camera, i: u32) -> ThreeDResult<()> {
        program.use_uniform_vec3(
            &format!("ambientColor{}", i),
            &(self.color.to_vec3() * self.intensity),
        )
    }
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
        }
    }
}
