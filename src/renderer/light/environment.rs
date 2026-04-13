use crate::core::*;
use crate::renderer::*;

///
/// Options used when generating an [Environment], ie. when precalculating lighting from an environment map.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EnvironmentOptions {
    /// The lighting model used for the precalculation.
    pub lighting_model: LightingModel,
    /// The texture size of the irradiance map.
    pub irradiance_map_size: u32,
    /// The number of samples used when generating the irradiance map.
    pub irradiance_sample_count: u32,
    /// The texture size of the prefilter map.
    pub prefilter_map_size: u32,
    /// The number of mip levels used when generating the prefilter map.
    pub prefilter_map_max_mip_levels: u32,
    /// The number of samples used when generating the prefilter map.
    pub prefilter_sample_count: u32,
    /// The texture size of the BRDF map.
    pub brdf_map_size: u32,
    /// The number of samples used when generating the BRDF map.
    pub brdf_sample_count: u32,
}

impl Default for EnvironmentOptions {
    fn default() -> Self {
        Self {
            lighting_model: LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
            irradiance_map_size: 32,
            irradiance_sample_count: 256,
            prefilter_map_size: 128,
            prefilter_map_max_mip_levels: 5,
            prefilter_sample_count: 256,
            brdf_map_size: 512,
            brdf_sample_count: 128,
        }
    }
}

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
    /// Computes the maps needed for image based lighting with lighting coming
    /// from the given environment map and using the default Cook-Torrance lighting model.
    ///
    pub fn new(context: &Context, environment_map: &TextureCubeMap) -> Self {
        Self::new_with_options(context, environment_map, EnvironmentOptions::default())
    }

    ///
    /// Computes the maps needed for image based lighting with lighting coming
    /// from the given environment map and using the specified lighting model.
    ///
    pub fn new_with_lighting_model(
        context: &Context,
        environment_map: &TextureCubeMap,
        lighting_model: LightingModel,
    ) -> Self {
        Self::new_with_options(
            context,
            environment_map,
            EnvironmentOptions {
                lighting_model,
                ..Default::default()
            },
        )
    }

    ///
    /// Computes the maps needed for image based lighting with lighting coming
    /// from the given environment map and using the specified [EnvironmentOptions].
    ///
    pub fn new_with_options(
        context: &Context,
        environment_map: &TextureCubeMap,
        options: EnvironmentOptions,
    ) -> Self {
        // Diffuse
        let irradiance_map = TextureCubeMap::new_empty::<[f16; 4]>(
            context,
            options.irradiance_map_size,
            options.irradiance_map_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Mipmap::default()),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        {
            let viewport = Viewport::new_at_origo(irradiance_map.width(), irradiance_map.height());
            for side in CubeMapSide::iter() {
                irradiance_map
                    .as_color_target(&[side], None)
                    .clear(ClearState::default())
                    .apply_screen_material(
                        &IrradianceMaterial {
                            environment_map,
                            side,
                            sample_count: options.irradiance_sample_count,
                        },
                        Camera::new_2d(viewport),
                        &[],
                    );
            }
        }

        // Prefilter
        let prefilter_map = TextureCubeMap::new_empty::<[f16; 4]>(
            context,
            options.prefilter_map_size,
            options.prefilter_map_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Mipmap::default()),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        {
            for mip in 0..options.prefilter_map_max_mip_levels {
                for side in CubeMapSide::iter() {
                    let sides = [side];
                    let color_target = prefilter_map.as_color_target(&sides, Some(mip));
                    let viewport =
                        Viewport::new_at_origo(color_target.width(), color_target.height());
                    color_target
                        .clear(ClearState::default())
                        .apply_screen_material(
                            &PrefilterMaterial {
                                lighting_model: options.lighting_model,
                                environment_map,
                                side,
                                mip,
                                max_mip_levels: options.prefilter_map_max_mip_levels,
                                sample_count: options.prefilter_sample_count,
                            },
                            Camera::new_2d(viewport),
                            &[],
                        );
                }
            }
        }

        // BRDF
        let brdf_map = Texture2D::new_empty::<[f32; 2]>(
            context,
            options.brdf_map_size,
            options.brdf_map_size,
            Interpolation::Linear,
            Interpolation::Linear,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let viewport = Viewport::new_at_origo(brdf_map.width(), brdf_map.height());
        brdf_map
            .as_color_target(None)
            .clear(ClearState::default())
            .apply_screen_material(
                &BrdfMaterial {
                    lighting_model: options.lighting_model,
                    sample_count: options.brdf_sample_count,
                },
                Camera::new_2d(viewport),
                &[],
            );

        Self {
            irradiance_map,
            prefilter_map,
            brdf_map,
        }
    }
}

struct PrefilterMaterial<'a> {
    lighting_model: LightingModel,
    environment_map: &'a TextureCubeMap,
    side: CubeMapSide,
    mip: u32,
    max_mip_levels: u32,
    sample_count: u32,
}

impl Material for PrefilterMaterial<'_> {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/light_shared.frag"),
            include_str!("shaders/prefilter.frag")
        )
    }

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::PrefilterMaterial
    }

    fn use_uniforms(&self, program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_uniform_if_required("lightingModel", lighting_model_to_id(self.lighting_model));
        program.use_texture_cube("environmentMap", self.environment_map);
        program.use_uniform(
            "roughness",
            self.mip as f32 / (self.max_mip_levels as f32 - 1.0),
        );
        program.use_uniform("resolution", self.environment_map.width() as f32);
        program.use_uniform("direction", self.side.direction());
        program.use_uniform("up", self.side.up());
        program.use_uniform("sampleCount", self.sample_count);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

struct BrdfMaterial {
    lighting_model: LightingModel,
    sample_count: u32,
}

impl Material for BrdfMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/light_shared.frag"),
            include_str!("shaders/brdf.frag")
        )
    }

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::BrdfMaterial
    }

    fn use_uniforms(&self, program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_uniform_if_required("lightingModel", lighting_model_to_id(self.lighting_model));
        program.use_uniform("sampleCount", self.sample_count);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

struct IrradianceMaterial<'a> {
    environment_map: &'a TextureCubeMap,
    side: CubeMapSide,
    sample_count: u32,
}

impl Material for IrradianceMaterial<'_> {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/irradiance.frag")
        )
    }

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::IrradianceMaterial
    }

    fn use_uniforms(&self, program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_texture_cube("environmentMap", self.environment_map);
        program.use_uniform("direction", self.side.direction());
        program.use_uniform("up", self.side.up());
        program.use_uniform("sampleCount", self.sample_count);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
