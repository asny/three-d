mod tone_mapping;
pub use tone_mapping::*;

mod color_space;
pub use color_space::*;

mod camera;
pub use camera::*;

use crate::*;

pub use three_d_asset::Frustum;

macro_rules! impl_viewer_body {
    ($inner:ident) => {
        fn position(&self) -> Vec3 {
            self.$inner().position()
        }

        fn view(&self) -> Mat4 {
            self.$inner().view()
        }

        fn projection(&self) -> Mat4 {
            self.$inner().projection()
        }

        fn viewport(&self) -> Viewport {
            self.$inner().viewport()
        }

        fn z_near(&self) -> f32 {
            self.$inner().z_near()
        }

        fn z_far(&self) -> f32 {
            self.$inner().z_far()
        }

        fn color_mapping(&self) -> ColorMapping {
            self.$inner().color_mapping()
        }

        fn tone_mapping(&self) -> ToneMapping {
            self.$inner().tone_mapping()
        }
    };
}

///
/// Represents a viewer, usually some kind of camera.
/// The default implementation of this trait is the [Camera] which should be adequate for most use cases.
///
pub trait Viewer {
    /// The position of the viewer.
    fn position(&self) -> Vec3;

    /// The view matrix which transforms from world space to view space.
    fn view(&self) -> Mat4;

    /// The projection matrix which transforms from view space to clip space (2D position on the screen).
    fn projection(&self) -> Mat4;

    /// The 2D [Viewport] of the viewer.
    fn viewport(&self) -> Viewport;

    /// Defines the minimum depth in world space.
    fn z_near(&self) -> f32;

    /// Defines the maximum depth in world space.
    fn z_far(&self) -> f32;

    /// Defines the [ColorMapping] applied to the final rendered image.
    fn color_mapping(&self) -> ColorMapping;

    /// Defines the [ToneMapping] applied to the final rendered image.
    fn tone_mapping(&self) -> ToneMapping;
}

use std::ops::Deref;
impl<T: Viewer + ?Sized> Viewer for &T {
    impl_viewer_body!(deref);
}

impl<T: Viewer + ?Sized> Viewer for &mut T {
    impl_viewer_body!(deref);
}

impl<T: Viewer> Viewer for Box<T> {
    impl_viewer_body!(as_ref);
}

impl<T: Viewer> Viewer for std::rc::Rc<T> {
    impl_viewer_body!(as_ref);
}

impl<T: Viewer> Viewer for std::sync::Arc<T> {
    impl_viewer_body!(as_ref);
}

impl<T: Viewer> Viewer for std::cell::RefCell<T> {
    impl_viewer_body!(borrow);
}

impl<T: Viewer> Viewer for std::sync::RwLock<T> {
    fn position(&self) -> Vec3 {
        self.read().unwrap().position()
    }

    fn view(&self) -> Mat4 {
        self.read().unwrap().view()
    }

    fn projection(&self) -> Mat4 {
        self.read().unwrap().projection()
    }

    fn viewport(&self) -> Viewport {
        self.read().unwrap().viewport()
    }

    fn z_near(&self) -> f32 {
        self.read().unwrap().z_near()
    }

    fn z_far(&self) -> f32 {
        self.read().unwrap().z_far()
    }

    fn color_mapping(&self) -> ColorMapping {
        self.read().unwrap().color_mapping()
    }

    fn tone_mapping(&self) -> ToneMapping {
        self.read().unwrap().tone_mapping()
    }
}
