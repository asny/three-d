use crate::core::*;
use crate::renderer::*;

///
/// A light which shines equally on all parts of any surface.
///
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
    pub environment: Option<Environment>,
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
    
                vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
                {{
                    vec3 N = normal;
                    vec3 V = normalize(eyePosition - position);
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
                    vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
                    {{
                        return occlusion * ambientColor * mix(surface_color, vec3(0.0), metallic);
                    }}
                
                ", i)
        }
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, i: u32) -> ThreeDResult<()> {
        if let Some(ref environment) = self.environment {
            program.use_texture_cube("irradianceMap", &environment.irradiance_map)?;
            program.use_texture_cube("prefilterMap", &environment.prefilter_map)?;
            program.use_texture("brdfLUT", &environment.brdf_map)?;
            program.use_uniform_vec3("eyePosition", camera.position())?;
        }
        program.use_uniform_vec3("ambientColor", &(self.color.to_vec3() * self.intensity))
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
