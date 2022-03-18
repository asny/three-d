//!
//! A collection of geometries implementing the [Geometry] trait, for example a [Mesh].
//! A geometry together with a [material] can be rendered directly, or combined into an [object] (see [Gm]) that can be used in a render call, for example [render_pass].
//!

mod mesh;
#[doc(inline)]
pub use mesh::*;

mod instanced_mesh;
#[doc(inline)]
pub use instanced_mesh::*;

mod sprites;
#[doc(inline)]
pub use sprites::*;

mod particles;
#[doc(inline)]
pub use particles::*;

use crate::core::*;
use crate::renderer::*;

///
/// Represents a 3D geometry that, together with a [material], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [render_pass].
///
/// If requested by the material, the geometry has to support the following attributes in the vertex shader source code.
/// - position (in world space): `out vec3 pos;`
/// - normal: `out vec3 nor;`
/// - tangent: `out vec3 tang;`
/// - bitangent: `out vec3 bitang;`
/// - uv coordinates: `out vec2 uvs;`
/// - color: `out vec4 col;`
///
pub trait Geometry {
    ///
    /// Render the geometry with the given material.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    /// You can use an empty array for the `lights` argument, if you know the object does not require lights to be rendered.
    ///
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()>;

    ///
    /// Returns the [AxisAlignedBoundingBox] for this geometry in the global coordinate system.
    ///
    fn aabb(&self) -> AxisAlignedBoundingBox;
}

impl<T: Geometry + ?Sized> Geometry for &T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        (*self).render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

impl<T: Geometry + ?Sized> Geometry for &mut T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        (**self).render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        (**self).aabb()
    }
}

impl<T: Geometry> Geometry for Box<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.as_ref().aabb()
    }
}

impl<T: Geometry> Geometry for std::rc::Rc<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.as_ref().aabb()
    }
}

impl<T: Geometry> Geometry for std::rc::Rc<std::cell::RefCell<T>> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.borrow().render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.borrow().aabb()
    }
}

///
/// Represents a 2D geometry that is possible to render with a [Material].
///
pub trait Geometry2D {
    ///
    /// Render the object with the given material.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_with_material(&self, material: &dyn Material, viewport: Viewport)
        -> ThreeDResult<()>;
}

impl<T: Geometry2D + ?Sized> Geometry2D for &T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (*self).render_with_material(material, viewport)
    }
}

impl<T: Geometry2D + ?Sized> Geometry2D for &mut T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (**self).render_with_material(material, viewport)
    }
}

impl<T: Geometry2D> Geometry2D for Box<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, viewport)
    }
}

impl<T: Geometry2D> Geometry2D for std::rc::Rc<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, viewport)
    }
}

impl<T: Geometry2D> Geometry2D for std::rc::Rc<std::cell::RefCell<T>> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.borrow().render_with_material(material, viewport)
    }
}
