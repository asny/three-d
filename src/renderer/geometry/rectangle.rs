use crate::renderer::*;

///
/// A rectangle 2D geometry which can be rendered using a camera created by [Camera::new_2d].
///
pub struct Rectangle {
    mesh: Mesh,
    width: f32,
    height: f32,
    center: PhysicalPoint,
    rotation: Radians,
}

impl Rectangle {
    ///
    /// Constructs a new rectangle geometry.
    ///
    pub fn new(
        context: &Context,
        center: impl Into<PhysicalPoint>,
        rotation: impl Into<Radians>,
        width: f32,
        height: f32,
    ) -> Self {
        let mut mesh = CpuMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5))).unwrap();
        let mut rectangle = Self {
            mesh: Mesh::new(context, &mesh),
            width,
            height,
            center: center.into(),
            rotation: rotation.into(),
        };
        rectangle.update();
        rectangle
    }

    /// Set the size of the rectangle.
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.update();
    }

    /// Get the size of the rectangle.
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Set the center of the rectangle.
    pub fn set_center(&mut self, center: impl Into<PhysicalPoint>) {
        self.center = center.into();
        self.update();
    }

    /// Get the center of the rectangle.
    pub fn center(&self) -> PhysicalPoint {
        self.center
    }

    /// Set the rotation of the rectangle.
    pub fn set_rotation(&mut self, rotation: impl Into<Radians>) {
        self.rotation = rotation.into();
        self.update();
    }

    /// Get the rotation of the rectangle.
    pub fn rotation(&self) -> Radians {
        self.rotation
    }

    fn update(&mut self) {
        self.mesh.set_transformation_2d(
            Mat3::from_translation(self.center.into())
                * Mat3::from_angle_z(self.rotation)
                * Mat3::from_nonuniform_scale(self.width, self.height),
        );
    }
}

impl<'a> IntoIterator for &'a Rectangle {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

use std::ops::Deref;
impl Deref for Rectangle {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl std::ops::DerefMut for Rectangle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl Geometry for Rectangle {
    impl_geometry_body!(deref);

    fn animate(&mut self, time: f32) {
        self.mesh.animate(time)
    }
}
