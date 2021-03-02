
use crate::math::*;

///
/// A light which shines equally on all parts of any surface.
///
pub struct AmbientLight
{
    pub color: Vec3,
    pub intensity: f32
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: vec3(1.0, 1.0, 1.0),
            intensity: 1.0
        }
     }
}
