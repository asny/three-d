use crate::core::*;
use crate::renderer::*;

///
/// Similar to [PhysicalMaterial] except that rendering happens in two stages which produces the same result, but is more efficient for complex scenes.
/// This material does not support transparency but does support [alpha cutout](DeferredPhysicalMaterial::alpha_cutout).
///
/// The first stage renders geometry information to a [RenderTarget] and the second stage uses this render target to apply lighting based on the geometry information which means the expensive lighting calculations are only done once per pixel.
/// The [RenderTarget::render], [ColorTarget::render] or [DepthTarget::render] methods all support the two stages required by this material, so just pass the [Object] with this material applied into one of these methods.
/// However, it is not possible to use the [Object::render] method to render a [Geometry] with this material directly to the screen.
/// Instead render the object into a [RenderTarget] consisting of a [Texture2DArray] with three RGBA u8 layers as color target and a [DepthTexture2D] as depth target.
/// Then call the [DeferredPhysicalMaterial::lighting_pass] method with these textures to render to the screen.
///
#[derive(Clone)]
pub struct DeferredPhysicalMaterial {
    /// Name.
    pub name: String,
    /// Albedo base color, also called diffuse color.
    pub albedo: Srgba,
    /// Texture with albedo base colors, also called diffuse color.
    /// The colors are assumed to be in linear sRGB (`RgbU8`), linear sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub albedo_texture: Option<Texture2DRef>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
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
    /// Render states
    pub render_states: RenderStates,
    /// Color of light shining from an object.
    pub emissive: Srgba,
    /// Texture with color of light shining from an object.
    /// The colors are assumed to be in linear sRGB (`RgbU8`), linear sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub emissive_texture: Option<Texture2DRef>,
    /// A threshold on the alpha value of the color as a workaround for transparency.
    /// If the alpha value of a pixel touched by an object with this material is less than the threshold, then that object is not contributing to the color of that pixel.
    /// On the other hand, if the alpha value is more than the threshold, then it is contributing fully to that pixel and thereby blocks out everything behind.
    pub alpha_cutout: Option<f32>,
}

impl DeferredPhysicalMaterial {
    ///
    /// Constructs a new deferred physical material from a [CpuMaterial].
    /// If the input contains an [CpuMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [DeferredPhysicalMaterial::metallic_roughness_texture] and [DeferredPhysicalMaterial::occlusion_texture] while any [CpuMaterial::metallic_roughness_texture] or [CpuMaterial::occlusion_texture] are ignored.
    ///
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> Self {
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
            render_states: RenderStates::default(),
            alpha_cutout: cpu_material.alpha_cutout,
            emissive: cpu_material.emissive,
            emissive_texture,
        }
    }

    ///
    /// Constructs a deferred physical material from a physical material.
    ///
    pub fn from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            name: physical_material.name.clone(),
            albedo: physical_material.albedo,
            albedo_texture: physical_material.albedo_texture.clone(),
            metallic: physical_material.metallic,
            roughness: physical_material.roughness,
            metallic_roughness_texture: physical_material.metallic_roughness_texture.clone(),
            normal_texture: physical_material.normal_texture.clone(),
            normal_scale: physical_material.normal_scale,
            occlusion_texture: physical_material.occlusion_texture.clone(),
            occlusion_strength: physical_material.occlusion_strength,
            render_states: RenderStates {
                write_mask: WriteMask::default(),
                blend: Blend::Disabled,
                ..physical_material.render_states
            },
            emissive: physical_material.emissive,
            emissive_texture: physical_material.emissive_texture.clone(),
            alpha_cutout: if physical_material.is_transparent {
                Some(0.5)
            } else {
                None
            },
        }
    }
    ///
    /// The second stage of a deferred render call.
    /// Use the [Object::render] method to render the objects with this material into a [RenderTarget] and then call this method with these textures to render to the screen.
    /// See [DeferredPhysicalMaterial] for more information.
    ///
    pub fn lighting_pass(
        context: &Context,
        camera: &Camera,
        geometry_pass_color_texture: ColorTexture,
        geometry_pass_depth_texture: DepthTexture,
        lights: &[&dyn Light],
    ) {
        apply_screen_effect(
            context,
            lighting_pass::LightingPassEffect {},
            camera,
            lights,
            Some(geometry_pass_color_texture),
            Some(geometry_pass_depth_texture),
        );
    }
}

impl FromCpuMaterial for DeferredPhysicalMaterial {
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new(context, cpu_material)
    }
}

impl Material for DeferredPhysicalMaterial {
    fn id(&self) -> u16 {
        let mut id = 0b1u16 << 15 | 0b1u16 << 6;
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
        if self.alpha_cutout.is_some() {
            id |= 0b1u16 << 5;
        }
        id
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        let mut output = include_str!("../../core/shared.frag").to_string();
        if self.albedo_texture.is_some()
            || self.metallic_roughness_texture.is_some()
            || self.normal_texture.is_some()
            || self.occlusion_texture.is_some()
            || self.emissive_texture.is_some()
            || self.alpha_cutout.is_some()
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
            if self.alpha_cutout.is_some() {
                output.push_str(
                    format!(
                        "#define ALPHACUT;\nfloat acut = {};",
                        self.alpha_cutout.unwrap()
                    )
                    .as_str(),
                );
            }
        }
        output.push_str(include_str!("shaders/deferred_physical_material.frag"));
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
                || self.emissive_texture.is_some()
                || self.alpha_cutout.is_some(),
            tangents: self.normal_texture.is_some(),
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("metallic", self.metallic);
        program.use_uniform("roughness", self.roughness);
        program.use_uniform("albedo", self.albedo.to_linear_srgb());
        program.use_uniform("emissive", self.emissive.to_linear_srgb());
        if let Some(ref texture) = self.albedo_texture {
            program.use_texture("albedoTexture", texture);
            program.use_uniform("albedoTexTransform", texture.transformation);
        }
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture);
            program.use_uniform("metallicRoughnessTexTransform", texture.transformation);
        }
        if let Some(ref texture) = self.occlusion_texture {
            program.use_uniform("occlusionStrength", self.occlusion_strength);
            program.use_uniform("occlusionTexTransform", texture.transformation);
            program.use_texture("occlusionTexture", texture);
        }
        if let Some(ref texture) = self.normal_texture {
            program.use_uniform("normalScale", self.normal_scale);
            program.use_uniform("normalTexTransform", texture.transformation);
            program.use_texture("normalTexture", texture);
        }
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
        MaterialType::Deferred
    }
}

impl Default for DeferredPhysicalMaterial {
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
            alpha_cutout: None,
            emissive: Srgba::BLACK,
            emissive_texture: None,
        }
    }
}
