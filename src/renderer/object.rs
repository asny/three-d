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

// Object trait

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
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()>;

    ///
    /// Returns whether or not this object should be considered transparent.
    ///
    fn is_transparent(&self) -> bool;
}

impl<T: Object + ?Sized> Object for &T {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        (*self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

impl<T: Object + ?Sized> Object for &mut T {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        (**self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (**self).is_transparent()
    }
}

// Geometry trait

///
/// Represents a 3D geometry.
///
pub trait Geometry: Shadable {
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
    fn aabb(&self) -> AxisAlignedBoundingBox {
        (*self).aabb()
    }

    fn transformation(&self) -> Mat4 {
        (*self).transformation()
    }
}

impl<T: Geometry + ?Sized> Geometry for &mut T {
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

// Shadable trait

///
/// Represents a 3D object that is possible to render with [ForwardMaterial]s and [DeferredMaterial]s.
///
/// If requested by the material, the shadable object has to support the attributes position (in world space) `out vec3 pos;`,
/// normal `out vec3 nor;`, uv coordinates `out vec2 uvs;` and color `out vec4 col;` in the vertex shader source code.
///
pub trait Shadable {
    ///
    /// Render the object with the given material.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// You can use [Lights::default()] if you know the material does not require lights.
    ///
    fn render_with_material(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()>;

    ///
    /// Render the object with the given material.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// You can use [Lights::default()] if you know the material does not require lights.
    ///
    #[deprecated = "use render_with_material instead"]
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()>;

    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should usually not be called directly but used in [DeferredPipeline::geometry_pass].
    ///
    #[deprecated]
    fn render_deferred(
        &self,
        material: &DeferredPhysicalMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()>;
}

impl<T: Shadable + ?Sized> Shadable for &T {
    fn render_with_material(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        (*self).render_with_material(material, camera, lights)
    }

    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        (*self).render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &DeferredPhysicalMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

impl<T: Shadable + ?Sized> Shadable for &mut T {
    fn render_with_material(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        (**self).render_with_material(material, camera, lights)
    }

    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        (**self).render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &DeferredPhysicalMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (**self).render_deferred(material, camera, viewport)
    }
}

// Shadable2D trait

///
/// Represents a 2D object that is possible to render with [ForwardMaterial]s.
///
pub trait Shadable2D {
    ///
    /// Render the object with the given material.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        viewport: Viewport,
    ) -> ThreeDResult<()>;
}

impl<T: Shadable2D + ?Sized> Shadable2D for &T {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (*self).render_forward(material, viewport)
    }
}

impl<T: Shadable2D + ?Sized> Shadable2D for &mut T {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (**self).render_forward(material, viewport)
    }
}
// Object2D trait

///
/// Represents a 2D object which can be rendered.
///
pub trait Object2D: Shadable2D {
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
