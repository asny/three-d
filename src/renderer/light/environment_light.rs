use crate::core::*;
use crate::renderer::*;

pub struct EnvironmentLight {
    irradiance_map: ColorTargetTextureCubeMap<f32>,
}

impl EnvironmentLight {
    pub fn new(context: &Context, environment_map: &impl TextureCube) -> ThreeDResult<Self> {
        let vertex_buffer = VertexBuffer::new_with_static(context, &CPUMesh::cube().positions)?;
        let mut camera = Camera::new_perspective(
            context,
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, -1.0),
            vec3(0.0, 1.0, 0.0),
            degrees(90.0),
            0.1,
            10.0,
        )?;

        // Diffuse
        let program = Program::from_source(
            context,
            include_str!("shaders/cubemap.vert"),
            include_str!("shaders/irradiance.frag"),
        )?;
        let irradiance_map = ColorTargetTextureCubeMap::new(
            context,
            32,
            32,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::Repeat,
            Wrapping::Repeat,
            Wrapping::Repeat,
            Format::RGBA,
        )?;
        Self::render_to_cube(
            &program,
            &irradiance_map,
            &vertex_buffer,
            &mut camera,
            || program.use_texture_cube("environmentMap", environment_map),
        )?;

        Ok(Self { irradiance_map })
    }

    fn render_to_cube<T: TextureDataType>(
        program: &Program,
        target: &ColorTargetTextureCubeMap<T>,
        vertex_buffer: &VertexBuffer,
        camera: &mut Camera,
        use_uniforms: impl Fn() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        camera.set_viewport(Viewport::new_at_origo(target.width(), target.height()))?;
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
            program.use_attribute_vec3("position", &vertex_buffer)?;
            use_uniforms()?;
            target.write(i, ClearState::default(), || {
                program.draw_arrays(RenderStates::default(), camera.viewport(), 36);
                Ok(())
            })?;
        }
        Ok(())
    }
}

impl Light for EnvironmentLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "

            uniform samplerCube irradianceMap{};  // prefiltered env cubemap
            //uniform sampler2D iblbrdf; // IBL BRDF normalization precalculated tex

            vec3 fresnelSchlickRoughness(float NdV, vec3 F0, float roughness)
            {{
                return F0 + (max(vec3(1.0 - roughness), F0) - F0) * pow(clamp(1.0 - NdV, 0.0, 1.0), 5.0);
            }}

            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                vec3 V = normalize(eyePosition - position);
                float NdV = max(0.001, dot(normal, V));

                vec3 F0 = vec3(0.04); 
                F0 = mix(F0, surface_color, metallic);
                vec3 kS = fresnelSchlickRoughness(NdV, F0, roughness); 
                vec3 kD = 1.0 - kS;
                vec3 irradiance = texture(irradianceMap{}, normal).rgb;
                vec3 diffuse    = irradiance * surface_color;
                vec3 ambient = (kD * diffuse) * occlusion;
                return ambient;
            }}
        
        ", i, i, i)
    }
    fn use_uniforms(&self, program: &Program, _camera: &Camera, i: u32) -> ThreeDResult<()> {
        program.use_texture_cube(&format!("irradianceMap{}", i), &self.irradiance_map)?;
        Ok(())
    }
}
