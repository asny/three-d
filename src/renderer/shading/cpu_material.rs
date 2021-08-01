use crate::core::*;

///
/// A CPU-side version of a [material](crate::Material).
/// Can be constructed manually or loaded via [io](crate::io).
/// Textures are assumed to be in sRGB with or without an alpha channel.
///
pub struct CPUMaterial {
    pub name: String,
    pub albedo: Color,
    pub albedo_texture: Option<CPUTexture<u8>>,
    pub metallic: f32,
    pub roughness: f32,
    pub metallic_roughness_texture: Option<CPUTexture<u8>>,
}

impl Default for CPUMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: Color::WHITE,
            albedo_texture: None,
            metallic_roughness_texture: None,
            metallic: 0.0,
            roughness: 1.0,
        }
    }
}
