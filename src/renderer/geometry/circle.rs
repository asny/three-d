use crate::renderer::*;

///
/// A circle 2D geometry which can be rendered using a camera created by [Camera::new_2d].
///
pub struct Circle {
    mesh: Mesh,
    radius: f32,
    center: PhysicalPoint,
}

impl Circle {
    ///
    /// Constructs a new circle geometry.
    ///
    pub fn new(context: &Context, center: impl Into<PhysicalPoint>, radius: f32) -> Self {
        let mesh = CpuMesh::circle(64);
        let mut circle = Self {
            mesh: Mesh::new(context, &mesh),
            center: center.into(),
            radius,
        };
        circle.update();
        circle
    }

    /// Set the radius of the circle.
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.update();
    }

    /// Get the radius of the circle.
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Set the center of the circle.
    pub fn set_center(&mut self, center: impl Into<PhysicalPoint>) {
        self.center = center.into();
        self.update();
    }

    /// Get the center of the circle.
    pub fn center(&self) -> PhysicalPoint {
        self.center
    }

    fn update(&mut self) {
        self.mesh.set_transformation_2d(
            Mat3::from_translation(self.center.into()) * Mat3::from_scale(self.radius),
        );
    }
}

impl<'a> IntoIterator for &'a Circle {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

use std::ops::Deref;
impl Deref for Circle {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl std::ops::DerefMut for Circle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl Geometry for Circle {
    impl_geometry_body!(deref);

    fn animate(&mut self, time: f32) {
        self.mesh.animate(time)
    }
}
