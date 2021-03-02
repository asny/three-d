
use crate::math::*;
use crate::core::*;

///
/// A light which shines from the given position in all directions.
///
pub struct PointLight {
    light_buffer: UniformBuffer
}

impl PointLight {

    pub fn new(context: &Context, intensity: f32, color: &Vec3, position: &Vec3,
               attenuation_constant: f32, attenuation_linear: f32, attenuation_exponential: f32) -> Result<PointLight, Error>
    {
        let mut light = PointLight { light_buffer: UniformBuffer::new(context, &[3u32, 1, 1, 1, 1, 1, 3, 1])? };

        light.set_intensity(intensity);
        light.set_color(color);
        light.set_position(position);
        light.set_attenuation(attenuation_constant, attenuation_linear, attenuation_exponential);
        Ok(light)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32)
    {
        self.light_buffer.update(2, &[constant]).unwrap();
        self.light_buffer.update(3, &[linear]).unwrap();
        self.light_buffer.update(4, &[exponential]).unwrap();
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        self.light_buffer.update(6, &position.to_slice()).unwrap();
    }

    pub fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }
}