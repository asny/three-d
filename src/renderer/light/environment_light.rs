use crate::core::*;
use crate::renderer::*;

pub struct EnvironmentLight {
    irradiance_map: ColorTargetTextureCubeMap<f32>,
    prefilter_map: ColorTargetTextureCubeMap<f32>,
    brdf_map: ColorTargetTexture2D<f32>,
}

impl EnvironmentLight {
    pub fn new(context: &Context, environment_map: &impl TextureCube) -> ThreeDResult<Self> {
        // Diffuse
        let irradiance_map = ColorTargetTextureCubeMap::new(
            context,
            32,
            32,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;
        irradiance_map.write_to_all(
            ClearState::default(),
            &format!(
                "{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/irradiance.frag")
            ),
            |program| program.use_texture_cube("environmentMap", environment_map),
        )?;

        // Prefilter
        let prefilter_map = ColorTargetTextureCubeMap::new(
            context,
            128,
            128,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;
        prefilter_map.write_to_all(
            ClearState::default(),
            &format!(
                "#define COOK\n#define COOK_GGX\n{}{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/light_shared.frag"),
                include_str!("shaders/prefilter.frag")
            ),
            |program| program.use_texture_cube("environmentMap", environment_map),
        )?;

        // BRDF
        let brdf_map = ColorTargetTexture2D::new(
            context,
            512,
            512,
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RG,
        )?;
        let effect = ImageEffect::new(
            context,
            &format!(
                "#define COOK\n#define COOK_GGX\n{}{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/light_shared.frag"),
                include_str!("shaders/brdf.frag")
            ),
        )?;
        brdf_map.write(ClearState::default(), || {
            effect.apply(
                RenderStates::default(),
                Viewport::new_at_origo(brdf_map.width(), brdf_map.height()),
            )
        })?;

        Ok(Self {
            irradiance_map,
            prefilter_map,
            brdf_map,
        })
    }
}

impl Light for EnvironmentLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "
            uniform samplerCube irradianceMap;
            uniform samplerCube prefilterMap;
            uniform sampler2D brdfLUT;

            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                vec3 N = normal;
                vec3 V = normalize(eyePosition - position);
                vec3 R = reflect(-V, N); 
                
                // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0 
                // of 0.04 and if it's a metal, use the albedo color as F0 (metallic workflow)    
                vec3 F0 = vec3(0.04); 
                F0 = mix(F0, surface_color, metallic);

                // ambient lighting (we now use IBL as the ambient term)
                vec3 F = fresnelSchlickRoughness(max(dot(N, V), 0.0), F0, roughness);
                
                vec3 kS = F;
                vec3 kD = 1.0 - kS;
                kD *= 1.0 - metallic;	  
                
                vec3 irradiance = texture(irradianceMap, N).rgb;
                vec3 diffuse      = irradiance * surface_color;
                
                // sample both the pre-filter map and the BRDF lut and combine them together as per the Split-Sum approximation to get the IBL specular part.
                const float MAX_REFLECTION_LOD = 4.0;
                vec3 prefilteredColor = textureLod(prefilterMap, R,  roughness * MAX_REFLECTION_LOD).rgb;    
                vec2 brdf  = texture(brdfLUT, vec2(max(dot(N, V), 0.0), roughness)).rg;
                vec3 specular = prefilteredColor * (F * brdf.x + brdf.y);

                return (kD * diffuse + specular) * occlusion;
            }}
        
        ", i)
    }
    fn use_uniforms(&self, program: &Program, _camera: &Camera, i: u32) -> ThreeDResult<()> {
        program.use_texture_cube("irradianceMap", &self.irradiance_map)?;
        program.use_texture_cube("prefilterMap", &self.prefilter_map)?;
        Ok(())
    }
}
