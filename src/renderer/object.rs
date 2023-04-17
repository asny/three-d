#![macro_use]
//!
//! A collection of objects implementing the [Object] trait.
//!
//! Objects can be rendered directly or used in a render call, for example [RenderTarget::render].
//! Use the [Gm] struct to combine any [geometry] and [material] into an [Object].
//!

macro_rules! impl_object_body {
    ($inner:ident) => {
        fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
            self.$inner().render(camera, lights)
        }

        fn material_type(&self) -> MaterialType {
            self.$inner().material_type()
        }
    };
}

mod gm;
#[doc(inline)]
pub use gm::*;

mod model;
#[doc(inline)]
pub use model::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

mod voxel_grid;
#[doc(inline)]
pub use voxel_grid::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod terrain;
#[doc(inline)]
pub use terrain::*;

mod water;
#[doc(inline)]
pub use water::*;

mod axes;
#[doc(inline)]
pub use axes::*;

use crate::core::*;
use crate::renderer::*;

///
/// Represents a 3D object which can be rendered directly or used in a render call, for example [RenderTarget::render].
///
pub trait Object: Geometry {
    ///
    /// Render the object.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    fn render(&self, camera: &Camera, lights: &[&dyn Light]);

    ///
    /// Returns the type of material applied to this object.
    ///
    fn material_type(&self) -> MaterialType;
}

use std::ops::Deref;
impl<T: Object + ?Sized> Object for &T {
    impl_object_body!(deref);
}

impl<T: Object + ?Sized> Object for &mut T {
    impl_object_body!(deref);
}

impl<T: Object> Object for Box<T> {
    impl_object_body!(as_ref);
}

impl<T: Object> Object for std::rc::Rc<T> {
    impl_object_body!(as_ref);
}

impl<T: Object> Object for std::sync::Arc<T> {
    impl_object_body!(as_ref);
}

impl<T: Object> Object for std::cell::RefCell<T> {
    impl_object_body!(borrow);
}

impl<T: Object> Object for std::sync::RwLock<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.read().unwrap().render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
}
