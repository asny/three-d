use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct NormalMaterial {
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Rc<Texture2D>>,
    pub render_states: RenderStates,
}

impl NormalMaterial {
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> ThreeDResult<Self> {
        let normal_texture = if let Some(ref cpu_texture) = cpu_material.normal_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
            normal_scale: cpu_material.normal_scale,
            normal_texture: normal_texture,
            render_states: RenderStates::default(),
        })
    }
}

impl ForwardMaterial for NormalMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &Lights) -> String {
        let mut shader = String::new();
        if self.normal_texture.is_some() {
            shader.push_str("#define USE_TEXTURE\nin vec2 uvs;\n");
        }
        shader.push_str(include_str!("../../core/shared.frag"));
        shader.push_str(include_str!("shaders/normal_material.frag"));
        shader
    }
    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &Lights,
    ) -> ThreeDResult<()> {
        if let Some(ref tex) = self.normal_texture {
            program.use_uniform_float("normalScale", &self.normal_scale)?;
            program.use_texture("normalTexture", &**tex)?;
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

impl ForwardMaterial for &NormalMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String {
        (*self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        (*self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

impl Default for NormalMaterial {
    fn default() -> Self {
        Self {
            normal_texture: None,
            normal_scale: 1.0,
            render_states: RenderStates::default(),
        }
    }
}
