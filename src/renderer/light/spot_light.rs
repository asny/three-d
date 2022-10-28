use crate::core::*;
use crate::renderer::light::*;
use crate::renderer::*;

///
/// A light which shines from the given position and in the given direction.
/// The light will cast shadows if you [generate a shadow map](SpotLight::generate_shadow_map).
///
pub struct SpotLight {
    context: Context,
    shadow_texture: Option<DepthTexture2D>,
    shadow_matrix: Mat4,
    /// The intensity of the light. This allows for higher intensity than 1 which can be used to simulate high intensity light sources like the sun.
    pub intensity: f32,
    /// The base color of the light.
    pub color: Color,
    /// The position of the light.
    pub position: Vec3,
    /// The direction the light shines.
    pub direction: Vec3,
    /// The cutoff angle for the light.
    pub cutoff: Radians,
    /// The [Attenuation] of the light.
    pub attenuation: Attenuation,
}

impl SpotLight {
    /// Constructs a new spot light.
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        position: &Vec3,
        direction: &Vec3,
        cutoff: impl Into<Radians>,
        attenuation: Attenuation,
    ) -> SpotLight {
        SpotLight {
            context: context.clone(),
            shadow_texture: None,
            intensity,
            color,
            position: *position,
            direction: *direction,
            cutoff: cutoff.into(),
            attenuation,
            shadow_matrix: Mat4::identity(),
        }
    }

    ///
    /// Clear the shadow map, effectively disable the shadow.
    /// Only necessary if you want to disable the shadow, if you want to update the shadow, just use [SpotLight::generate_shadow_map].
    ///
    pub fn clear_shadow_map(&mut self) {
        self.shadow_texture = None;
        self.shadow_matrix = Mat4::identity();
    }

    ///
    /// Generate a shadow map which is used to simulate shadows from the spot light onto the geometries given as input.
    /// It is recomended that the texture size is power of 2.
    /// If the shadows are too low resolution (the edges between shadow and non-shadow are pixelated) try to increase the texture size.
    ///
    pub fn generate_shadow_map(
        &mut self,
        texture_size: u32,
        geometries: impl IntoIterator<Item = impl Geometry> + Clone,
    ) {
        let position = self.position;
        let direction = self.direction;
        let up = compute_up_direction(self.direction);

        let viewport = Viewport::new_at_origo(texture_size, texture_size);

        let mut z_far = 0.0f32;
        let mut z_near = f32::MAX;
        for geometry in geometries.clone() {
            let aabb = geometry.aabb();
            if !aabb.is_empty() {
                z_far = z_far.max(aabb.distance_max(&self.position));
                z_near = z_near.min(aabb.distance(&self.position));
            }
        }

        let shadow_camera = Camera::new_perspective(
            viewport,
            position,
            position + direction,
            up,
            self.cutoff,
            z_near.max(0.01),
            z_far,
        );
        self.shadow_matrix = shadow_matrix(&shadow_camera);

        let mut shadow_texture = DepthTexture2D::new::<f32>(
            &self.context,
            texture_size,
            texture_size,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let depth_material = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        shadow_texture
            .as_depth_target()
            .clear(ClearState::default())
            .write(|| {
                for geometry in geometries
                    .into_iter()
                    .filter(|g| shadow_camera.in_frustum(&g.aabb()))
                {
                    geometry.render_with_material(&depth_material, &shadow_camera, &[]);
                }
            });
        self.shadow_texture = Some(shadow_texture);
    }

    ///
    /// Returns a reference to the shadow map if it has been generated.
    ///
    pub fn shadow_map(&self) -> Option<&DepthTexture2D> {
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
    fn use_uniforms(&self, program: &Program, i: u32) {
        if let Some(ref tex) = self.shadow_texture {
            program.use_depth_texture(&format!("shadowMap{}", i), tex);
            program.use_uniform(&format!("shadowMVP{}", i), &self.shadow_matrix);
        }
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
        program.use_uniform(&format!("direction{}", i), &self.direction.normalize());
        program.use_uniform(&format!("cutoff{}", i), &self.cutoff.0);
    }
}
