mod tone_mapping;
pub use tone_mapping::*;

mod color_space;
pub use color_space::*;

mod camera;
pub use camera::*;

use crate::*;

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

///
/// The view frustum which can be used for frustum culling.
///
pub struct Frustum([Vec4; 6]);

impl Frustum {
    /// Computes the frustum for the given view-projection matrix.
    pub fn new(view_projection: Mat4) -> Self {
        let m = view_projection;
        Self([
            vec4(m.x.w + m.x.x, m.y.w + m.y.x, m.z.w + m.z.x, m.w.w + m.w.x),
            vec4(m.x.w - m.x.x, m.y.w - m.y.x, m.z.w - m.z.x, m.w.w - m.w.x),
            vec4(m.x.w + m.x.y, m.y.w + m.y.y, m.z.w + m.z.y, m.w.w + m.w.y),
            vec4(m.x.w - m.x.y, m.y.w - m.y.y, m.z.w - m.z.y, m.w.w - m.w.y),
            vec4(m.x.w + m.x.z, m.y.w + m.y.z, m.z.w + m.z.z, m.w.w + m.w.z),
            vec4(m.x.w - m.x.z, m.y.w - m.y.z, m.z.w - m.z.z, m.w.w - m.w.z),
        ])
    }

    /// Used for frustum culling. Returns false if the entire bounding box is outside of the frustum.
    pub fn contains(&self, aabb: AxisAlignedBoundingBox) -> bool {
        if aabb.is_infinite() {
            return true;
        }
        if aabb.is_empty() {
            return false;
        }
        // check box outside/inside of frustum
        for i in 0..6 {
            let mut out = 0;
            if self.0[i].dot(vec4(aabb.min().x, aabb.min().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.max().x, aabb.min().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.min().x, aabb.max().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.max().x, aabb.max().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.min().x, aabb.min().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.max().x, aabb.min().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.min().x, aabb.max().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.0[i].dot(vec4(aabb.max().x, aabb.max().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if out == 8 {
                return false;
            }
        }
        // TODO: Test the frustum corners against the box planes (http://www.iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm)

        true
    }
}
