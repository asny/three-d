use crate::core::*;
use crate::renderer::light::*;
use crate::renderer::*;

///
/// A light which shines in the given direction.
/// The light will cast shadows if you [generate a shadow map](DirectionalLight::generate_shadow_map).
///
pub struct DirectionalLight {
    context: Context,
    shadow_texture: Option<DepthTexture2D>,
    shadow_matrix: Mat4,
    /// The intensity of the light. This allows for higher intensity than 1 which can be used to simulate high intensity light sources like the sun.
    pub intensity: f32,
    /// The base color of the light.
    pub color: Color,
    /// The direction the light shines.
    pub direction: Vec3,
}

impl DirectionalLight {
    /// Creates a new directional light.
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        direction: &Vec3,
    ) -> DirectionalLight {
        DirectionalLight {
            context: context.clone(),
            shadow_matrix: Mat4::identity(),
            shadow_texture: None,
            intensity,
            color,
            direction: *direction,
        }
    }

    ///
    /// Clear the shadow map, effectively disable the shadow.
    /// Only necessary if you want to disable the shadow, if you want to update the shadow, just use [DirectionalLight::generate_shadow_map].
    ///
    pub fn clear_shadow_map(&mut self) {
        self.shadow_texture = None;
        self.shadow_matrix = Mat4::identity();
    }

    ///
    /// Generate a shadow map which is used to simulate shadows from the directional light onto the geometries given as input.
    /// It is recomended that the texture size is power of 2.
    /// If the shadows are too low resolution (the edges between shadow and non-shadow are pixelated) try to increase the texture size
    /// and/or split the scene by creating another light source with same parameters and let the two light sources shines on different parts of the scene.
    ///
    pub fn generate_shadow_map(
        &mut self,
        texture_size: u32,
        geometries: impl IntoIterator<Item = impl Geometry> + Clone,
    ) {
        let up = compute_up_direction(self.direction);

        let viewport = Viewport::new_at_origo(texture_size, texture_size);
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for geometry in geometries.clone() {
            aabb.expand_with_aabb(&geometry.aabb());
        }
        if aabb.is_empty() {
            return;
        }
        let target = aabb.center();
        let position = target - aabb.max().distance(aabb.min()) * self.direction;
        let z_far = aabb.distance_max(&position);
        let z_near = aabb.distance(&position);
        let frustum_height = aabb.max().distance(aabb.min()); // TODO: more tight fit
        let shadow_camera = Camera::new_orthographic(
            viewport,
            position,
            target,
            up,
            frustum_height,
            z_near,
            z_far,
        );
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
        self.shadow_matrix = shadow_matrix(&shadow_camera);
    }

    ///
    /// Returns a reference to the shadow map if it has been generated.
    ///
    pub fn shadow_map(&self) -> Option<&DepthTexture2D> {
        self.shadow_texture.as_ref()
    }
}

impl Light for DirectionalLight {
    fn shader_source(&self, i: u32) -> String {
        if self.shadow_texture.is_some() {
            format!(
                "
                    uniform sampler2D shadowMap{};
                    uniform mat4 shadowMVP{};
        
                    uniform vec3 color{};
                    uniform vec3 direction{};
        
                    vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
                    {{
                        return calculate_light(color{}, -direction{}, surface_color, view_direction, normal, metallic, roughness) 
                            * calculate_shadow(shadowMap{}, shadowMVP{}, position);
                    }}
                
                ", i, i, i, i, i, i, i, i, i)
        } else {
            format!(
                "
                    uniform vec3 color{};
                    uniform vec3 direction{};
        
                    vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
                    {{
                        return calculate_light(color{}, -direction{}, surface_color, view_direction, normal, metallic, roughness);
                    }}
                
                ", i, i, i, i, i)
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
        program.use_uniform(&format!("direction{}", i), &self.direction.normalize());
    }
}
