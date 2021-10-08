//!
//! A collection of objects that can be rendered, for example a mesh.
//!

pub use crate::core::{AxisAlignedBoundingBox, CPUMesh};

mod model;
#[doc(inline)]
pub use model::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

mod line;
#[doc(inline)]
pub use line::*;

mod rectangle;
#[doc(inline)]
pub use rectangle::*;

mod circle;
#[doc(inline)]
pub use circle::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod axes;
#[doc(inline)]
pub use axes::*;

mod particles;
#[doc(inline)]
pub use particles::*;

use crate::core::*;
use crate::renderer::*;

pub struct Glue<'a, T: Shadable + Cullable> {
    pub geometry: &'a T,
    pub material: &'a dyn ForwardMaterial,
}

impl<'a, T: Shadable + Cullable> Drawable for Glue<'a, T> {
    fn render(&self, camera: &Camera) -> Result<()> {
        self.geometry.render_forward(self.material, camera)
    }
}

impl<'a, T: Shadable + Cullable> Cullable for Glue<'a, T> {
    fn in_frustum(&self, camera: &Camera) -> bool {
        self.geometry.in_frustum(camera)
    }
}

pub trait Shadable {
    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()>;

    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()>;
}

pub trait Cullable {
    fn in_frustum(&self, camera: &Camera) -> bool;
}

pub trait Shadable2D {
    fn render(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()>;
}

pub trait Drawable {
    ///
    /// Render the object.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render(&self, camera: &Camera) -> Result<()>;
}
