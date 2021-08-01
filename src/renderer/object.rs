//!
//! A collection of objects that can be rendered, for example a mesh.
//!

mod cpu_mesh;
#[doc(inline)]
pub use cpu_mesh::*;

mod mesh;
#[doc(inline)]
pub use mesh::*;

mod instanced_mesh;
#[doc(inline)]
pub use instanced_mesh::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod particles;
#[doc(inline)]
pub use particles::*;

mod axes;
#[doc(inline)]
pub use axes::*;

use crate::core::*;
use crate::renderer::*;

pub trait Geometry {
    ///
    /// Render only the depth into the current depth render target which is useful for shadow maps or depth pre-pass.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth(&self, render_states: RenderStates, camera: &Camera) -> Result<(), Error>;

    ///
    /// Render the depth (scaled such that a value of 1 corresponds to max_depth) into the red channel of the current color render target which for example is used for picking.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth_to_red(
        &self,
        render_states: RenderStates,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error>;

    fn aabb(&self) -> Option<AxisAlignedBoundingBox>;
}
