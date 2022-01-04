use crate::core::*;
use crate::renderer::*;
pub struct Environment {
    pub irradiance_map: ColorTargetTextureCubeMap<f32>,
    pub prefilter_map: ColorTargetTextureCubeMap<f32>,
    pub brdf_map: ColorTargetTexture2D<f32>,
}

impl Environment {
    pub fn new(context: &Context, environment_map: &impl TextureCube) -> ThreeDResult<Self> {
        let lighting_model = LightingModel::Cook(
            NormalDistributionFunction::TrowbridgeReitzGGX,
            GeometryFunction::SmithSchlickGGX,
        );
        // Diffuse
        let irradiance_map = ColorTargetTextureCubeMap::new(
            context,
            32,
            32,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;
        irradiance_map.write_to_all(
            ClearState::default(),
            &format!(
                "{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/irradiance.frag")
            ),
            |program| program.use_texture_cube("environmentMap", environment_map),
        )?;

        // Prefilter
        let prefilter_map = ColorTargetTextureCubeMap::new(
            context,
            128,
            128,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;
        let max_mip_levels = 5;
        for mip in 0..max_mip_levels {
            let roughness = mip as f32 / (max_mip_levels as f32 - 1.0);
            prefilter_map.write_to_all_to_mip_level(
                mip,
                ClearState::default(),
                &format!(
                    "{}{}{}{}",
                    lighting_model.shader(),
                    include_str!("../../core/shared.frag"),
                    include_str!("shaders/light_shared.frag"),
                    include_str!("shaders/prefilter.frag")
                ),
                |program| {
                    program.use_texture_cube("environmentMap", environment_map)?;
                    program.use_uniform_float("roughness", &roughness)?;
                    program.use_uniform_float("resolution", &(environment_map.width() as f32))
                },
            )?;
        }

        // BRDF
        let brdf_map = ColorTargetTexture2D::new(
            context,
            512,
            512,
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RG,
        )?;
        let effect = ImageEffect::new(
            context,
            &format!(
                "{}{}{}{}",
                lighting_model.shader(),
                include_str!("../../core/shared.frag"),
                include_str!("shaders/light_shared.frag"),
                include_str!("shaders/brdf.frag")
            ),
        )?;
        brdf_map.write(ClearState::default(), || {
            effect.apply(
                RenderStates::default(),
                Viewport::new_at_origo(brdf_map.width(), brdf_map.height()),
            )
        })?;

        Ok(Self {
            irradiance_map,
            prefilter_map,
            brdf_map,
        })
    }
}
