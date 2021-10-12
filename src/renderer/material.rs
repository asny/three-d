use crate::core::*;
use crate::renderer::*;

pub use crate::core::{
    CPUMaterial, Color, GeometryFunction, LightingModel, NormalDistributionFunction,
};

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
    fn fragment_shader_source(&self, use_vertex_colors: bool) -> String;
    fn use_uniforms(&self, program: &Program) -> Result<()>;
    fn render_states(&self, transparent: bool) -> RenderStates;
    fn is_transparent(&self) -> bool;
}

impl ForwardMaterial for &dyn ForwardMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool) -> String {
        (*self).fragment_shader_source(use_vertex_colors)
    }
    fn use_uniforms(&self, program: &Program) -> Result<()> {
        (*self).use_uniforms(program)
    }
    fn render_states(&self, transparent: bool) -> RenderStates {
        (*self).render_states(transparent)
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

pub trait DeferredMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String;
    fn use_uniforms_deferred(&self, program: &Program) -> Result<()>;
    fn render_states_deferred(&self) -> RenderStates;
}

impl DeferredMaterial for &dyn DeferredMaterial {
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
