use crate::renderer::*;

#[derive(Clone)]
pub struct Circle {
    context: Context,
    model: Model,
    radius: f32,
    center: Vec2,
}

impl Circle {
    pub fn new(context: &Context, center: Vec2, radius: f32) -> Result<Self> {
        let mesh = CPUMesh::circle(64);
        let mut circle = Self {
            context: context.clone(),
            model: Model::new(context, &mesh)?,
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

impl Shadable2D for Circle {
    fn render(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render_forward(material, camera2d)
        })
    }
}
