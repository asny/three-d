//!
//! Definitions for a CPU- and GPU-side material.
//!
use crate::core::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LightingModel {
    Phong,
    Blinn,
    Cook(NormalDistributionFunction, GeometryFunction),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GeometryFunction {
    SmithSchlickGGX,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NormalDistributionFunction {
    Blinn,
    Beckmann,
    TrowbridgeReitzGGX,
}

///
/// A CPU-side version of a [material](crate::Material).
/// Can be constructed manually or loaded via [io](crate::io).
///
pub struct CPUMaterial {
    /// Name. Used for matching geometry and material.
    pub name: String,
    /// Albedo base color, also called diffuse color. Assumed to be in linear color space.
    pub albedo: Color,
    /// Texture with albedo base colors, also called diffuse color. Assumed to be in sRGB with or without an alpha channel.
    pub albedo_texture: Option<CPUTexture<u8>>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the occlusion, metallic and roughness parameters.
    /// The occlusion values are sampled from the red channel, metallic from the blue channel and the roughness from the green channel.
    /// Is sometimes in two textures, see [Self::occlusion_texture] and [Self::metallic_roughness_texture].
    pub occlusion_metallic_roughness_texture: Option<CPUTexture<u8>>,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    /// Can be combined with occlusion into one texture, see [Self::occlusion_metallic_roughness_texture].
    pub metallic_roughness_texture: Option<CPUTexture<u8>>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    /// Can be combined with metallic and roughness into one texture, see [Self::occlusion_metallic_roughness_texture].
    pub occlusion_texture: Option<CPUTexture<u8>>,
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<CPUTexture<u8>>,

    pub lighting_model: LightingModel,
}

impl Default for CPUMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: Color::WHITE,
            albedo_texture: None,
            occlusion_metallic_roughness_texture: None,
            metallic_roughness_texture: None,
            occlusion_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            occlusion_strength: 1.0,
            normal_texture: None,
            normal_scale: 1.0,
            lighting_model: LightingModel::Blinn,
        }
    }
}
