use crate::core::*;
use crate::renderer::*;

pub struct EnvironmentLight {
    irradiance_map: ColorTargetTextureCubeMap<f32>,
}

impl EnvironmentLight {
    pub fn new(context: &Context, environment_map: &impl TextureCube) -> ThreeDResult<Self> {
        let program = Program::from_source(
            context,
            include_str!("shaders/irradiance.vert"),
            include_str!("shaders/irradiance.frag"),
        )?;
        let vertex_buffer = VertexBuffer::new_with_static(context, &CPUMesh::cube().positions)?;
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

        let mut camera = Camera::new_perspective(
            context,
            Viewport::new_at_origo(irradiance_map.width(), irradiance_map.height()),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, -1.0),
            vec3(0.0, 1.0, 0.0),
            degrees(90.0),
            0.1,
            10.0,
        )?;
        for i in 0..6 {
            match i {
                0 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                1 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(-1.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                2 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ),
                3 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                    vec3(0.0, 0.0, -1.0),
                ),
                4 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                5 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 0.0, -1.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                _ => unreachable!(),
            }?;

            program.use_uniform_block("Camera", camera.uniform_buffer());
            program.use_texture_cube("environmentMap", environment_map)?;
            program.use_attribute_vec3("position", &vertex_buffer)?;
            irradiance_map.write(i, ClearState::default(), || {
                program.draw_arrays(RenderStates::default(), camera.viewport(), 36);
                Ok(())
            })?;
        }
        Ok(Self { irradiance_map })
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
