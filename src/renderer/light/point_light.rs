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
    ) -> ThreeDResult<PointLight> {
        Ok(PointLight {
            intensity,
            color,
            position: *position,
            attenuation,
        })
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn color(&self) -> Color {
        self.color
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_attenuation(&mut self, attenuation: Attenuation) {
        self.attenuation = attenuation
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn attenuation(&self) -> Attenuation {
        self.attenuation
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_position(&mut self, position: &Vec3) {
        self.position = *position;
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn position(&self) -> Vec3 {
        self.position
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
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        program.use_uniform_vec3(
            &format!("color{}", i),
            &(self.color.to_vec3() * self.intensity),
        )?;
        program.use_uniform_vec3(
            &format!("attenuation{}", i),
            &vec3(
                self.attenuation.constant,
                self.attenuation.linear,
                self.attenuation.quadratic,
            ),
        )?;
        program.use_uniform_vec3(&format!("position{}", i), &self.position)?;
        Ok(())
    }
}
