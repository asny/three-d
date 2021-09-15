use crate::core::*;
use crate::renderer::*;

pub use crate::core::{
    CPUMaterial, Color, GeometryFunction, LightingModel, NormalDistributionFunction, Program,
};

mod color_material;
#[doc(inline)]
pub use color_material::*;

mod texture_material;
#[doc(inline)]
pub use texture_material::*;

mod material;
#[doc(inline)]
pub use material::*;

pub trait Paint {
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
    fn transparent(&self) -> bool;
}
