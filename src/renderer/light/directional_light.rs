use crate::core::*;
use crate::renderer::light::*;
use crate::renderer::*;

///
/// A light which shines in the given direction.
/// The light will cast shadows if you [generate a shadow map](DirectionalLight::generate_shadow_map).
///
pub struct DirectionalLight {
    context: Context,
    shadow_texture: Option<DepthTargetTexture2D>,
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
    ) -> ThreeDResult<DirectionalLight> {
        Ok(DirectionalLight {
            context: context.clone(),
            shadow_matrix: Mat4::identity(),
            shadow_texture: None,
            intensity,
            color,
            direction: *direction,
        })
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn color(&self) -> Color {
        self.color
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn set_direction(&mut self, direction: &Vec3) {
        self.direction = *direction;
    }

    #[deprecated]
    #[allow(missing_docs)]
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
        let up = compute_up_direction(self.direction);

        let viewport = Viewport::new_at_origo(texture_size, texture_size);
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for geometry in geometries {
            aabb.expand_with_aabb(&geometry.aabb());
        }
        if aabb.is_empty() {
            return Ok(());
        }
        let target = aabb.center();
        let position = target - self.direction;
        let z_far = aabb.distance_max(&position);
        let z_near = aabb.distance(&position);
        let frustum_height = aabb.max().distance(aabb.min()); // TODO: more tight fit
        let shadow_camera = Camera::new_orthographic(
            &self.context,
            viewport,
            position,
            target,
            up,
            frustum_height,
            z_near,
            z_far,
        )?;
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
        self.shadow_matrix = shadow_matrix(&shadow_camera);
        Ok(())
    }

    pub fn shadow_map(&self) -> Option<&DepthTargetTexture2D> {
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
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        if let Some(ref tex) = self.shadow_texture {
            program.use_texture(&format!("shadowMap{}", i), tex)?;
            program.use_uniform_mat4(&format!("shadowMVP{}", i), &self.shadow_matrix)?;
        }
        program.use_uniform_vec3(
            &format!("color{}", i),
            &(self.color.to_vec3() * self.intensity),
        )?;
        program.use_uniform_vec3(&format!("direction{}", i), &self.direction.normalize())?;
        Ok(())
    }
}
