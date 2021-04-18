use crate::camera::*;
use crate::core::*;
use crate::math::*;

pub trait Geometry {
    fn pick(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error>;

    fn render_depth(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error>;
}
