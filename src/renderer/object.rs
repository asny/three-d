//!
//! A collection of objects that can be rendered, for example a mesh.
//!

pub use crate::core::{AxisAlignedBoundingBox, CPUMesh, Indices};

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
