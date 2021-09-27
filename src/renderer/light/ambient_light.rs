use crate::Color;

///
/// A light which shines equally on all parts of any surface.
///
#[derive(Clone, Debug)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
        }
    }
}
