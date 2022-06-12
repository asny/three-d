use crate::core::*;
use crate::renderer::*;

///
/// A light which shines on all surfaces.
/// Can be uniform (a light that shines equally on any surface) or calculated from an environment map using the [Environment] struct.
///
pub struct AmbientLight {
    /// The intensity of the light. This allows for higher intensity than 1 which can be used to simulate high intensity light sources like the sun.
    pub intensity: f32,
    /// The base color of the light.
    pub color: Color,
    /// The light shining from the environment. This is calculated based on an environment map.
    pub environment: Option<Environment>,
}

impl AmbientLight {
    /// Constructs an ambient light that shines equally on all surfaces.
    pub fn new(_context: &Context, intensity: f32, color: Color) -> Self {
        Self {
            intensity,
            color,
            environment: None,
        }
    }

    /// Constructs an ambient light that shines based on the given environment map.
    pub fn new_with_environment(
        context: &Context,
        intensity: f32,
        color: Color,
        environment_map: &TextureCubeMap,
    ) -> Self {
        Self {
            intensity,
            color,
            environment: Some(Environment::new(context, environment_map)),
        }
    }
}

impl Light for AmbientLight {
    fn shader_source(&self, i: u32) -> String {
        if self.environment.is_some() {
            format!(
            "
                uniform samplerCube irradianceMap;
                uniform samplerCube prefilterMap;
                uniform sampler2D brdfLUT;
                uniform vec3 ambientColor;
    
                vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
                {{
                    vec3 N = normal;
                    vec3 V = view_direction;
                    vec3 R = reflect(-V, N); 
                    float NdV = max(0.001, dot(N, V));
                    
                    // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0 
                    // of 0.04 and if it's a metal, use the albedo color as F0 (metallic workflow)    
                    vec3 F0 = mix(vec3(0.04), surface_color, metallic);
                    vec3 specular_fresnel = fresnel_schlick_roughness(F0, NdV, roughness);
                    vec3 diffuse_fresnel = 1.0 - specular_fresnel;

                    // Diffuse
                    vec3 irradiance = texture(irradianceMap, N).rgb;
                    vec3 diffuse = diffuse_fresnel * mix(surface_color, vec3(0.0), metallic) * irradiance;
                    
                    // sample both the pre-filter map and the BRDF lut and combine them together as per the Split-Sum approximation to get the IBL specular part.
                    const float MAX_REFLECTION_LOD = 4.0;
                    vec3 prefilteredColor = textureLod(prefilterMap, R,  roughness * MAX_REFLECTION_LOD).rgb;    
                    vec2 brdf  = texture(brdfLUT, vec2(NdV, roughness)).rg;
                    vec3 specular = prefilteredColor * (specular_fresnel * brdf.x + brdf.y);
    
                    return (diffuse + specular) * occlusion * ambientColor;
                }}
            
            ", i)
        } else {
            format!(
                "
                    uniform vec3 ambientColor;
                    vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
                    {{
                        return occlusion * ambientColor * mix(surface_color, vec3(0.0), metallic);
                    }}
                
                ", i)
        }
    }
    fn use_uniforms(&self, program: &Program, _i: u32) {
        if let Some(ref environment) = self.environment {
            program.use_texture_cube("irradianceMap", &environment.irradiance_map);
            program.use_texture_cube("prefilterMap", &environment.prefilter_map);
            program.use_texture("brdfLUT", &environment.brdf_map);
        }
        program.use_uniform("ambientColor", &(self.color.to_vec3() * self.intensity));
    }
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
            environment: None,
        }
    }
}
