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

pub struct Glue<M: ForwardMaterial, G: Geometry> {
    pub geometry: G,
    pub material: M,
}

impl<M: ForwardMaterial, G: Geometry> Shadable for Glue<M, G> {
    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()> {
        self.geometry.render_forward(material, camera)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        self.geometry.render_deferred(material, camera, viewport)
    }
}

impl<M: ForwardMaterial, G: Geometry> Drawable for Glue<M, G> {
    fn render(&self, camera: &Camera) -> Result<()> {
        self.geometry.render_forward(&self.material, camera)
    }
}

impl<M: ForwardMaterial, G: Geometry> Cullable for Glue<M, G> {
    fn in_frustum(&self, camera: &Camera) -> bool {
        self.geometry.in_frustum(camera)
    }
}

impl<M: ForwardMaterial, G: Geometry> Object for Glue<M, G> {}

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

impl Shadable for &dyn Shadable {
    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()> {
        (*self).render_forward(material, camera)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

impl Shadable for &dyn Geometry {
    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()> {
        (*self).render_forward(material, camera)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

pub trait Cullable {
    fn in_frustum(&self, camera: &Camera) -> bool;
}

impl Cullable for &dyn Cullable {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

impl Cullable for &dyn Object {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

impl Cullable for &dyn Geometry {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

pub trait Drawable {
    ///
    /// Render the object.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render(&self, camera: &Camera) -> Result<()>;
}

impl Drawable for &dyn Drawable {
    fn render(&self, camera: &Camera) -> Result<()> {
        (*self).render(camera)
    }
}

impl Drawable for &dyn Object {
    fn render(&self, camera: &Camera) -> Result<()> {
        (*self).render(camera)
    }
}

pub trait Object: Drawable + Cullable {}

impl Object for &dyn Object {}

pub trait Geometry: Shadable + Cullable {}

impl Geometry for &dyn Geometry {}

pub trait Shadable2D {
    fn render_forward(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()>;
}

pub trait Cullable2D {
    fn in_frustum(&self, viewport: Viewport) -> bool;
}

pub trait Geometry2D: Shadable2D + Cullable2D {}
