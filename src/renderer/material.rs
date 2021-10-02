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

mod pick_material;
#[doc(inline)]
pub use pick_material::*;

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
    fn fragment_shader_source(&self, lights: &[&dyn Light], use_vertex_colors: bool) -> String;
    fn bind(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) -> Result<()>;
    fn render_states(&self, transparent: bool) -> RenderStates;
    fn is_transparent(&self) -> bool;
}

pub trait DeferredMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool) -> String;
    fn bind(&self, program: &Program) -> Result<()>;
    fn render_states(&self) -> RenderStates;
}
