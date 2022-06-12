use crate::core::*;
use crate::renderer::*;

///
/// A light which shines from the given position in all directions.
///
pub struct PointLight {
    /// The intensity of the light. This allows for higher intensity than 1 which can be used to simulate high intensity light sources like the sun.
    pub intensity: f32,
    /// The base color of the light.
    pub color: Color,
    /// The position of the light.
    pub position: Vec3,
    /// The [Attenuation] of the light.
    pub attenuation: Attenuation,
}

impl PointLight {
    /// Constructs a new point light.
    pub fn new(
        _context: &Context,
        intensity: f32,
        color: Color,
        position: &Vec3,
        attenuation: Attenuation,
    ) -> PointLight {
        PointLight {
            intensity,
            color,
            position: *position,
            attenuation,
        }
    }
}

impl Light for PointLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "
            uniform vec3 color{};
            uniform vec3 attenuation{};
            uniform vec3 position{};

            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
            {{
                vec3 light_direction = position{} - position;
                float distance = length(light_direction);
                light_direction = light_direction / distance;

                vec3 light_color = attenuate(color{}, attenuation{}, distance);
                return calculate_light(light_color, light_direction, surface_color, view_direction, normal, metallic, roughness);
            }}
        
        ", i, i, i, i, i, i, i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) {
        program.use_uniform(
            &format!("color{}", i),
            &(self.color.to_vec3() * self.intensity),
        );
        program.use_uniform(
            &format!("attenuation{}", i),
            &vec3(
                self.attenuation.constant,
                self.attenuation.linear,
                self.attenuation.quadratic,
            ),
        );
        program.use_uniform(&format!("position{}", i), &self.position);
    }
}
