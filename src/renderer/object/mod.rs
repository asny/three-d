//!
//! A collection of objects that can be rendered, for example a mesh.
//!

pub use crate::core::{CPUMesh, CullType};

mod model;
#[doc(inline)]
pub use model::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod axes;
#[doc(inline)]
pub use axes::*;

use crate::core::*;

pub trait Geometry {
    ///
    /// Render only the depth into the current depth render target which is useful for shadow maps or depth pre-pass.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth(&self, camera: &Camera) -> Result<(), Error>;

    ///
    /// Render the depth (scaled such that a value of 1 corresponds to max_depth) into the red channel of the current color render target which for example is used for picking.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<(), Error>;

    fn aabb(&self) -> AxisAlignedBoundingBox;
}
