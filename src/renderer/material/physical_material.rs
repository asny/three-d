use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

#[deprecated = "Use 'PhysicalMaterial' instead."]
pub type Material = PhysicalMaterial;

///
/// A physically-based material used for shading an object.
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
    pub lighting_model: LightingModel,
    pub render_states: RenderStates,
    pub transparent_render_states: RenderStates,
}

impl PhysicalMaterial {
    ///
    /// Moves the material information from the [CPUMaterial] to the GPU.
    /// If the input contains an [CPUMaterial::occlusion_metallic_roughness_texture], this texture is used for both
    /// [Material::metallic_roughness_texture] and [Material::occlusion_texture] while any [CPUMaterial::metallic_roughness_texture] or [CPUMaterial::occlusion_texture] are ignored.
    ///
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> Result<Self> {
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
            lighting_model: LightingModel::Blinn,
            ..Default::default()
        })
    }

    fn bind_internal(&self, program: &Program) -> Result<()> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;
        program.use_uniform_vec4("albedo", &self.albedo.to_vec4())?;
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
        Ok(())
    }

    pub fn fragment_shader_source<L: Light>(
        &self,
        lights: &[L],
        use_vertex_colors: bool,
    ) -> String {
        let mut shader_source = lights_shader_source(self.lighting_model, lights);
        shader_source.push_str(&material_shader_source(self, use_vertex_colors));
        shader_source
    }
    pub fn use_uniforms<L: Light>(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[L],
    ) -> Result<()> {
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, camera, i as u32)?;
        }
        self.bind_internal(program)
    }

    pub fn render_states(&self, transparent: bool) -> RenderStates {
        if transparent || self.is_transparent() {
            self.transparent_render_states
        } else {
            self.render_states
        }
    }
    pub fn is_transparent(&self) -> bool {
        self.albedo.a != 255
            || self
                .albedo_texture
                .as_ref()
                .map(|t| t.is_transparent())
                .unwrap_or(false)
    }
}

impl DeferredMaterial for PhysicalMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String {
        format!(
            "#define DEFERRED\n{}",
            material_shader_source(self, use_vertex_colors)
        )
    }
    fn use_uniforms_deferred(&self, program: &Program) -> Result<()> {
        self.bind_internal(program)
    }

    fn render_states_deferred(&self) -> RenderStates {
        self.render_states
    }
}

impl DeferredMaterial for &PhysicalMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String {
        (*self).fragment_shader_source_deferred(use_vertex_colors)
    }
    fn use_uniforms_deferred(&self, program: &Program) -> Result<()> {
        (*self).use_uniforms_deferred(program)
    }
    fn render_states_deferred(&self) -> RenderStates {
        (*self).render_states_deferred()
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
            lighting_model: LightingModel::Blinn,
            render_states: RenderStates::default(),
            transparent_render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
        }
    }
}

pub(in crate::renderer) fn lights_shader_source<L: Light>(
    lighting_model: LightingModel,
    lights: &[L],
) -> String {
    let mut shader_source = match lighting_model {
        LightingModel::Phong => "#define PHONG",
        LightingModel::Blinn => "#define BLINN",
        LightingModel::Cook(normal, _) => match normal {
            NormalDistributionFunction::Blinn => "#define COOK\n#define COOK_BLINN\n",
            NormalDistributionFunction::Beckmann => "#define COOK\n#define COOK_BECKMANN\n",
            NormalDistributionFunction::TrowbridgeReitzGGX => "#define COOK\n#define COOK_GGX\n",
        },
    }
    .to_string();
    shader_source.push_str(include_str!("../../core/shared.frag"));
    shader_source.push_str(include_str!("shaders/light_shared.frag"));
    let mut dir_fun = String::new();
    for (i, light) in lights.iter().enumerate() {
        shader_source.push_str(&light.shader_source(i as u32));
        dir_fun.push_str(&format!("color += calculate_lighting{}(surface_color, position, normal, metallic, roughness, occlusion);", i))
    }
    shader_source.push_str(&format!(
        "
            vec3 calculate_lighting(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                vec3 color = vec3(0.0, 0.0, 0.0);
                {}
                return color;
            }}
            ",
        &dir_fun
    ));
    shader_source
}

fn material_shader_source(material: &PhysicalMaterial, use_vertex_colors: bool) -> String {
    let mut output = String::new();
    if material.albedo_texture.is_some()
        || material.metallic_roughness_texture.is_some()
        || material.normal_texture.is_some()
        || material.occlusion_texture.is_some()
    {
        output.push_str("in vec2 uvs;\n");
        if material.albedo_texture.is_some() {
            output.push_str("#define USE_ALBEDO_TEXTURE;\n");
        }
        if material.metallic_roughness_texture.is_some() {
            output.push_str("#define USE_METALLIC_ROUGHNESS_TEXTURE;\n");
        }
        if material.occlusion_texture.is_some() {
            output.push_str("#define USE_OCCLUSION_TEXTURE;\n");
        }
        if material.normal_texture.is_some() {
            output.push_str("#define USE_NORMAL_TEXTURE;\n");
        }
    }
    if use_vertex_colors {
        output.push_str("#define USE_VERTEX_COLORS\nin vec4 col;\n");
    }
    output.push_str(include_str!("shaders/physical_material.frag"));
    output
}
