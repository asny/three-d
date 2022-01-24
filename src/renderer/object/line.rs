use crate::renderer::*;

#[derive(Clone)]
pub struct Line<M: Material> {
    context: Context,
    model: Model<M>,
    pixel0: Vec2,
    pixel1: Vec2,
    width: f32,
}

impl<M: Material> Line<M> {
    pub fn new_with_material(
        context: &Context,
        pixel0: Vec2,
        pixel1: Vec2,
        width: f32,
        material: M,
    ) -> ThreeDResult<Self> {
        let mut mesh = CPUMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))));
        let mut line = Self {
            context: context.clone(),
            model: Model::new_with_material(context, &mesh, material)?,
            pixel0,
            pixel1,
            width,
        };
        line.update();
        Ok(line)
    }

    pub fn end_point0(&self) -> Vec2 {
        self.pixel0
    }

    pub fn end_point1(&self) -> Vec2 {
        self.pixel1
    }

    ///
    /// Change the two end points of the line.
    /// The pixel coordinates must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn set_endpoints(&mut self, pixel0: Vec2, pixel1: Vec2) {
        self.pixel0 = pixel0;
        self.pixel1 = pixel1;
        self.update();
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.update();
    }

    fn update(&mut self) {
        let dx = self.pixel1.x - self.pixel0.x;
        let dy = self.pixel1.y - self.pixel0.y;
        let length = (dx * dx + dy * dy).sqrt();
        let c = dx / length;
        let s = dy / length;
        let rot = Mat3::new(c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0);
        self.model.set_transformation_2d(
            Mat3::from_translation(self.pixel0)
                * rot
                * Mat3::from_nonuniform_scale(length, self.width),
        );
    }
}

impl<M: Material> Shadable2D for Line<M> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model
                .render_with_material(material, camera2d, &Lights::default())
        })
    }
}

impl<M: Material> Object2D for Line<M> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render(camera2d, &Lights::default())
        })
    }

    fn is_transparent(&self) -> bool {
        self.model.is_transparent()
    }
}
