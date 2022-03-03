use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// Render the object with colors that reflect its ORM (occlusion, roughness and metallic) values which primarily is used for debug purposes.
/// Occlusion is red, roughness green and metallic blue.
///
pub struct ORMMaterial<T: Texture> {
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    pub metallic_roughness_texture: Option<T>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    pub occlusion_texture: Option<T>,
    /// Render states.
    pub render_states: RenderStates,
}

impl ORMMaterial<Rc<Texture2D<u8>>> {
    /// Constructs a new ORM material from a [CpuMaterial] where only relevant information is used.
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> ThreeDResult<Self> {
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
        Ok(Self {
            metallic: cpu_material.metallic,
            roughness: cpu_material.roughness,
            metallic_roughness_texture,
            occlusion_texture,
            occlusion_strength: cpu_material.occlusion_strength,
            render_states: RenderStates::default(),
        })
    }
}

impl<T: Texture + Clone> ORMMaterial<T> {
    /// Creates a ORM material from a [PhysicalMaterial].
    pub fn from_physical_material<A: Texture, N: Texture, E: Texture>(
        physical_material: &PhysicalMaterial<A, T, N, E>,
    ) -> Self {
        Self {
            metallic: physical_material.metallic,
            roughness: physical_material.roughness,
            metallic_roughness_texture: physical_material.metallic_roughness_texture.clone(),
            occlusion_strength: physical_material.occlusion_strength,
            occlusion_texture: physical_material.occlusion_texture.clone(),
            render_states: RenderStates {
                write_mask: WriteMask::default(),
                blend: Blend::Disabled,
                ..physical_material.render_states
            },
        }
    }
}

impl<T: Texture> Material for ORMMaterial<T> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        let mut output = String::new();
        if self.metallic_roughness_texture.is_some() || self.occlusion_texture.is_some() {
            output.push_str("in vec2 uvs;\n");
            if self.metallic_roughness_texture.is_some() {
                output.push_str("#define USE_METALLIC_ROUGHNESS_TEXTURE;\n");
            }
            if self.occlusion_texture.is_some() {
                output.push_str("#define USE_OCCLUSION_TEXTURE;\n");
            }
        }
        output.push_str(include_str!("shaders/orm_material.frag"));
        output
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform("metallic", &self.metallic)?;
        program.use_uniform("roughness", &self.roughness)?;
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture)?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            program.use_uniform("occlusionStrength", &self.occlusion_strength)?;
            program.use_texture("occlusionTexture", texture)?;
        }
        Ok(())
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl<T: Texture + Clone> Clone for ORMMaterial<T> {
    fn clone(&self) -> Self {
        Self {
            metallic: self.metallic,
            roughness: self.roughness,
            metallic_roughness_texture: self.metallic_roughness_texture.clone(),
            occlusion_texture: self.occlusion_texture.clone(),
            occlusion_strength: self.occlusion_strength,
            render_states: self.render_states,
        }
    }
}

impl Default for ORMMaterial<Rc<Texture2D<u8>>> {
    fn default() -> Self {
        Self {
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            render_states: RenderStates::default(),
        }
    }
}
