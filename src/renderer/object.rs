//!
//! A collection of objects that can be rendered, for example a mesh.
//!

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

mod particles;
#[doc(inline)]
pub use particles::*;

use crate::core::*;
use crate::renderer::*;

///
/// Represents a 3D object which can be rendered.
///
pub trait Object: Geometry {
    ///
    /// Render the object.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// You can use [Lights::default()] if you know the object does not require lights to be rendered.
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
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
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
