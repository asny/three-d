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
    pub ambient: Option<AmbientLight>,
    pub directional: Vec<DirectionalLight>,
    pub spot: Vec<SpotLight>,
    pub point: Vec<PointLight>,
}

impl Lights {
    pub const NONE: Self = Self {
        ambient: None,
        directional: Vec::new(),
        spot: Vec::new(),
        point: Vec::new(),
    };
}

impl Default for Lights {
    fn default() -> Self {
        Self::NONE
    }
}
