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
}

impl ForwardMaterial for PhysicalMaterial {
    fn fragment_shader_source(&self) -> String {
        let mut shader_source = shaded_fragment_shader(self.lighting_model);
        shader_source.push_str(&material_shader(self));
        shader_source.push_str(include_str!("shaders/physical_material.frag"));
        shader_source
    }
    fn bind(&self, program: &Program, camera: &Camera, lights: &Lights) -> Result<()> {
        bind_lights(program, lights, camera.position())?;
        self.bind_internal(program)
    }

    fn render_states(&self) -> RenderStates {
        let transparent = self.albedo.a != 255
            || self
                .albedo_texture
                .as_ref()
                .map(|t| t.is_transparent())
                .unwrap_or(false);

        if transparent {
            RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            }
        } else {
            RenderStates::default()
        }
    }
}

impl DeferredMaterial for PhysicalMaterial {
    fn fragment_shader_source(&self) -> String {
        format!(
            "#define DEFERRED\n{}{}",
            material_shader(self),
            include_str!("shaders/physical_material.frag")
        )
    }
    fn bind(&self, program: &Program) -> Result<()> {
        self.bind_internal(program)
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
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
        }
    }
}

const MAX_DIRECTIONAL_LIGHTS: usize = 5;
const MAX_SPOT_LIGHTS: usize = 5;
const MAX_POINT_LIGHTS: usize = 5;

pub(in crate::renderer) fn bind_lights(
    program: &Program,
    lights: &Lights,
    camera_position: &Vec3,
) -> Result<()> {
    if lights.directional.len() > MAX_DIRECTIONAL_LIGHTS
        || lights.spot.len() > MAX_SPOT_LIGHTS
        || lights.point.len() > MAX_POINT_LIGHTS
    {
        Err(RendererError::TooManyLights)?;
    }

    // Ambient light
    program.use_uniform_vec3(
        "ambientColor",
        &lights
            .ambient
            .as_ref()
            .map(|light| light.color.to_vec3() * light.intensity)
            .unwrap_or(vec3(0.0, 0.0, 0.0)),
    )?;

    if !lights.directional.is_empty() || !lights.spot.is_empty() || !lights.point.is_empty() {
        program.use_uniform_vec3("eyePosition", camera_position)?;
    }

    // Directional light
    for i in 0..MAX_DIRECTIONAL_LIGHTS {
        if let Some(light) = lights.directional.get(i) {
            if let Some(tex) = light.shadow_map() {
                program.use_texture(&format!("directionalShadowMap{}", i), tex)?;
            }
            program.use_uniform_block(&format!("DirectionalLightUniform{}", i), light.buffer());
            program.use_uniform_float(&format!("useDirectionalLight{}", i), &1.0)?;
        } else {
            program.use_uniform_float(&format!("useDirectionalLight{}", i), &0.0)?;
        }
    }

    // Spot light
    for i in 0..MAX_SPOT_LIGHTS {
        if let Some(light) = lights.spot.get(i) {
            if let Some(tex) = light.shadow_map() {
                program.use_texture(&format!("spotShadowMap{}", i), tex)?;
            }
            program.use_uniform_block(&format!("SpotLightUniform{}", i), light.buffer());
            program.use_uniform_float(&format!("useSpotLight{}", i), &1.0)?;
        } else {
            program.use_uniform_float(&format!("useSpotLight{}", i), &0.0)?;
        }
    }

    // Point light
    for i in 0..MAX_POINT_LIGHTS {
        if let Some(light) = lights.point.get(i) {
            program.use_uniform_block(&format!("PointLightUniform{}", i), light.buffer());
            program.use_uniform_float(&format!("usePointLight{}", i), &1.0)?;
        } else {
            program.use_uniform_float(&format!("usePointLight{}", i), &0.0)?;
        }
    }
    Ok(())
}

pub(in crate::renderer) fn shaded_fragment_shader(lighting_model: LightingModel) -> String {
    let mut dir_uniform = String::new();
    let mut dir_fun = String::new();
    for i in 0..MAX_DIRECTIONAL_LIGHTS {
        dir_uniform.push_str(&format!(
            "
                uniform sampler2D directionalShadowMap{};
                uniform float useDirectionalLight{};
                layout (std140) uniform DirectionalLightUniform{}
                {{
                    DirectionalLight directionalLight{};
                }};",
            i, i, i, i
        ));
        dir_fun.push_str(&format!("
            if(useDirectionalLight{} > 0.5) {{
                    color += calculate_directional_light(directionalLight{}, surface_color, position, normal, metallic, roughness, occlusion, directionalShadowMap{});
                }}", i, i, i));
    }
    let mut spot_uniform = String::new();
    let mut spot_fun = String::new();
    for i in 0..MAX_SPOT_LIGHTS {
        spot_uniform.push_str(&format!(
            "
                uniform float useSpotLight{};
                uniform sampler2D spotShadowMap{};
                layout (std140) uniform SpotLightUniform{}
                {{
                    SpotLight spotLight{};
                }};",
            i, i, i, i
        ));
        spot_fun.push_str(&format!(
            "if(useSpotLight{} > 0.5) {{
                    color += calculate_spot_light(spotLight{}, surface_color, position, normal, metallic, roughness, occlusion, spotShadowMap{});
            }}",
            i, i, i
        ));
    }
    let mut point_uniform = String::new();
    let mut point_fun = String::new();
    for i in 0..MAX_POINT_LIGHTS {
        point_uniform.push_str(&format!(
            "
                uniform float usePointLight{};
                layout (std140) uniform PointLightUniform{}
                {{
                    PointLight pointLight{};
                }};",
            i, i, i
        ));
        point_fun.push_str(&format!(
            "if(usePointLight{} > 0.5) {{
                    color += calculate_point_light(pointLight{}, surface_color, position, normal, metallic, roughness, occlusion);
            }}",
            i, i
        ));
    }

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
    shader_source.push_str(&format!(
        "
            uniform vec3 ambientColor;
            {} // Directional lights
            {} // Spot lights
            {} // Point lights

            vec3 calculate_lighting(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                vec3 color = occlusion * ambientColor * mix(surface_color, vec3(0.0), metallic); // Ambient light
                {} // Directional lights
                {} // Spot lights
                {} // Point lights
                return color;
            }}
            ",
        &dir_uniform, &spot_uniform, &point_uniform, &dir_fun, &spot_fun, &point_fun
    ));
    shader_source
}

fn material_shader(material: &PhysicalMaterial) -> String {
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
    output
}
