use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;

///
/// A light which shines from the given position and in the given direction.
/// The light will cast shadows if you [generate a shadow map](SpotLight::generate_shadow_map).
///
pub struct SpotLight {
    context: Context,
    light_buffer: UniformBuffer,
    shadow_texture: DepthTargetTexture2D,
    shadow_camera: Option<Camera>,
}

impl SpotLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: &Vec3,
        position: &Vec3,
        direction: &Vec3,
        cutoff: f32,
        attenuation_constant: f32,
        attenuation_linear: f32,
        attenuation_exponential: f32,
    ) -> Result<SpotLight, Error> {
        let uniform_sizes = [3u32, 1, 1, 1, 1, 1, 3, 1, 3, 1, 16];
        let mut light = SpotLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &uniform_sizes)?,
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

    pub fn set_color(&mut self, color: &Vec3) {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32) {
        self.light_buffer.update(2, &[constant]).unwrap();
        self.light_buffer.update(3, &[linear]).unwrap();
        self.light_buffer.update(4, &[exponential]).unwrap();
    }

    pub fn set_position(&mut self, position: &Vec3) {
        self.light_buffer.update(6, &position.to_slice()).unwrap();
    }

    pub fn position(&self) -> Vec3 {
        let p = self.light_buffer.get(6).unwrap();
        vec3(p[0], p[1], p[2])
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.light_buffer.update(7, &[cutoff]).unwrap();
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
        self.light_buffer.update(9, &[0.0]).unwrap();
    }

    pub fn generate_shadow_map<F: FnOnce(Viewport, &Camera) -> Result<(), Error>>(
        &mut self,
        frustrum_depth: f32,
        texture_size: usize,
        render_scene: F,
    ) -> Result<(), Error> {
        let position = self.position();
        let direction = self.direction();
        let up = compute_up_direction(direction);
        let cutoff = self.light_buffer.get(7).unwrap()[0];

        self.shadow_camera = Some(Camera::new_perspective(
            &self.context,
            position,
            position + direction,
            up,
            degrees(cutoff),
            1.0,
            0.1,
            frustrum_depth,
        )?);
        self.light_buffer.update(
            10,
            &shadow_matrix(self.shadow_camera.as_ref().unwrap()).to_slice(),
        )?;

        self.shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            texture_size,
            texture_size,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        self.shadow_texture.write(Some(1.0), || {
            render_scene(
                Viewport::new_at_origo(texture_size, texture_size),
                self.shadow_camera.as_ref().unwrap(),
            )?;
            Ok(())
        })?;
        self.light_buffer.update(9, &[1.0])?;
        Ok(())
    }

    pub fn shadow_map(&self) -> &dyn Texture {
        &self.shadow_texture
    }

    pub fn buffer(&self) -> &UniformBuffer {
        &self.light_buffer
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
