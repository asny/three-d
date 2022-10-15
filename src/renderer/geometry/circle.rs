use crate::renderer::*;

///
/// A circle 2D object which can be rendered.
///
pub struct Circle {
    mesh: Mesh,
    radius: f32,
    center: Vec2,
}

impl Circle {
    ///
    /// Constructs a new circle object with the given material.
    ///
    pub fn new_with_material(context: &Context, center: Vec2, radius: f32) -> Self {
        let mesh = CpuMesh::circle(64);
        let mut circle = Self {
            mesh: Mesh::new(context, &mesh),
            center,
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
    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
        self.update();
    }

    /// Get the center of the circle.
    pub fn center(&self) -> &Vec2 {
        &self.center
    }

    fn update(&mut self) {
        self.mesh.set_transformation_2d(
            Mat3::from_translation(self.center) * Mat3::from_scale(self.radius),
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
        effect: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) {
        unimplemented!()
    }

    ///
    /// Returns the [AxisAlignedBoundingBox] for this geometry in the global coordinate system.
    ///
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(&[
            (self.center - vec2(self.radius, self.radius)).extend(0.0),
            (self.center + vec2(self.radius, self.radius)).extend(0.0),
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
