use crate::core::*;

///
/// Precalculations of light shining from an environment map (known as image based lighting - IBL).
/// This allows for real-time rendering of ambient light from the environment (see [AmbientLight](crate::AmbientLight)).
///
pub struct Environment {
    /// A cube map used to calculate the diffuse contribution from the environment.
    pub irradiance_map: TextureCubeMap,
    /// A cube map used to calculate the specular contribution from the environment.
    /// Each mip-map level contain the prefiltered color for a certain surface roughness.
    pub prefilter_map: TextureCubeMap,
    /// A 2D texture that contain the BRDF lookup tables (LUT).
    pub brdf_map: Texture2D,
}

impl Environment {
    ///
    /// Computes the maps needed for physically based rendering with lighting from an environment from the given environment map.
    /// A default Cook-Torrance lighting model is used.
    ///
    pub fn new(context: &Context, environment_map: &TextureCubeMap) -> ThreeDResult<Self> {
        Self::new_with_lighting_model(
            context,
            environment_map,
            LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
        )
    }

    ///
    /// Computes the maps needed for physically based rendering with lighting from an environment from the given environment map and with the specified lighting model.
    ///
    pub fn new_with_lighting_model(
        context: &Context,
        environment_map: &TextureCubeMap,
        lighting_model: LightingModel,
    ) -> ThreeDResult<Self> {
        // Diffuse
        let irradiance_size = 32;
        let mut irradiance_map = TextureCubeMap::new_empty::<Vector4<f16>>(
            context,
            irradiance_size,
            irradiance_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        )?;
        {
            let fragment_shader_source = format!(
                "{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/irradiance.frag")
            );
            let effect = ImageCubeEffect::new(context, &fragment_shader_source)?;
            for side in CubeMapSide::iter() {
                effect.use_texture_cube("environmentMap", environment_map)?;
                let viewport = Viewport::new_at_origo(irradiance_size, irradiance_size);
                irradiance_map
                    .as_render_target(side, None)?
                    .clear(Color::BLACK, 1.0)?
                    .write(|| effect.render(side, RenderStates::default(), viewport))?;
            }
        }

        // Prefilter
        let prefilter_size = 128;
        let mut prefilter_map = TextureCubeMap::new_empty::<Vector4<f16>>(
            context,
            prefilter_size,
            prefilter_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        )?;
        {
            let fragment_shader_source = format!(
                "{}{}{}{}",
                lighting_model.shader(),
                include_str!("../../core/shared.frag"),
                include_str!("shaders/light_shared.frag"),
                include_str!("shaders/prefilter.frag")
            );
            let program = ImageCubeEffect::new(context, &fragment_shader_source)?;
            let max_mip_levels = 5;
            for mip in 0..max_mip_levels {
                let roughness = mip as f32 / (max_mip_levels as f32 - 1.0);
                let viewport = Viewport::new_at_origo(
                    prefilter_size / 2u32.pow(mip),
                    prefilter_size / 2u32.pow(mip),
                );
                for side in CubeMapSide::iter() {
                    program.use_texture_cube("environmentMap", environment_map)?;
                    program.use_uniform("roughness", &roughness)?;
                    program.use_uniform("resolution", &(environment_map.width() as f32))?;
                    prefilter_map
                        .as_render_target(side, Some(mip))?
                        .clear(Color::BLACK, 1.0)?
                        .write(|| program.render(side, RenderStates::default(), viewport))?;
                }
            }
        }

        // BRDF
        let mut brdf_map = Texture2D::new_empty::<Vector2<f32>>(
            context,
            512,
            512,
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
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
        let viewport = Viewport::new_at_origo(brdf_map.width(), brdf_map.height());
        brdf_map
            .as_render_target(None)?
            .clear(Color::BLACK, 1.0)?
            .write(|| effect.apply(RenderStates::default(), viewport))?;

        Ok(Self {
            irradiance_map,
            prefilter_map,
            brdf_map,
        })
    }
}
