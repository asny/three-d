use crate::core::*;
use crate::renderer::*;
pub struct Environment {
    pub irradiance_map: TextureCubeMap<f16>,
    pub prefilter_map: TextureCubeMap<f16>,
    pub brdf_map: Texture2D<f16>,
}

impl Environment {
    pub fn new(context: &Context, environment_map: &impl TextureCube) -> ThreeDResult<Self> {
        let lighting_model = LightingModel::Cook(
            NormalDistributionFunction::TrowbridgeReitzGGX,
            GeometryFunction::SmithSchlickGGX,
        );
        // Diffuse
        let mut irradiance_map = TextureCubeMap::new_empty(
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
        {
            let fragment_shader_source = format!(
                "{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/irradiance.frag")
            );
            let effect = ImageCubeEffect::new(context, &fragment_shader_source)?;
            let render_target = RenderTargetCubeMap::new_color(context, &mut irradiance_map)?;
            for side in CubeMapSide::iter() {
                effect.use_texture_cube("environmentMap", environment_map)?;
                let viewport =
                    Viewport::new_at_origo(render_target.width(), render_target.height());
                render_target.write(side, ClearState::default(), || {
                    effect.render(side, RenderStates::default(), viewport)
                })?;
            }
        }

        // Prefilter
        let mut prefilter_map = TextureCubeMap::new_empty(
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
        {
            let fragment_shader_source = format!(
                "{}{}{}{}",
                lighting_model.shader(),
                include_str!("../../core/shared.frag"),
                include_str!("shaders/light_shared.frag"),
                include_str!("shaders/prefilter.frag")
            );
            let program = ImageCubeEffect::new(context, &fragment_shader_source)?;
            let render_target = RenderTargetCubeMap::new_color(context, &mut prefilter_map)?;

            let max_mip_levels = 5;
            for mip in 0..max_mip_levels {
                let roughness = mip as f32 / (max_mip_levels as f32 - 1.0);
                let viewport = Viewport::new_at_origo(
                    render_target.width() / 2u32.pow(mip),
                    render_target.height() / 2u32.pow(mip),
                );
                for side in CubeMapSide::iter() {
                    program.use_texture_cube("environmentMap", environment_map)?;
                    program.use_uniform_float("roughness", &roughness)?;
                    program.use_uniform_float("resolution", &(environment_map.width() as f32))?;
                    render_target.write_to_mip_level(side, mip, ClearState::default(), || {
                        program.render(side, RenderStates::default(), viewport)
                    })?;
                }
            }
        }

        // BRDF
        let mut brdf_map = Texture2D::new_empty(
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
        let viewport = Viewport::new_at_origo(brdf_map.width(), brdf_map.height());
        brdf_map.write(ClearState::default(), || {
            effect.apply(RenderStates::default(), viewport)
        })?;

        Ok(Self {
            irradiance_map,
            prefilter_map,
            brdf_map,
        })
    }
}
