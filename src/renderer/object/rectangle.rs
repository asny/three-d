use crate::renderer::*;

#[derive(Clone)]
pub struct Rectangle {
    model: Model2D,
    context: Context,
    width: f32,
    height: f32,
    center: Vec2,
    rotation: Radians,
}

impl Rectangle {
    pub fn new(
        context: &Context,
        center: Vec2,
        rotation: impl Into<Radians>,
        width: f32,
        height: f32,
    ) -> Result<Self> {
        let mut mesh = CPUMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5)));
        let mut rectangle = Self {
            model: Model2D::new(context, &mesh)?,
            context: context.clone(),
            width,
            height,
            center,
            rotation: rotation.into(),
        };
        rectangle.update();
        Ok(rectangle)
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.update();
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
        self.update();
    }

    pub fn set_rotation(&mut self, rotation: impl Into<Radians>) {
        self.rotation = rotation.into();
        self.update();
    }

    fn update(&mut self) {
        self.model.set_transformation(
            Mat3::from_translation(self.center)
                * Mat3::from_angle_z(self.rotation)
                * Mat3::from_nonuniform_scale(self.width, self.height),
        );
    }
}

impl std::ops::Deref for Rectangle {
    type Target = Model2D;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}
