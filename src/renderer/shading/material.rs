use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A material used for shading an object using physically based rendering (PBR).
///
#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub albedo: Vec4,
    pub albedo_texture: Option<Rc<Texture2D>>,
    pub metallic: f32,
    pub roughness: f32,
    pub metallic_roughness_texture: Option<Rc<Texture2D>>,
}

impl Material {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> Result<Self, Error> {
        let albedo_texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        let metallic_roughness_texture =
            if let Some(ref cpu_texture) = cpu_material.metallic_roughness_texture {
                Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
            } else {
                None
            };
        Ok(Self {
            name: cpu_material.name.clone(),
            albedo: cpu_material.albedo.to_vec4(),
            albedo_texture,
            metallic: cpu_material.metallic,
            roughness: cpu_material.roughness,
            metallic_roughness_texture,
        })
    }

    pub(crate) fn bind(&self, program: &Program) -> Result<(), Error> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;
        program.use_uniform_vec4("albedo", &self.albedo)?;
        if let Some(ref texture) = self.albedo_texture {
            program.use_texture("albedoTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture.as_ref())?;
        }
        Ok(())
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: vec4(1.0, 1.0, 1.0, 1.0),
            albedo_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
        }
    }
}
