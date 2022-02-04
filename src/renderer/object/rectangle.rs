use crate::renderer::*;

#[derive(Clone)]
pub struct Rectangle<M: Material> {
    model: Model<M>,
    context: Context,
    width: f32,
    height: f32,
    center: Vec2,
    rotation: Radians,
}

impl<M: Material> Rectangle<M> {
    pub fn new_with_material(
        context: &Context,
        center: Vec2,
        rotation: impl Into<Radians>,
        width: f32,
        height: f32,
        material: M,
    ) -> ThreeDResult<Self> {
        let mut mesh = CpuMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5)));
        let mut rectangle = Self {
            model: Model::new_with_material(context, &mesh, material)?,
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

impl<M: Material> Geometry2D for Rectangle<M> {
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

impl<M: Material> Object2D for Rectangle<M> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.context
            .camera2d(viewport, |camera2d| self.model.render(camera2d, &[]))
    }

    fn is_transparent(&self) -> bool {
        self.model.is_transparent()
    }
}
