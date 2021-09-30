//!
//! A collection of light types.
//! Currently implemented light types are ambient light, directional light, spot light and point light.
//! Directional and spot lights can cast shadows.
//!

mod directional_light;
#[doc(inline)]
pub use directional_light::*;

mod spot_light;
#[doc(inline)]
pub use spot_light::*;

mod point_light;
#[doc(inline)]
pub use point_light::*;

mod ambient_light;
#[doc(inline)]
pub use ambient_light::*;

use crate::core::*;

pub trait Light {
    fn shader_source(&self, i: u32) -> String;
    fn bind(&self, program: &Program, camera: &Camera, i: u32) -> Result<()>;
}
