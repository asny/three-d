use crate::core::*;

///
/// A light which shines from the given position in all directions.
///
pub struct PointLight {
    context: Context,
    light_buffer: UniformBuffer,
}

impl PointLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        position: &Vec3,
        attenuation_constant: f32,
        attenuation_linear: f32,
        attenuation_exponential: f32,
    ) -> Result<PointLight> {
        let mut light = PointLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &[3u32, 1, 1, 1, 1, 1, 3, 1])?,
        };

        light.set_intensity(intensity);
        light.set_color(color);
        light.set_position(position);
        light.set_attenuation(
            attenuation_constant,
            attenuation_linear,
            attenuation_exponential,
        );
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

    pub fn buffer(&self) -> &UniformBuffer {
        &self.light_buffer
    }
}

impl Clone for PointLight {
    fn clone(&self) -> Self {
        let (attenuation_constant, attenuation_linear, attenuation_exponential) =
            self.attenuation();
        Self::new(
            &self.context,
            self.intensity(),
            self.color(),
            &self.position(),
            attenuation_constant,
            attenuation_linear,
            attenuation_exponential,
        )
        .unwrap()
    }
}
