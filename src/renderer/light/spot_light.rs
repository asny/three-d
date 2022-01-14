use crate::core::*;
use crate::renderer::light::*;
use crate::renderer::*;

///
/// A light which shines from the given position and in the given direction.
/// The light will cast shadows if you [generate a shadow map](SpotLight::generate_shadow_map).
///
pub struct SpotLight {
    context: Context,
    light_buffer: UniformBuffer,
    shadow_texture: Option<DepthTargetTexture2D>,
}

impl SpotLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        position: &Vec3,
        direction: &Vec3,
        cutoff: impl Into<Radians>,
        attenuation_constant: f32,
        attenuation_linear: f32,
        attenuation_exponential: f32,
    ) -> ThreeDResult<SpotLight> {
        let uniform_sizes = [3u32, 1, 1, 1, 1, 1, 3, 1, 3, 1, 16];
        let mut light = SpotLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &uniform_sizes)?,
            shadow_texture: None,
        };
        light.set_intensity(intensity);
        light.set_color(color);
        light.set_cutoff(cutoff);
        light.set_direction(direction);
        light.set_position(position);
        light.set_attenuation(
            attenuation_constant,
            attenuation_linear,
            attenuation_exponential,
        );
        Ok(light)
    }

    pub fn set_color(&mut self, color: Color) {
        self.light_buffer
            .update(
                0,
                &[
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                    color.a as f32 / 255.0,
                ],
            )
            .unwrap();
    }

    pub fn color(&self) -> Color {
        let c = self.light_buffer.get(0).unwrap();
        [c[0], c[1], c[2]].into()
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn intensity(&self) -> f32 {
        self.light_buffer.get(1).unwrap()[0]
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32) {
        self.light_buffer.update(2, &[constant]).unwrap();
        self.light_buffer.update(3, &[linear]).unwrap();
        self.light_buffer.update(4, &[exponential]).unwrap();
    }

    pub fn attenuation(&self) -> (f32, f32, f32) {
        (
            self.light_buffer.get(2).unwrap()[0],
            self.light_buffer.get(3).unwrap()[0],
            self.light_buffer.get(4).unwrap()[0],
        )
    }

    pub fn set_position(&mut self, position: &Vec3) {
        self.light_buffer.update(6, &position.to_slice()).unwrap();
    }

    pub fn position(&self) -> Vec3 {
        let p = self.light_buffer.get(6).unwrap();
        vec3(p[0], p[1], p[2])
    }

    pub fn set_cutoff(&mut self, cutoff: impl Into<Radians>) {
        self.light_buffer.update(7, &[cutoff.into().0]).unwrap();
    }

    pub fn cutoff(&self) -> Radians {
        radians(self.light_buffer.get(7).unwrap()[0])
    }

    pub fn set_direction(&mut self, direction: &Vec3) {
        self.light_buffer
            .update(8, &direction.normalize().to_slice())
            .unwrap();
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(8).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self) {
        self.shadow_texture = None;
        self.light_buffer.update(9, &[0.0]).unwrap();
    }

    pub fn generate_shadow_map(
        &mut self,
        texture_size: u32,
        geometries: &[impl Geometry],
    ) -> ThreeDResult<()> {
        let position = self.position();
        let direction = self.direction();
        let up = compute_up_direction(direction);

        let viewport = Viewport::new_at_origo(texture_size, texture_size);

        let mut z_far = 0.0f32;
        let mut z_near = f32::MAX;
        for geometry in geometries {
            let aabb = geometry.aabb();
            if !aabb.is_empty() {
                z_far = z_far.max(aabb.distance_max(&position));
                z_near = z_near.min(aabb.distance(&position));
            }
        }

        let shadow_camera = Camera::new_perspective(
            &self.context,
            viewport,
            position,
            position + direction,
            up,
            self.cutoff(),
            z_near.max(0.01),
            z_far,
        )?;
        self.light_buffer
            .update(10, &shadow_matrix(&shadow_camera).to_slice())?;

        let mut shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            texture_size,
            texture_size,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        let depth_material = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        shadow_texture.write(Some(1.0), || {
            for geometry in geometries
                .iter()
                .filter(|g| shadow_camera.in_frustum(&g.aabb()))
            {
                geometry.render_with_material(
                    &depth_material,
                    &shadow_camera,
                    &Lights::default(),
                )?;
            }
            Ok(())
        })?;
        self.shadow_texture = Some(shadow_texture);
        self.light_buffer.update(9, &[1.0])?;
        Ok(())
    }

    pub fn shadow_map(&self) -> Option<&DepthTargetTexture2D> {
        self.shadow_texture.as_ref()
    }

    pub fn buffer(&self) -> &UniformBuffer {
        &self.light_buffer
    }
}

impl Light for SpotLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "
            uniform sampler2D shadowMap{};
            layout (std140) uniform LightUniform{}
            {{
                BaseLight base{};
                Attenuation attenuation{};
                vec3 position{};
                float cutoff{};
                vec3 direction{};
                float shadowEnabled{};
                mat4 shadowMVP{};
            }};
            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
            {{
                if(base{}.intensity > 0.001) {{
                    vec3 light_direction = position{} - position;
                    float distance = length(light_direction);
                    light_direction = light_direction / distance;

                    float angle = acos(dot(-light_direction, normalize(direction{})));
                    float cutoff = cutoff{};
                
                    vec3 result = vec3(0.0);
                    if (angle < cutoff) {{
    
                        vec3 light_color = attenuate(base{}.intensity * base{}.color, attenuation{}, distance);
                        result = calculate_light(light_color, light_direction, surface_color, view_direction, normal, 
                            metallic, roughness) * (1.0 - smoothstep(0.75 * cutoff, cutoff, angle));
                        if(shadowEnabled{} > 0.5) {{
                            result *= calculate_shadow(shadowMap{}, shadowMVP{}, position);
                        }}
                    }}
                    return result;
                }}
                else {{
                    return vec3(0.0, 0.0, 0.0);
                }}
            }}
        
        ", i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        if let Some(tex) = self.shadow_map() {
            program.use_texture(&format!("shadowMap{}", i), tex)?;
        } else {
            self.context
                .use_texture_dummy(&program, &format!("shadowMap{}", i))?;
        }
        program.use_uniform_block(&format!("LightUniform{}", i), self.buffer());
        Ok(())
    }
}
