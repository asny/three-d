use crate::core::*;
use crate::renderer::*;

///
/// A light which shines in the given direction.
/// The light will cast shadows if you [generate a shadow map](DirectionalLight::generate_shadow_map).
///
pub struct DirectionalLight {
    context: Context,
    light_buffer: UniformBuffer,
    shadow_texture: DepthTargetTexture2D,
    shadow_camera: Option<Camera>,
}

impl DirectionalLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        direction: &Vec3,
    ) -> Result<DirectionalLight> {
        let mut light = DirectionalLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &[3u32, 1, 3, 1, 16])?,
            shadow_texture: DepthTargetTexture2D::new(
                context,
                1,
                1,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            )?,
            shadow_camera: None,
        };

        light.set_intensity(intensity);
        light.set_color(color);
        light.set_direction(direction);
        Ok(light)
    }

    pub fn set_color(&mut self, color: Color) {
        self.light_buffer.update(0, &color.to_rgb_slice()).unwrap();
    }

    pub fn color(&self) -> Color {
        let c = self.light_buffer.get(0).unwrap();
        Color::new_from_rgb_slice(&[c[0], c[1], c[2]])
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn intensity(&self) -> f32 {
        self.light_buffer.get(1).unwrap()[0]
    }

    pub fn set_direction(&mut self, direction: &Vec3) {
        self.light_buffer
            .update(2, &direction.normalize().to_slice())
            .unwrap();
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(2).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self) {
        self.shadow_camera = None;
        self.shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            1,
            1,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )
        .unwrap();
        self.light_buffer.update(3, &[0.0]).unwrap();
    }

    pub fn generate_shadow_map(
        &mut self,
        target: &Vec3,
        frustrum_height: f32,
        frustrum_depth: f32,
        texture_width: u32,
        texture_height: u32,
        objects: &[&dyn Object],
    ) -> Result<()> {
        let direction = self.direction();
        let up = compute_up_direction(direction);

        let viewport = Viewport::new_at_origo(texture_width, texture_height);
        self.shadow_camera = Some(Camera::new_orthographic(
            &self.context,
            viewport,
            target - direction.normalize() * 0.5 * frustrum_depth,
            *target,
            up,
            frustrum_height,
            0.0,
            frustrum_depth,
        )?);
        self.light_buffer.update(
            4,
            &shadow_matrix(self.shadow_camera.as_ref().unwrap()).to_slice(),
        )?;

        self.shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            texture_width,
            texture_height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )
        .unwrap();
        self.shadow_texture.write(Some(1.0), || {
            for object in objects {
                if in_frustum(self.shadow_camera.as_ref().unwrap(), object) {
                    object.render_forward(
                        &DepthMaterial::default(),
                        self.shadow_camera.as_ref().unwrap(),
                        &Lights::NONE,
                    )?;
                }
            }
            Ok(())
        })?;
        self.light_buffer.update(3, &[1.0])?;
        Ok(())
    }

    pub fn shadow_map(&self) -> &DepthTargetTexture2D {
        &self.shadow_texture
    }

    pub fn buffer(&self) -> &UniformBuffer {
        &self.light_buffer
    }
}

impl Clone for DirectionalLight {
    fn clone(&self) -> Self {
        Self::new(
            &self.context,
            self.intensity(),
            self.color(),
            &self.direction(),
        )
        .unwrap()
    }
}

fn shadow_matrix(camera: &Camera) -> Mat4 {
    let bias_matrix = crate::Mat4::new(
        0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 1.0,
    );
    bias_matrix * camera.projection() * camera.view()
}

fn compute_up_direction(direction: Vec3) -> Vec3 {
    if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9 {
        (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
    } else {
        (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
    }
}
