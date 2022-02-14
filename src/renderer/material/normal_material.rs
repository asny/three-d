use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

pub struct NormalMaterial<T: Texture> {
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<T>,
    pub render_states: RenderStates,
}

impl NormalMaterial<Rc<Texture2D<u8>>> {
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> ThreeDResult<Self> {
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

impl<T: Texture + Clone> NormalMaterial<T> {
    pub fn from_physical_material<A: Texture, ORM: Texture, E: Texture>(
        physical_material: &PhysicalMaterial<A, ORM, T, E>,
    ) -> Self {
        Self {
            normal_scale: physical_material.normal_scale,
            normal_texture: physical_material.normal_texture.clone(),
            render_states: RenderStates {
                write_mask: WriteMask::default(),
                blend: Blend::Disabled,
                ..physical_material.render_states
            },
        }
    }
}

impl<T: Texture> Material for NormalMaterial<T> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        let mut shader = String::new();
        if self.normal_texture.is_some() {
            shader.push_str("#define USE_TEXTURE\nin vec2 uvs;\nin vec3 tang;\nin vec3 bitang;\n");
        }
        shader.push_str(include_str!("shaders/normal_material.frag"));
        shader
    }
    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        if let Some(ref tex) = self.normal_texture {
            program.use_uniform_float("normalScale", &self.normal_scale)?;
            program.use_texture("normalTexture", tex)?;
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

impl<T: Texture + Clone> Clone for NormalMaterial<T> {
    fn clone(&self) -> Self {
        Self {
            normal_scale: self.normal_scale,
            normal_texture: self.normal_texture.clone(),
            render_states: self.render_states,
        }
    }
}

impl Default for NormalMaterial<Rc<Texture2D<u8>>> {
    fn default() -> Self {
        Self {
            normal_texture: None,
            normal_scale: 1.0,
            render_states: RenderStates::default(),
        }
    }
}
