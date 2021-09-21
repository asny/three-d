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

mod material;
#[doc(inline)]
pub use material::*;

pub trait ForwardMaterial {
    fn fragment_shader_source(
        &self,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> String;
    fn bind(
        &self,
        program: &Program,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()>;
    fn render_states(&self) -> RenderStates;
}

pub trait DeferredMaterial {
    fn render_states(&self) -> RenderStates;
    fn fragment_shader_source(&self) -> String;
    fn bind(&self, program: &Program) -> Result<()>;
}
