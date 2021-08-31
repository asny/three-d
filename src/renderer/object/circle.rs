use crate::renderer::*;

#[derive(Clone)]
pub struct Circle {
    model: Model2D,
    context: Context,
    radius: f32,
    center: Vec2,
}

impl Circle {
    pub fn new(context: &Context, center: Vec2, radius: f32) -> Result<Self> {
        let mesh = CPUMesh::circle(64);
        let mut circle = Self {
            model: Model2D::new(context, &mesh)?,
            context: context.clone(),
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
        self.model.set_transformation(
            Mat3::from_translation(self.center) * Mat3::from_scale(self.radius),
        );
    }
}

impl std::ops::Deref for Circle {
    type Target = Model2D;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}
