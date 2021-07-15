use crate::core::*;
use crate::definition::*;
use crate::math::*;
use std::rc::Rc;

///
/// The source of color on an object, either a fixed value or a texture.
///
#[derive(Clone)]
pub enum ColorSource {
    Color(Vec4),
    Texture(Rc<Texture2D>),
}

impl std::fmt::Display for ColorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSource::Color(_) => {
                write!(f, "Color")
            }
            ColorSource::Texture(_) => {
                write!(f, "Texture")
            }
        }
    }
}

///
/// A material used for shading an object using physically based rendering (PBR).
///
#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub color_source: ColorSource,
    pub metallic: f32,
    pub roughness: f32,
}

impl Material {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> Result<Self, Error> {
        let color_source = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            ColorSource::Texture(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            ColorSource::Color(cpu_material.albedo.to_vec4())
        };
        Ok(Self {
            name: cpu_material.name.clone(),
            color_source,
            metallic: cpu_material.metallic,
            roughness: cpu_material.roughness,
        })
    }

    pub(crate) fn bind(&self, program: &Program) -> Result<(), Error> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;

        match self.color_source {
            ColorSource::Color(ref color) => {
                program.use_uniform_vec4("surfaceColor", color)?;
            }
            ColorSource::Texture(ref texture) => {
                program.use_texture("tex", texture.as_ref())?;
            }
        }
        Ok(())
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color_source: ColorSource::Color(vec4(1.0, 1.0, 1.0, 1.0)),
            metallic: 0.0,
            roughness: 0.0,
        }
    }
}
