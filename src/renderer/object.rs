//!
//! A collection of objects (implementing the [Object] trait) that can be rendered directly or used in a render call, for example [render_pass].
//! Can be a combination of any [geometry] and [material] by using the [Gm] struct.
//!

mod gm;
#[doc(inline)]
pub use gm::*;

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

mod bounding_box;
#[doc(inline)]
pub use bounding_box::*;

use crate::core::*;
use crate::renderer::*;

pub use three_d_asset::Model as CpuModel;

///
/// Represents a 3D object which can be rendered directly or used in a render call, for example [render_pass].
///
pub trait Object: Geometry {
    ///
    /// Render the object.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()>;

    ///
    /// Returns whether or not this object should be considered transparent.
    ///
    fn is_transparent(&self) -> bool;
}

impl<T: Object + ?Sized> Object for &T {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        (*self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

impl<T: Object + ?Sized> Object for &mut T {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        (**self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (**self).is_transparent()
    }
}

impl<T: Object> Object for Box<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.as_ref().render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.as_ref().is_transparent()
    }
}

impl<T: Object> Object for std::rc::Rc<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.as_ref().render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.as_ref().is_transparent()
    }
}

impl<T: Object> Object for std::rc::Rc<std::cell::RefCell<T>> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.borrow().render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.borrow().is_transparent()
    }
}

// Object2D trait

///
/// Represents a 2D object which can be rendered.
///
pub trait Object2D: Geometry2D {
    ///
    /// Render the object.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    fn render(&self, viewport: Viewport) -> ThreeDResult<()>;

    ///
    /// Returns whether or not this object should be considered transparent.
    ///
    fn is_transparent(&self) -> bool;
}

impl<T: Object2D + ?Sized> Object2D for &T {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        (*self).render(viewport)
    }

    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

impl<T: Object2D + ?Sized> Object2D for &mut T {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        (**self).render(viewport)
    }

    fn is_transparent(&self) -> bool {
        (**self).is_transparent()
    }
}

impl<T: Object2D> Object2D for Box<T> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.as_ref().render(viewport)
    }

    fn is_transparent(&self) -> bool {
        self.as_ref().is_transparent()
    }
}

impl<T: Object2D> Object2D for std::rc::Rc<T> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.as_ref().render(viewport)
    }

    fn is_transparent(&self) -> bool {
        self.as_ref().is_transparent()
    }
}

impl<T: Object2D> Object2D for std::rc::Rc<std::cell::RefCell<T>> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.borrow().render(viewport)
    }

    fn is_transparent(&self) -> bool {
        self.borrow().is_transparent()
    }
}
