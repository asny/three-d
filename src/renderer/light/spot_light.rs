use crate::core::*;
use crate::renderer::light::*;
use crate::renderer::*;

///
/// A light which shines from the given position and in the given direction.
/// The light will cast shadows if you [generate a shadow map](SpotLight::generate_shadow_map).
///
pub struct SpotLight {
    context: Context,
    shadow_texture: Option<DepthTargetTexture2D>,
    shadow_matrix: Mat4,
    pub intensity: f32,
    pub color: Color,
    pub position: Vec3,
    pub direction: Vec3,
    pub cutoff: Radians,
    pub attenuation: Attenuation,
}

impl SpotLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        position: &Vec3,
        direction: &Vec3,
        cutoff: impl Into<Radians>,
        attenuation: Attenuation,
    ) -> ThreeDResult<SpotLight> {
        Ok(SpotLight {
            context: context.clone(),
            shadow_texture: None,
            intensity,
            color,
            position: *position,
            direction: *direction,
            cutoff: cutoff.into(),
            attenuation,
            shadow_matrix: Mat4::identity(),
        })
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_attenuation(&mut self, attenuation: Attenuation) {
        self.attenuation = attenuation
    }

    pub fn attenuation(&self) -> Attenuation {
        self.attenuation
    }

    pub fn set_position(&mut self, position: &Vec3) {
        self.position = *position;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_cutoff(&mut self, cutoff: impl Into<Radians>) {
        self.cutoff = cutoff.into();
    }

    pub fn cutoff(&self) -> Radians {
        self.cutoff
    }

    pub fn set_direction(&mut self, direction: &Vec3) {
        self.direction = *direction;
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn clear_shadow_map(&mut self) {
        self.shadow_texture = None;
        self.shadow_matrix = Mat4::identity();
    }

    pub fn generate_shadow_map(
        &mut self,
        texture_size: u32,
        geometries: &[&dyn Geometry],
    ) -> ThreeDResult<()> {
        let position = self.position;
        let direction = self.direction;
        let up = compute_up_direction(self.direction);

        let viewport = Viewport::new_at_origo(texture_size, texture_size);

        let mut z_far = 0.0f32;
        let mut z_near = f32::MAX;
        for geometry in geometries {
            let aabb = geometry.aabb();
            if !aabb.is_empty() {
                z_far = z_far.max(aabb.distance_max(&self.position));
                z_near = z_near.min(aabb.distance(&self.position));
            }
        }

        let shadow_camera = Camera::new_perspective(
            &self.context,
            viewport,
            position,
            position + direction,
            up,
            self.cutoff,
            z_near.max(0.01),
            z_far,
        )?;
        self.shadow_matrix = shadow_matrix(&shadow_camera);

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
                geometry.render_with_material(&depth_material, &shadow_camera, &[])?;
            }
            Ok(())
        })?;
        self.shadow_texture = Some(shadow_texture);
        Ok(())
    }

    pub fn shadow_map(&self) -> Option<&DepthTargetTexture2D> {
        self.shadow_texture.as_ref()
    }
}

impl Light for SpotLight {
    fn shader_source(&self, i: u32) -> String {
        if self.shadow_texture.is_some() {
            format!(
                "
                    uniform sampler2D shadowMap{};
                    uniform mat4 shadowMVP{};
        
                    uniform vec3 color{};
                    uniform vec3 attenuation{};
                    uniform vec3 position{};
                    uniform float cutoff{};
                    uniform vec3 direction{};
                    vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
                    {{
                        vec3 light_direction = position{} - position;
                        float distance = length(light_direction);
                        light_direction = light_direction / distance;
        
                        float angle = acos(dot(-light_direction, normalize(direction{})));
                        float cutoff = cutoff{};
                    
                        vec3 result = vec3(0.0);
                        if (angle < cutoff) {{
                            vec3 light_color = attenuate(color{}, attenuation{}, distance);
                            result = calculate_light(light_color, light_direction, surface_color, view_direction, normal, 
                                metallic, roughness) * (1.0 - smoothstep(0.75 * cutoff, cutoff, angle));
                            result *= calculate_shadow(shadowMap{}, shadowMVP{}, position);
                        }}
                        return result;
                    }}
                
                ", i, i, i, i, i, i, i, i, i, i, i, i, i, i, i)
        } else {
            format!(
                "
                    uniform vec3 color{};
                    uniform vec3 attenuation{};
                    uniform vec3 position{};
                    uniform float cutoff{};
                    uniform vec3 direction{};
                    vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
                    {{
                        vec3 light_direction = position{} - position;
                        float distance = length(light_direction);
                        light_direction = light_direction / distance;
        
                        float angle = acos(dot(-light_direction, normalize(direction{})));
                        float cutoff = cutoff{};
                    
                        vec3 result = vec3(0.0);
                        if (angle < cutoff) {{
                            vec3 light_color = attenuate(color{}, attenuation{}, distance);
                            result = calculate_light(light_color, light_direction, surface_color, view_direction, normal, 
                                metallic, roughness) * (1.0 - smoothstep(0.75 * cutoff, cutoff, angle));
                        }}
                        return result;
                    }}
                
                ", i, i, i, i, i, i, i, i, i, i, i)
        }
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        if let Some(ref tex) = self.shadow_texture {
            program.use_texture(&format!("shadowMap{}", i), tex)?;
            program.use_uniform_mat4(&format!("shadowMVP{}", i), &self.shadow_matrix)?;
        }
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
        program.use_uniform_vec3(&format!("direction{}", i), &self.direction.normalize())?;
        program.use_uniform_float(&format!("cutoff{}", i), &self.cutoff.0)?;
        Ok(())
    }
}
