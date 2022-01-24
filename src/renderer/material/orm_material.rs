use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct ORMMaterial {
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters which are multiplied with the [Self::metallic] and [Self::roughness] values in the shader.
    /// The metallic values are sampled from the blue channel and the roughness from the green channel.
    pub metallic_roughness_texture: Option<Rc<Texture2D<u8>>>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    pub occlusion_texture: Option<Rc<Texture2D<u8>>>,
    /// Render states.
    pub render_states: RenderStates,
}

impl ORMMaterial {
    pub fn from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            metallic: physical_material.metallic,
            roughness: physical_material.roughness,
            metallic_roughness_texture: physical_material.metallic_roughness_texture.clone(),
            occlusion_strength: physical_material.occlusion_strength,
            occlusion_texture: physical_material.occlusion_texture.clone(),
            render_states: physical_material.opaque_render_states,
        }
    }
}

impl Material for ORMMaterial {
    fn fragment_shader_source(
        &self,
        _use_vertex_colors: bool,
        _lights: &mut dyn std::iter::Iterator<Item = &dyn Light>,
    ) -> String {
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
        _lights: &mut dyn std::iter::Iterator<Item = &dyn Light>,
    ) -> ThreeDResult<()> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            program.use_uniform_float("occlusionStrength", &self.occlusion_strength)?;
            program.use_texture("occlusionTexture", texture.as_ref())?;
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
