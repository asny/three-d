use crate::renderer::*;

///
/// A rectangle 2D geometry which can be rendered using the [camera2d] camera.
///
pub struct Rectangle {
    mesh: Mesh,
    width: f32,
    height: f32,
    center: Vec2,
    rotation: Radians,
}

impl Rectangle {
    ///
    /// Constructs a new rectangle geometry.
    ///
    pub fn new(
        context: &Context,
        center: Vec2,
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
            center,
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
    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
        self.update();
    }

    /// Get the center of the rectangle.
    pub fn center(&self) -> &Vec2 {
        &self.center
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
            Mat3::from_translation(self.center)
                * Mat3::from_angle_z(self.rotation)
                * Mat3::from_nonuniform_scale(self.width, self.height),
        );
    }
}

impl Geometry for Rectangle {
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
            (self.center - 0.5 * vec2(self.width, self.height)).extend(0.0),
            (self.center + 0.5 * vec2(self.width, self.height)).extend(0.0),
        ])
    }
}

impl<'a> IntoIterator for &'a Rectangle {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}
