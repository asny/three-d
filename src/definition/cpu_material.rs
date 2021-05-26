use crate::definition::*;

///
/// A CPU-side version of a [material](crate::Material).
/// Can be constructed manually or loaded via [io](crate::io).
///
pub struct CPUMaterial {
    pub name: String,
    pub color: Option<(f32, f32, f32, f32)>,
    pub color_texture: Option<CPUTexture<u8>>,
    pub metallic_factor: Option<f32>,
    pub roughness_factor: Option<f32>,
    pub metallic_roughness_texture: Option<CPUTexture<u8>>,
}

impl Default for CPUMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color: None,
            color_texture: None,
            metallic_roughness_texture: None,
            metallic_factor: None,
            roughness_factor: None,
        }
    }
}
