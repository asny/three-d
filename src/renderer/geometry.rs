//!
//! A collection of objects that can be rendered, for example a mesh.
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

use crate::core::*;
use crate::renderer::*;

///
/// Represents a 3D geometry.
///
/// It is possible to render the geometry with a material that implements the [Material] trait.
///
/// If requested by the material, the geometry has to support the attributes position (in world space) `out vec3 pos;`,
/// normal `out vec3 nor;`, uv coordinates `out vec2 uvs;` and color `out vec4 col;` in the vertex shader source code.
///
pub trait Geometry {
    ///
    /// Render the geometry with the given material.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// You can use [Lights::default()] if you know the material does not require lights.
    ///
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()>;

    ///
    /// Returns the [AxisAlignedBoundingBox] for this geometry.
    ///
    fn aabb(&self) -> AxisAlignedBoundingBox;

    ///
    /// Returns the local to world transformation applied to this geometry.
    ///
    fn transformation(&self) -> Mat4;
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

    fn transformation(&self) -> Mat4 {
        (*self).transformation()
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

    fn transformation(&self) -> Mat4 {
        (**self).transformation()
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

    fn transformation(&self) -> Mat4 {
        self.as_ref().transformation()
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

    fn transformation(&self) -> Mat4 {
        self.as_ref().transformation()
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

    fn transformation(&self) -> Mat4 {
        self.borrow().transformation()
    }
}

///
/// Represents a 3D geometry.
///
pub trait GeometryMut: Geometry {
    ///
    /// Set the local to world transformation applied to this geometry.
    ///
    fn set_transformation(&mut self, transformation: Mat4);
}

impl<T: GeometryMut + ?Sized> GeometryMut for &mut T {
    fn set_transformation(&mut self, transformation: Mat4) {
        (*self).set_transformation(transformation);
    }
}

impl<T: GeometryMut> GeometryMut for std::rc::Rc<std::cell::RefCell<T>> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.borrow_mut().set_transformation(transformation);
    }
}

///
/// Represents a 2D geometry that is possible to render with [Material]s.
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
