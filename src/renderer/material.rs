use crate::core::*;
use crate::renderer::*;

pub use crate::core::{CPUMaterial, Color};

mod color_material;
#[doc(inline)]
pub use color_material::*;

mod depth_material;
#[doc(inline)]
pub use depth_material::*;

mod normal_material;
#[doc(inline)]
pub use normal_material::*;

mod uv_material;
#[doc(inline)]
pub use uv_material::*;

mod physical_material;
#[doc(inline)]
pub use physical_material::*;

pub trait ForwardMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String;
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &Lights) -> Result<()>;
    fn render_states(&self) -> RenderStates;
    fn is_transparent(&self) -> bool;
}

impl ForwardMaterial for &dyn ForwardMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String {
        (*self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

pub trait DeferredMaterial: ForwardMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String;
}

impl ForwardMaterial for &dyn DeferredMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String {
        (*self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

impl DeferredMaterial for &dyn DeferredMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String {
        (*self).fragment_shader_source_deferred(use_vertex_colors)
    }
}
