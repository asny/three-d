use crate::renderer::*;

///
/// A circle 2D object which can be rendered.
///
pub struct Circle<M: Material> {
    context: Context,
    model: Gm<Mesh, M>,
    radius: f32,
    center: Vec2,
}

impl<M: Material> Circle<M> {
    ///
    /// Constructs a new circle object with the given material.
    ///
    pub fn new_with_material(context: &Context, center: Vec2, radius: f32, material: M) -> Self {
        let mesh = CpuMesh::circle(64);
        let mut circle = Self {
            context: context.clone(),
            model: Gm::new(Mesh::new(context, &mesh), material),
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
        self.model.set_transformation_2d(
            Mat3::from_translation(self.center) * Mat3::from_scale(self.radius),
        );
    }
}

impl<M: Material> Geometry2D for Circle<M> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render_with_material(material, camera2d, &[])
        })
    }
}

impl<M: Material> Object2D for Circle<M> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.context
            .camera2d(viewport, |camera2d| self.model.render(camera2d, &[]))
    }

    fn material_type(&self) -> MaterialType {
        self.model.material_type()
    }
}
