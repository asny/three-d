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
    fn bind(&self, program: &Program, camera: &Camera) -> Result<()>;
    fn render_states(&self, transparent: bool) -> RenderStates;
    fn is_transparent(&self) -> bool;
}

pub trait DeferredMaterial {
    fn fragment_shader_source_deferred(&self, use_vertex_colors: bool) -> String;
    fn use_deferred(&self, program: &Program) -> Result<()>;
    fn render_states_deferred(&self) -> RenderStates;
}

pub struct LitMaterial<'a> {
    pub material: &'a PhysicalMaterial,
    pub lights: &'a [&'a dyn Light],
}

impl<'a> ForwardMaterial for LitMaterial<'a> {
    fn fragment_shader_source(&self, use_vertex_colors: bool) -> String {
        self.material
            .fragment_shader_source(self.lights, use_vertex_colors)
    }

    fn bind(&self, program: &Program, camera: &Camera) -> Result<()> {
        self.material.bind(program, camera, self.lights)
    }
    fn render_states(&self, transparent: bool) -> RenderStates {
        self.material.render_states(transparent)
    }
    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}
