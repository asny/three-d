use crate::renderer::*;

///
/// A circle 2D geometry which can be rendered using the [camera2d] camera.
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

impl Geometry for Circle {
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
        let center: Vec2 = self.center.into();
        AxisAlignedBoundingBox::new_with_positions(&[
            (center - vec2(self.radius, self.radius)).extend(0.0),
            (center + vec2(self.radius, self.radius)).extend(0.0),
        ])
    }
}

impl<'a> IntoIterator for &'a Circle {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}
