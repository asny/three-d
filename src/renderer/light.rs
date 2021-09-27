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

pub struct Lights {
    pub ambient_light: Option<AmbientLight>,
    pub directional_lights: Vec<DirectionalLight>,
    pub spot_lights: Vec<SpotLight>,
    pub point_lights: Vec<PointLight>,
}

impl Lights {
    pub const NONE: Self = Self {
        ambient_light: None,
        directional_lights: Vec::new(),
        spot_lights: Vec::new(),
        point_lights: Vec::new(),
    };
}

impl Default for Lights {
    fn default() -> Self {
        Self::NONE
    }
}
