use crate::core::*;
use crate::renderer::*;

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
    pub fn new(context: &Context, environment_map: &TextureCubeMap) -> Self {
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
    ) -> Self {
        // Diffuse
        let irradiance_size = 32;
        let mut irradiance_map = TextureCubeMap::new_empty::<[f16; 4]>(
            context,
            irradiance_size,
            irradiance_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        {
            let viewport = Viewport::new_at_origo(irradiance_size, irradiance_size);
            for side in CubeMapSide::iter() {
                irradiance_map
                    .as_color_target(&[side], None)
                    .clear(ClearState::default())
                    .apply_screen_material(
                        &IrradianceMaterial {
                            environment_map,
                            side,
                        },
                        &Camera::new_2d(viewport),
                        &[],
                    );
            }
        }

        // Prefilter
        let prefilter_size = 128;
        let mut prefilter_map = TextureCubeMap::new_empty::<[f16; 4]>(
            context,
            prefilter_size,
            prefilter_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        {
            let max_mip_levels = 5;
            for mip in 0..max_mip_levels {
                for side in CubeMapSide::iter() {
                    let sides = [side];
                    let color_target = prefilter_map.as_color_target(&sides, Some(mip));
                    let viewport =
                        Viewport::new_at_origo(color_target.width(), color_target.height());
                    color_target
                        .clear(ClearState::default())
                        .apply_screen_material(
                            &PrefilterMaterial {
                                lighting_model,
                                environment_map,
                                side,
                                mip,
                                max_mip_levels,
                            },
                            &Camera::new_2d(viewport),
                            &[],
                        );
                }
            }
        }

        // BRDF
        let mut brdf_map = Texture2D::new_empty::<[f32; 2]>(
            context,
            512,
            512,
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
                &BrdfMaterial { lighting_model },
                &Camera::new_2d(viewport),
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
}

impl Material for PrefilterMaterial<'_> {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}{}{}",
            super::lighting_model_shader(self.lighting_model),
            include_str!("../../core/shared.frag"),
            include_str!("shaders/light_shared.frag"),
            include_str!("shaders/prefilter.frag")
        )
    }

    fn id(&self) -> u16 {
        0b1u16 << 15 | 0b1u16 << 7
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_texture_cube("environmentMap", self.environment_map);
        program.use_uniform(
            "roughness",
            self.mip as f32 / (self.max_mip_levels as f32 - 1.0),
        );
        program.use_uniform("resolution", self.environment_map.width() as f32);
        program.use_uniform("direction", self.side.direction());
        program.use_uniform("up", self.side.up());
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
}

impl Material for BrdfMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}{}{}",
            super::lighting_model_shader(self.lighting_model),
            include_str!("../../core/shared.frag"),
            include_str!("shaders/light_shared.frag"),
            include_str!("shaders/brdf.frag")
        )
    }

    fn id(&self) -> u16 {
        0b1u16 << 15 | 0b1110u16
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {}

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
}

impl Material for IrradianceMaterial<'_> {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/irradiance.frag")
        )
    }

    fn id(&self) -> u16 {
        0b1u16 << 15 | 0b1111u16
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_texture_cube("environmentMap", self.environment_map);
        program.use_uniform("direction", self.side.direction());
        program.use_uniform("up", self.side.up());
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
