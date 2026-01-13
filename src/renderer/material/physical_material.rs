use crate::core::*;
use crate::renderer::*;

/// Quality presets for parallax occlusion mapping.
/// Controls the number of ray-march layers, secant refinement iterations, and fade distances.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HeightQuality {
    /// Minimal quality with short fade range. Use for subtle displacement
    /// or when many POM surfaces are visible simultaneously.
    VeryLow,
    /// Low quality with medium fade range. Use for background surfaces
    /// or floors where the camera rarely gets close.
    Low,
    /// Balanced quality and performance. Suitable for most surfaces that
    /// the player can approach but won't inspect closely.
    #[default]
    Medium,
    /// High quality with extended range. Use for important surfaces like
    /// walls or objects the player will examine up close.
    High,
    /// Maximum quality. Use sparingly for hero assets or surfaces where
    /// fine displacement detail is critical.
    VeryHigh,
}

impl HeightQuality {
    /// Returns (base_layers, refinement_iterations, fade_dist_start, fade_dist_end) for this quality level.
    /// fade_dist_start: distance where POM starts fading, fade_dist_end: distance where POM is fully off.
    #[inline]
    pub const fn params(self) -> (u32, u32, f32, f32) {
        match self {
            HeightQuality::VeryLow => (4, 0, 5.0, 15.0),
            HeightQuality::Low => (8, 0, 8.0, 25.0),
            HeightQuality::Medium => (8, 2, 10.0, 50.0),
            HeightQuality::High => (12, 3, 15.0, 50.0),
            HeightQuality::VeryHigh => (16, 4, 15.0, 50.0),
        }
    }
}

///
/// A physically-based material that renders a [Geometry] in an approximate correct physical manner based on Physically Based Rendering (PBR).
/// This material is affected by lights.
///
#[derive(Clone)]
pub struct PhysicalMaterial {
    /// Name.
    pub name: String,
    /// Albedo base color, also called diffuse color.
    pub albedo: Srgba,
    /// Texture with albedo base colors, also called diffuse color.
    /// The colors are assumed to be in linear sRGB (`RgbU8`), linear sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub albedo_texture: Option<Texture2DRef>,
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    pub metallic_roughness_texture: Option<Texture2DRef>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    pub occlusion_texture: Option<Texture2DRef>,
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Texture2DRef>,
    /// Render states.
    pub render_states: RenderStates,
    /// Whether this material should be treated as a transparent material (An object needs to be rendered differently depending on whether it is transparent or opaque).
    pub is_transparent: bool,
    /// Color of light shining from an object.
    pub emissive: Srgba,
    /// Texture with color of light shining from an object.
    /// The colors are assumed to be in linear sRGB (`RgbU8`), linear sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub emissive_texture: Option<Texture2DRef>,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
    /// Height map texture for parallax occlusion mapping.
    /// White = raised, 50% gray = surface level, Black = lowered.
    /// Height values are sampled from the red channel.
    pub height_texture: Option<Texture2DRef>,
    /// Height scale (depth) for parallax occlusion mapping.
    /// Typical range: 0.01 - 0.1. Higher = more depth but more artifacts at glancing angles.
    pub height_scale: f32,
    /// Quality preset for parallax occlusion mapping.
    pub height_quality: HeightQuality,
}

impl PhysicalMaterial {
    ///
    /// Constructs a new physical material from a [CpuMaterial].
    /// If the input contains an [CpuMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [PhysicalMaterial::metallic_roughness_texture] and [PhysicalMaterial::occlusion_texture] while any [CpuMaterial::metallic_roughness_texture] or [CpuMaterial::occlusion_texture] are ignored.
    /// Tries to infer whether this material is transparent or opaque from the alpha value of the albedo color and the alpha values in the albedo texture.
    /// Since this is not always correct, it is preferred to use [PhysicalMaterial::new_opaque] or [PhysicalMaterial::new_transparent].
    ///
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new_internal(context, cpu_material, super::is_transparent(cpu_material))
    }

    /// Constructs a new opaque physical material from a [CpuMaterial].
    /// If the input contains an [CpuMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [PhysicalMaterial::metallic_roughness_texture] and [PhysicalMaterial::occlusion_texture] while any [CpuMaterial::metallic_roughness_texture] or [CpuMaterial::occlusion_texture] are ignored.
    pub fn new_opaque(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new_internal(context, cpu_material, false)
    }

    /// Constructs a new transparent physical material from a [CpuMaterial].
    /// If the input contains an [CpuMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [PhysicalMaterial::metallic_roughness_texture] and [PhysicalMaterial::occlusion_texture] while any [CpuMaterial::metallic_roughness_texture] or [CpuMaterial::occlusion_texture] are ignored.
    pub fn new_transparent(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new_internal(context, cpu_material, true)
    }

    fn new_internal(context: &Context, cpu_material: &CpuMaterial, is_transparent: bool) -> Self {
        let albedo_texture =
            cpu_material
                .albedo_texture
                .as_ref()
                .map(|cpu_texture| match &cpu_texture.data {
                    TextureData::RgbU8(_) | TextureData::RgbaU8(_) => {
                        let mut cpu_texture = cpu_texture.clone();
                        cpu_texture.data.to_linear_srgb();
                        Texture2DRef::from_cpu_texture(context, &cpu_texture)
                    }
                    _ => Texture2DRef::from_cpu_texture(context, cpu_texture),
                });
        let metallic_roughness_texture =
            if let Some(ref cpu_texture) = cpu_material.occlusion_metallic_roughness_texture {
                Some(Texture2DRef::from_cpu_texture(context, cpu_texture))
            } else {
                cpu_material
                    .metallic_roughness_texture
                    .as_ref()
                    .map(|cpu_texture| Texture2DRef::from_cpu_texture(context, cpu_texture))
            };
        let occlusion_texture = if cpu_material.occlusion_metallic_roughness_texture.is_some() {
            metallic_roughness_texture.clone()
        } else {
            cpu_material
                .occlusion_texture
                .as_ref()
                .map(|cpu_texture| Texture2DRef::from_cpu_texture(context, cpu_texture))
        };
        let normal_texture = cpu_material
            .normal_texture
            .as_ref()
            .map(|cpu_texture| Texture2DRef::from_cpu_texture(context, cpu_texture));
        let emissive_texture =
            cpu_material
                .emissive_texture
                .as_ref()
                .map(|cpu_texture| match &cpu_texture.data {
                    TextureData::RgbU8(_) | TextureData::RgbaU8(_) => {
                        let mut cpu_texture = cpu_texture.clone();
                        cpu_texture.data.to_linear_srgb();
                        Texture2DRef::from_cpu_texture(context, &cpu_texture)
                    }
                    _ => Texture2DRef::from_cpu_texture(context, cpu_texture),
                });
        Self {
            name: cpu_material.name.clone(),
            albedo: cpu_material.albedo,
            albedo_texture,
            metallic: cpu_material.metallic,
            roughness: cpu_material.roughness,
            metallic_roughness_texture,
            normal_texture,
            normal_scale: cpu_material.normal_scale,
            occlusion_texture,
            occlusion_strength: cpu_material.occlusion_strength,
            render_states: if is_transparent {
                RenderStates {
                    write_mask: WriteMask::COLOR,
                    blend: Blend::TRANSPARENCY,
                    ..Default::default()
                }
            } else {
                RenderStates::default()
            },
            is_transparent,
            emissive: cpu_material.emissive,
            emissive_texture,
            lighting_model: cpu_material.lighting_model,
            height_texture: None,
            height_scale: 0.05,
            height_quality: HeightQuality::default(),
        }
    }
}

impl FromCpuMaterial for PhysicalMaterial {
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new(context, cpu_material)
    }
}

impl Material for PhysicalMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::PhysicalMaterial(
            self.albedo_texture.is_some(),
            self.metallic_roughness_texture.is_some(),
            self.occlusion_texture.is_some(),
            self.normal_texture.is_some(),
            self.emissive_texture.is_some(),
            self.height_texture.is_some(),
        )
    }

    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        let mut output = lights_shader_source(lights);
        let uses_textures = self.albedo_texture.is_some()
            || self.metallic_roughness_texture.is_some()
            || self.normal_texture.is_some()
            || self.occlusion_texture.is_some()
            || self.emissive_texture.is_some()
            || self.height_texture.is_some();

        if uses_textures {
            output.push_str("in vec2 uvs;\n");
            if self.albedo_texture.is_some() {
                output.push_str("#define USE_ALBEDO_TEXTURE\n");
            }
            if self.metallic_roughness_texture.is_some() {
                output.push_str("#define USE_METALLIC_ROUGHNESS_TEXTURE\n");
            }
            if self.occlusion_texture.is_some() {
                output.push_str("#define USE_OCCLUSION_TEXTURE\n");
            }
            // Normal texture OR height texture requires tangent/bitangent
            if self.normal_texture.is_some() || self.height_texture.is_some() {
                output.push_str("in vec3 tang;\nin vec3 bitang;\n");
            }
            if self.normal_texture.is_some() {
                output.push_str("#define USE_NORMAL_TEXTURE\n");
            }
            if self.emissive_texture.is_some() {
                output.push_str("#define USE_EMISSIVE_TEXTURE\n");
            }
            if self.height_texture.is_some() {
                output.push_str("#define USE_HEIGHT_TEXTURE\n");
            }
        }
        output.push_str(ToneMapping::fragment_shader_source());
        output.push_str(ColorMapping::fragment_shader_source());
        output.push_str(include_str!("shaders/physical_material.frag"));
        output
    }

    fn use_uniforms(&self, program: &Program, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        program.use_uniform_if_required("lightingModel", lighting_model_to_id(self.lighting_model));
        viewer.tone_mapping().use_uniforms(program);
        viewer.color_mapping().use_uniforms(program);
        program.use_uniform_if_required("cameraPosition", viewer.position());
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform_if_required("metallic", self.metallic);
        program.use_uniform_if_required("roughness", self.roughness);
        if program.requires_uniform("albedoTexture") {
            if let Some(ref texture) = self.albedo_texture {
                program.use_uniform("albedoTexTransform", texture.transformation);
                program.use_texture("albedoTexture", texture);
            }
        }
        if program.requires_uniform("metallicRoughnessTexture") {
            if let Some(ref texture) = self.metallic_roughness_texture {
                program.use_uniform("metallicRoughnessTexTransform", texture.transformation);
                program.use_texture("metallicRoughnessTexture", texture);
            }
        }
        if program.requires_uniform("occlusionTexture") {
            if let Some(ref texture) = self.occlusion_texture {
                program.use_uniform("occlusionTexTransform", texture.transformation);
                program.use_uniform("occlusionStrength", self.occlusion_strength);
                program.use_texture("occlusionTexture", texture);
            }
        }
        if program.requires_uniform("normalTexture") {
            if let Some(ref texture) = self.normal_texture {
                program.use_uniform("normalTexTransform", texture.transformation);
                program.use_uniform("normalScale", self.normal_scale);
                program.use_texture("normalTexture", texture);
            }
        }

        program.use_uniform("albedo", self.albedo.to_linear_srgb());
        program.use_uniform("emissive", self.emissive.to_linear_srgb());
        if program.requires_uniform("emissiveTexture") {
            if let Some(ref texture) = self.emissive_texture {
                program.use_uniform("emissiveTexTransform", texture.transformation);
                program.use_texture("emissiveTexture", texture);
            }
        }
        if program.requires_uniform("heightTexture") {
            if let Some(ref texture) = self.height_texture {
                let (base_layers, refinement_iterations, fade_dist_start, fade_dist_end) =
                    self.height_quality.params();
                // Precompute height scale factor on CPU:
                // 0.001 -> 0.25, 0.02 -> 1.0, 0.1 -> 3.0
                let height_layer_scale = if self.height_scale < 0.02 {
                    let t = ((self.height_scale - 0.001) / (0.02 - 0.001)).clamp(0.0, 1.0);
                    0.25 + t * 0.75
                } else {
                    let t = ((self.height_scale - 0.02) / (0.1 - 0.02)).clamp(0.0, 1.0);
                    1.0 + t * 2.0
                };
                // UV transformation matrix for height texture
                program.use_uniform("heightTexTransform", texture.transformation);
                // Depth scale for parallax displacement (typical: 0.01-0.1)
                program.use_uniform("heightScale", self.height_scale);
                // Base number of ray-march layers (before quality scaling)
                program.use_uniform("heightBaseLayers", base_layers as i32);
                // Secant refinement iterations for sub-layer precision
                program.use_uniform("heightRefinementIterations", refinement_iterations as i32);
                // Layer count multiplier based on height_scale (0.25-3.0)
                program.use_uniform("heightLayerScale", height_layer_scale);
                // Distance where POM quality starts fading
                program.use_uniform("heightFadeDistStart", fade_dist_start);
                // Distance where POM is fully disabled (falls back to flat UVs)
                program.use_uniform("heightFadeDistEnd", fade_dist_end);
                // Height map sampler (red channel: white = raised, 50% gray = surface level, black = lowered)
                program.use_texture("heightTexture", texture);
            }
        }
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn material_type(&self) -> MaterialType {
        if self.is_transparent {
            MaterialType::Transparent
        } else {
            MaterialType::Opaque
        }
    }
}

impl Default for PhysicalMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: Srgba::WHITE,
            albedo_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            render_states: RenderStates::default(),
            is_transparent: false,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            lighting_model: LightingModel::Blinn,
            height_texture: None,
            height_scale: 0.05,
            height_quality: HeightQuality::default(),
        }
    }
}

/// Parameters for automatic texture generation from heightmaps.
#[derive(Clone, Debug)]
pub struct HeightmapTextureParams {
    /// Strength multiplier for normal map generation (typical: 1.0 to 10.0).
    /// Higher values produce more pronounced surface details.
    pub normal_strength: f32,
    /// Number of rays to trace per texel for AO generation (typical: 5-16).
    /// Higher values produce smoother, more accurate AO but take longer to compute.
    pub ao_ray_count: u32,
    /// Maximum ray distance in texels for AO generation (typical: 8-32).
    /// Higher values capture larger-scale occlusion but take longer to compute.
    pub ao_max_distance: u32,
    /// Intensity multiplier for AO effect (typical: 1.0 to 2.0).
    /// Higher values produce darker shadows in occluded areas.
    pub ao_intensity: f32,
    /// Rotation offset for AO rays in radians to avoid axis-aligned artifacts.
    /// Use ~0.1 radians (≈6°) for good results. Use 0.0 for no offset.
    pub ao_angle_offset: f32,
}

impl Default for HeightmapTextureParams {
    fn default() -> Self {
        Self {
            normal_strength: 2.0,
            ao_ray_count: 12,
            ao_max_distance: 16,
            ao_intensity: 1.5,
            ao_angle_offset: 0.0,
        }
    }
}

impl PhysicalMaterial {
    /// Sets the height texture and automatically generates a normal map from it.
    ///
    /// If the material doesn't already have a normal texture, this will generate one
    /// from the heightmap using Sobel filtering.
    ///
    /// # Arguments
    /// * `context` - The rendering context
    /// * `heightmap` - The heightmap as a CpuTexture (height values read from red channel)
    /// * `height_scale` - Depth scale for parallax displacement (typical: 0.01 to 0.1)
    /// * `normal_strength` - Strength for the generated normal map (typical: 1.0 to 10.0)
    pub fn set_height_texture_with_normal(
        &mut self,
        context: &Context,
        heightmap: &CpuTexture,
        height_scale: f32,
        normal_strength: f32,
    ) {
        self.height_texture = Some(Texture2DRef::from_cpu_texture(context, heightmap));
        self.height_scale = height_scale;

        // Generate normal map if not already set
        if self.normal_texture.is_none() {
            let normal_cpu = super::heightmap_to_normal(heightmap, normal_strength);
            self.normal_texture = Some(Texture2DRef::from_cpu_texture(context, &normal_cpu));
        }
    }

    /// Sets the height texture and automatically generates both normal and AO maps from it.
    ///
    /// This is the most convenient method for setting up a heightmap with full derived textures.
    /// If the material already has a normal or occlusion texture, those will be preserved.
    ///
    /// # Arguments
    /// * `context` - The rendering context
    /// * `heightmap` - The heightmap as a CpuTexture (height values read from red channel)
    /// * `height_scale` - Depth scale for parallax displacement (typical: 0.01 to 0.1)
    /// * `params` - Parameters for texture generation (use Default::default() for reasonable values)
    pub fn set_height_texture_with_normal_and_ao(
        &mut self,
        context: &Context,
        heightmap: &CpuTexture,
        height_scale: f32,
        params: &HeightmapTextureParams,
    ) {
        self.height_texture = Some(Texture2DRef::from_cpu_texture(context, heightmap));
        self.height_scale = height_scale;

        // Generate normal map if not already set
        if self.normal_texture.is_none() {
            let normal_cpu = super::heightmap_to_normal(heightmap, params.normal_strength);
            self.normal_texture = Some(Texture2DRef::from_cpu_texture(context, &normal_cpu));
        }

        // Generate AO map if not already set
        if self.occlusion_texture.is_none() {
            let ao_cpu = super::heightmap_to_ao(
                heightmap,
                params.ao_ray_count,
                params.ao_max_distance,
                params.ao_intensity,
                params.ao_angle_offset,
            );
            self.occlusion_texture = Some(Texture2DRef::from_cpu_texture(context, &ao_cpu));
        }
    }
}
