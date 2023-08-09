use crate::core::*;
use crate::renderer::*;

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
        }
    }
}

impl FromCpuMaterial for PhysicalMaterial {
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new(context, cpu_material)
    }
}

impl Material for PhysicalMaterial {
    fn id(&self) -> u16 {
        let mut id = 0b1u16 << 15 | 0b1u16 << 5;
        if self.albedo_texture.is_some() {
            id |= 0b1u16;
        }
        if self.metallic_roughness_texture.is_some() {
            id |= 0b1u16 << 1;
        }
        if self.occlusion_texture.is_some() {
            id |= 0b1u16 << 2;
        }
        if self.normal_texture.is_some() {
            id |= 0b1u16 << 3;
        }
        if self.emissive_texture.is_some() {
            id |= 0b1u16 << 4;
        }
        id
    }

    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        let mut output = lights_shader_source(lights, self.lighting_model);
        if self.albedo_texture.is_some()
            || self.metallic_roughness_texture.is_some()
            || self.normal_texture.is_some()
            || self.occlusion_texture.is_some()
            || self.emissive_texture.is_some()
        {
            output.push_str("in vec2 uvs;\n");
            if self.albedo_texture.is_some() {
                output.push_str("#define USE_ALBEDO_TEXTURE;\n");
            }
            if self.metallic_roughness_texture.is_some() {
                output.push_str("#define USE_METALLIC_ROUGHNESS_TEXTURE;\n");
            }
            if self.occlusion_texture.is_some() {
                output.push_str("#define USE_OCCLUSION_TEXTURE;\n");
            }
            if self.normal_texture.is_some() {
                output.push_str("#define USE_NORMAL_TEXTURE;\nin vec3 tang;\nin vec3 bitang;\n");
            }
            if self.emissive_texture.is_some() {
                output.push_str("#define USE_EMISSIVE_TEXTURE;\n");
            }
        }
        output.push_str(ToneMapping::fragment_shader_source());
        output.push_str(ColorMapping::fragment_shader_source());
        output.push_str(include_str!("shaders/physical_material.frag"));
        output
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            normal: true,
            color: true,
            uv: self.albedo_texture.is_some()
                || self.metallic_roughness_texture.is_some()
                || self.normal_texture.is_some()
                || self.occlusion_texture.is_some()
                || self.emissive_texture.is_some(),
            tangents: self.normal_texture.is_some(),
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        camera.tone_mapping.use_uniforms(program);
        camera.color_mapping.use_uniforms(program);
        if !lights.is_empty() {
            program.use_uniform_if_required("cameraPosition", camera.position());
            for (i, light) in lights.iter().enumerate() {
                light.use_uniforms(program, i as u32);
            }
            program.use_uniform("metallic", self.metallic);
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
        }
        program.use_uniform("albedo", self.albedo.to_linear_srgb());
        program.use_uniform("emissive", self.emissive.to_linear_srgb());
        if program.requires_uniform("emissiveTexture") {
            if let Some(ref texture) = self.emissive_texture {
                program.use_uniform("emissiveTexTransform", texture.transformation);
                program.use_texture("emissiveTexture", texture);
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
        }
    }
}
