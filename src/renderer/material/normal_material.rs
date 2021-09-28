use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct NormalMaterial {
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Rc<Texture2D>>,
}

impl NormalMaterial {
    pub fn new_from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            normal_scale: physical_material.normal_scale,
            normal_texture: physical_material.normal_texture.clone(),
        }
    }
}

impl ForwardMaterial for NormalMaterial {
    fn fragment_shader_source(&self) -> String {
        let mut shader = String::new();
        if self.normal_texture.is_some() {
            shader.push_str(include_str!("../../core/shared.frag"));
            shader.push_str("#define USE_TEXTURE\nin vec2 uvs;\n");
        }
        shader.push_str(include_str!("shaders/normal_material.frag"));
        shader
    }
    fn bind(&self, program: &Program, _camera: &Camera, _lights: &Lights) -> Result<()> {
        if let Some(ref tex) = self.normal_texture {
            program.use_uniform_float("normalScale", &self.normal_scale)?;
            program.use_texture("normalTexture", &**tex)?;
        }
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }
}

impl Default for NormalMaterial {
    fn default() -> Self {
        Self {
            normal_texture: None,
            normal_scale: 1.0,
        }
    }
}
