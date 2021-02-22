
pub struct CPUMaterial {
    pub name: String,
    pub color: Option<(f32, f32, f32, f32)>,
    pub texture_image: Option<crate::core::cpu_texture::CPUTexture>,
    pub diffuse_intensity: Option<f32>,
    pub specular_intensity: Option<f32>,
    pub specular_power: Option<f32>
}

impl Default for CPUMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color: Some((1.0, 1.0, 1.0, 1.0)),
            texture_image: None,
            diffuse_intensity: Some(0.5),
            specular_intensity: Some(0.2),
            specular_power: Some(6.0)
        }
     }
}