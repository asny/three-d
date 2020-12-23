
use crate::core::*;
use std::rc::Rc;

#[derive(Clone)]
pub enum ColorSource {
    Color(Vec4),
    Texture(Rc<Texture2D>)
}

#[derive(Clone)]
pub struct PhongMaterial {
    pub name: String,
    pub color_source: ColorSource,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl PhongMaterial {
    pub fn new(gl: &Gl, cpu_material: &CPUMaterial) -> Result<Self, Error> {
        let color_source = if let Some((data, width, height)) = &cpu_material.texture_image {
            ColorSource::Texture(Rc::new(texture::Texture2D::new_with_u8(&gl, Interpolation::Linear, Interpolation::Linear,
                                                                  Some(Interpolation::Linear), Wrapping::Repeat, Wrapping::Repeat,
                                                                  *width, *height, data)?))
        }
        else {
            ColorSource::Color(cpu_material.color.map(|(r, g, b, a)| vec4(r, g, b, a)).unwrap_or(vec4(1.0, 1.0, 1.0, 1.0)))
        };
        Ok(Self {name: cpu_material.name.clone(), color_source, diffuse_intensity: cpu_material.diffuse_intensity.unwrap_or(0.5),
            specular_intensity: cpu_material.specular_intensity.unwrap_or(0.2),
            specular_power: cpu_material.specular_power.unwrap_or(6.0)})
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color_source: ColorSource::Color(vec4(1.0, 1.0, 1.0, 1.0)),
            diffuse_intensity: 0.5,
            specular_intensity: 0.2,
            specular_power: 6.0
        }
     }
}