use crate::renderer::*;

///
/// A rectangle 2D geometry which can be rendered using the [camera2d] camera.
///
pub struct Line {
    mesh: Mesh,
    pixel0: Vec2,
    pixel1: Vec2,
    thickness: f32,
}

impl Line {
    ///
    /// Constructs a new line geometry.
    ///
    pub fn new(context: &Context, pixel0: Vec2, pixel1: Vec2, thickness: f32) -> Self {
        let mut mesh = CpuMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))))
            .unwrap();
        let mut line = Self {
            mesh: Mesh::new(context, &mesh),
            pixel0,
            pixel1,
            thickness,
        };
        line.update();
        line
    }

    /// Get one of the end points of the line.
    pub fn end_point0(&self) -> Vec2 {
        self.pixel0
    }

    /// Get one of the end points of the line.
    pub fn end_point1(&self) -> Vec2 {
        self.pixel1
    }

    ///
    /// Change the two end points of the line.
    /// The pixel coordinates must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn set_endpoints(&mut self, pixel0: Vec2, pixel1: Vec2) {
        self.pixel0 = pixel0;
        self.pixel1 = pixel1;
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
            Mat3::from_translation(self.pixel0)
                * rot
                * Mat3::from_nonuniform_scale(length, self.thickness),
        );
    }
}

impl Geometry for Line {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.mesh.render_with_material(material, camera, lights)
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.mesh
            .render_with_post_material(material, camera, lights, color_texture, depth_texture)
    }

    ///
    /// Returns the [AxisAlignedBoundingBox] for this geometry in the global coordinate system.
    ///
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(&[
            self.pixel0.extend(0.0),
            self.pixel1.extend(0.0),
        ])
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}
