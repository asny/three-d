use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A physically-based material that renders a [Shadable] object in an approximate correct physical manner based on Physically Based Rendering (PBR).
/// This material is affected by lights.
///
#[derive(Clone)]
pub struct PhysicalMaterial {
    /// Name. Used for matching geometry and material.
    pub name: String,
    /// Albedo base color, also called diffuse color. Assumed to be in linear color space.
    pub albedo: Color,
    /// Texture with albedo base colors, also called diffuse color. Assumed to be in sRGB with or without an alpha channel.
    pub albedo_texture: Option<Rc<Texture2D>>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    pub metallic_roughness_texture: Option<Rc<Texture2D>>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    pub occlusion_texture: Option<Rc<Texture2D>>,
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Rc<Texture2D>>,
    /// Render states used when the color is opaque (has a maximal alpha value).
    pub opaque_render_states: RenderStates,
    /// Render states used when the color is transparent (does not have a maximal alpha value).
    pub transparent_render_states: RenderStates,

    pub emissive: Color,

    pub emissive_texture: Option<Rc<Texture2D>>,
    /// Alpha cutoff value for transparency in deferred rendering pipeline.
    pub alpha_cutout: Option<f32>,
}

impl PhysicalMaterial {
    ///
    /// Constructs a new physical material from a [CPUMaterial].
    /// If the input contains an [CPUMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [PhysicalMaterial::metallic_roughness_texture] and [PhysicalMaterial::occlusion_texture] while any [CPUMaterial::metallic_roughness_texture] or [CPUMaterial::occlusion_texture] are ignored.
    ///
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> ThreeDResult<Self> {
        let albedo_texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        let metallic_roughness_texture =
            if let Some(ref cpu_texture) = cpu_material.occlusion_metallic_roughness_texture {
                Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
            } else {
                if let Some(ref cpu_texture) = cpu_material.metallic_roughness_texture {
                    Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
                } else {
                    None
                }
            };
        let occlusion_texture = if cpu_material.occlusion_metallic_roughness_texture.is_some() {
            metallic_roughness_texture.clone()
        } else {
            if let Some(ref cpu_texture) = cpu_material.occlusion_texture {
                Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
            } else {
                None
            }
        };
        let normal_texture = if let Some(ref cpu_texture) = cpu_material.normal_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        let emissive_texture = if let Some(ref cpu_texture) = cpu_material.emissive_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
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
            opaque_render_states: RenderStates::default(),
            transparent_render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
            emissive: cpu_material.emissive,
            emissive_texture,
            alpha_cutout: None,
        })
    }

    pub(crate) fn fragment_shader_source_internal(&self, use_vertex_colors: bool) -> String {
        let mut output = String::new();
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
                output.push_str("#define USE_NORMAL_TEXTURE;\n");
            }
            if self.emissive_texture.is_some() {
                output.push_str("#define USE_EMISSIVE_TEXTURE;\n");
            }
            if self.alpha_cutout.is_some() {
                output.push_str(format!("#define ALPHACUT;\nfloat acut = {};", self.alpha_cutout.unwrap()).as_str());
            }
        }
        if use_vertex_colors {
            output.push_str("#define USE_VERTEX_COLORS\nin vec4 col;\n");
        }
        output.push_str(include_str!("shaders/physical_material.frag"));
        output
    }

    pub(crate) fn use_uniforms_internal(&self, program: &Program) -> ThreeDResult<()> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;
        program.use_uniform_vec4("albedo", &self.albedo.to_vec4())?;
        if program.requires_uniform("emissive") {
            program.use_uniform_vec3("emissive", &self.emissive.to_vec3())?;
        }
        if let Some(ref texture) = self.albedo_texture {
            program.use_texture("albedoTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            program.use_uniform_float("occlusionStrength", &self.occlusion_strength)?;
            program.use_texture("occlusionTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.normal_texture {
            program.use_uniform_float("normalScale", &self.normal_scale)?;
            program.use_texture("normalTexture", texture.as_ref())?;
        }
        if program.requires_uniform("emissiveTexture") {
            if let Some(ref texture) = self.emissive_texture {
                program.use_texture("emissiveTexture", texture.as_ref())?;
            }
        }
        Ok(())
    }
}

impl ForwardMaterial for PhysicalMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String {
        let mut output = lights.fragment_shader_source();
        output.push_str(&self.fragment_shader_source_internal(use_vertex_colors));
        output
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        lights.use_uniforms(program, camera)?;
        self.use_uniforms_internal(program)?;
        Ok(())
    }

    fn render_states(&self) -> RenderStates {
        if self.is_transparent() {
            self.transparent_render_states
        } else {
            self.opaque_render_states
        }
    }
    fn is_transparent(&self) -> bool {
        match self.alpha_cutout {
            Some(_) => false,
            None => self.albedo.a != 255
                    || self
                        .albedo_texture
                        .as_ref()
                        .map(|t| t.is_transparent())
                        .unwrap_or(false),
        }
    }
}

impl DeferredMaterial for PhysicalMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String {
        format!(
            "#define DEFERRED\n{}",
            self.fragment_shader_source(use_vertex_colors, &Lights::default())
        )
    }
}

impl Default for PhysicalMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: Color::WHITE,
            albedo_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            opaque_render_states: RenderStates::default(),
            transparent_render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
            emissive: Color::BLACK,
            emissive_texture: None,
            alpha_cutout: None,
        }
    }
}
