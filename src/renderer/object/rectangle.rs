use crate::renderer::*;

#[derive(Clone)]
pub struct Rectangle {
    model: Model,
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
            model: Model::new(context, &mesh)?,
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

    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
        self.update();
    }

    pub fn center(&self) -> &Vec2 {
        &self.center
    }

    pub fn set_rotation(&mut self, rotation: impl Into<Radians>) {
        self.rotation = rotation.into();
        self.update();
    }

    pub fn rotation(&self) -> Radians {
        self.rotation
    }

    fn update(&mut self) {
        self.model.set_transformation_2d(
            Mat3::from_translation(self.center)
                * Mat3::from_angle_z(self.rotation)
                * Mat3::from_nonuniform_scale(self.width, self.height),
        );
    }
}

impl Shadable2D for Rectangle {
    fn render(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render_forward(material, camera2d)
        })
    }
}
