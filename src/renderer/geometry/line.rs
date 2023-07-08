use crate::renderer::*;

///
/// A rectangle 2D geometry which can be rendered using a camera created by [Camera::new_2d].
///
pub struct Line {
    mesh: Mesh,
    pixel0: PhysicalPoint,
    pixel1: PhysicalPoint,
    thickness: f32,
}

impl Line {
    ///
    /// Constructs a new line geometry.
    ///
    pub fn new(
        context: &Context,
        pixel0: impl Into<PhysicalPoint>,
        pixel1: impl Into<PhysicalPoint>,
        thickness: f32,
    ) -> Self {
        let mut mesh = CpuMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))))
            .unwrap();
        let mut line = Self {
            mesh: Mesh::new(context, &mesh),
            pixel0: pixel0.into(),
            pixel1: pixel1.into(),
            thickness,
        };
        line.update();
        line
    }

    /// Get one of the end points of the line.
    pub fn end_point0(&self) -> PhysicalPoint {
        self.pixel0
    }

    /// Get one of the end points of the line.
    pub fn end_point1(&self) -> PhysicalPoint {
        self.pixel1
    }

    ///
    /// Change the two end points of the line.
    /// The pixel coordinates must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn set_endpoints(
        &mut self,
        pixel0: impl Into<PhysicalPoint>,
        pixel1: impl Into<PhysicalPoint>,
    ) {
        self.pixel0 = pixel0.into();
        self.pixel1 = pixel1.into();
        self.update();
    }

    /// Set the line thickness.
    pub fn set_thickness(&mut self, thickness: f32) {
        self.thickness = thickness;
        self.update();
    }

    fn update(&mut self) {
        let dx = self.pixel1.x - self.pixel0.x;
        let dy = self.pixel1.y - self.pixel0.y;
        let length = (dx * dx + dy * dy).sqrt();
        let c = dx / length;
        let s = dy / length;
        let rot = Mat3::new(c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0);
        self.mesh.set_transformation_2d(
            Mat3::from_translation(self.pixel0.into())
                * rot
                * Mat3::from_nonuniform_scale(length, self.thickness),
        );
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

use std::ops::Deref;
impl Deref for Line {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl std::ops::DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl Geometry for Line {
    impl_geometry_body!(deref);

    fn animate(&mut self, time: f32) {
        self.mesh.animate(time)
    }
}
