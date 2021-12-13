use crate::core::*;
use crate::renderer::*;

///
/// A light which shines equally on all parts of any surface.
///
pub struct EnvironmentLight {
    irradiance_map: TextureCubeMap,
}

impl EnvironmentLight {
    pub fn new<T: TextureDataType>(
        context: &Context,
        cpu_texture: &CPUTexture<T>,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            irradiance_map: TextureCubeMap::new(context, cpu_texture)?,
        })
    }
}

impl Light for EnvironmentLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "

            uniform samplerCube irradianceMap{};  // prefiltered env cubemap
            //uniform sampler2D iblbrdf; // IBL BRDF normalization precalculated tex

            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                vec3 V = normalize(eyePosition - position);
                float NdV = max(0.001, dot(normal, V));
            
                vec3 diffuse_light = texture(irradianceMap{}, normal).rgb / PI;
            
                // specular IBL term
                //    11 magic number is total MIP levels in cubemap, this is simplest way for picking
                //    MIP level from roughness value (but it's not correct, however it looks fine)
                //vec3 refl = tnrm * reflect(-V, N);
                //vec3 reflected_light = textureCubeLod(
                //    irradianceMap, refl, max(roughness * 11.0, textureQueryLod(irradianceMap, refl).y)
                //).xyz;
            
                //vec2 brdf = texture2D(iblbrdf, vec2(roughness, 1.0 - NdV)).xy;
                //vec3 iblspec = min(vec3(0.99), fresnel_factor(specular, NdV) * brdf.x + brdf.y);

                return occlusion * diffuse_light * mix(surface_color, vec3(0.0), metallic); // + reflected_light * iblspec;
            }}
        
        ", i, i, i)
    }
    fn use_uniforms(&self, program: &Program, _camera: &Camera, i: u32) -> ThreeDResult<()> {
        program.use_texture_cube(&format!("irradianceMap{}", i), &self.irradiance_map)?;
        Ok(())
    }
}
