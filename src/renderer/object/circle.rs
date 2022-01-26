use crate::renderer::*;

#[derive(Clone)]
pub struct Circle<M: Material> {
    context: Context,
    model: Model<M>,
    radius: f32,
    center: Vec2,
}

impl<M: Material> Circle<M> {
    pub fn new_with_material(
        context: &Context,
        center: Vec2,
        radius: f32,
        material: M,
    ) -> ThreeDResult<Self> {
        let mesh = CPUMesh::circle(64);
        let mut circle = Self {
            context: context.clone(),
            model: Model::new_with_material(context, &mesh, material)?,
            center,
            radius,
        };
        circle.update();
        Ok(circle)
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.update();
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
        self.update();
    }

    pub fn center(&self) -> &Vec2 {
        &self.center
    }

    fn update(&mut self) {
        self.model.set_transformation_2d(
            Mat3::from_translation(self.center) * Mat3::from_scale(self.radius),
        );
    }
}

impl<M: Material> Shadable2D for Circle<M> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model
                .render_with_material(material, camera2d, &[])
        })
    }
}

impl<M: Material> Object2D for Circle<M> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render(camera2d, &[])
        })
    }

    fn is_transparent(&self) -> bool {
        self.model.is_transparent()
    }
}
