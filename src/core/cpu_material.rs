//!
//! Definitions for a CPU-side material.
//!
use crate::core::*;

/// Lighting models which specify how the lighting is computed when rendering a material.
/// This is a trade-off between how fast the computations are versus how physically correct they look.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LightingModel {
    /// Phong lighting model.
    /// The fastest lighting model to calculate.
    Phong,
    /// Blinn lighting model.
    /// Almost as fast as Phong and has less artifacts.
    Blinn,
    /// Cook-Torrance lighting model with the given normal distribution and geometry functions.
    /// The most physically correct lighting model but also the most expensive.
    Cook(NormalDistributionFunction, GeometryFunction),
}

/// The geometry function used in a Cook-Torrance lighting model.
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum GeometryFunction {
    SmithSchlickGGX,
}

/// The normal distribution function used in a Cook-Torrance lighting model.
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum NormalDistributionFunction {
    Blinn,
    Beckmann,
    TrowbridgeReitzGGX,
}

///
/// A CPU-side version of a material.
/// Can be constructed manually or loaded via [io](crate::io).
///
#[derive(Debug, Clone)]
pub struct CpuMaterial {
    /// Name. Used for matching geometry and material.
    pub name: String,
    /// Albedo base color, also called diffuse color. Assumed to be in linear color space.
    pub albedo: Color,
    /// Texture with albedo base colors, also called diffuse color. Assumed to be in sRGB with or without an alpha channel.
    pub albedo_texture: Option<CpuTexture>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the occlusion, metallic and roughness parameters.
    /// The occlusion values are sampled from the red channel, metallic from the blue channel and the roughness from the green channel.
    /// Is sometimes in two textures, see [Self::occlusion_texture] and [Self::metallic_roughness_texture].
    pub occlusion_metallic_roughness_texture: Option<CpuTexture>,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    /// Can be combined with occlusion into one texture, see [Self::occlusion_metallic_roughness_texture].
    pub metallic_roughness_texture: Option<CpuTexture>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    /// Can be combined with metallic and roughness into one texture, see [Self::occlusion_metallic_roughness_texture].
    pub occlusion_texture: Option<CpuTexture>,
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<CpuTexture>,
    /// Color of light shining from an object.
    pub emissive: Color,
    /// Texture with color of light shining from an object.
    pub emissive_texture: Option<CpuTexture>,
    /// Alpha cutout value for transparency in deferred rendering pipeline.
    pub alpha_cutout: Option<f32>,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl Default for CpuMaterial {
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
            emissive: Color::BLACK,
            emissive_texture: None,
            alpha_cutout: None,
            lighting_model: LightingModel::Blinn,
        }
    }
}
