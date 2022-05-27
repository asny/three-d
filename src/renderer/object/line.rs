use crate::renderer::*;

///
/// A line 2D object which can be rendered.
///
pub struct Line<M: Material> {
    context: Context,
    model: Gm<Mesh, M>,
    pixel0: Vec2,
    pixel1: Vec2,
    thickness: f32,
}

impl<M: Material> Line<M> {
    ///
    /// Constructs a new line object with the given material.
    ///
    pub fn new_with_material(
        context: &Context,
        pixel0: Vec2,
        pixel1: Vec2,
        thickness: f32,
        material: M,
    ) -> ThreeDResult<Self> {
        let mut mesh = CpuMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))))?;
        let mut line = Self {
            context: context.clone(),
            model: Gm::new(Mesh::new(context, &mesh)?, material),
            pixel0,
            pixel1,
            thickness,
        };
        line.update();
        Ok(line)
    }

    /// Get one of the end points of the line.
    pub fn end_point0(&self) -> Vec2 {
        self.pixel0
    }

    /// Get one of the end points of the line.
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

    /// Set the line thickness.
    pub fn set_thickness(&mut self, thickness: f32) {
        self.thickness = thickness;
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
                * Mat3::from_nonuniform_scale(length, self.thickness),
        );
    }
}

impl<M: Material> Geometry2D for Line<M> {
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

impl<M: Material> Object2D for Line<M> {
    fn render(&self, viewport: Viewport) -> ThreeDResult<()> {
        self.context
            .camera2d(viewport, |camera2d| self.model.render(camera2d, &[]))
    }

    fn is_transparent(&self) -> bool {
        self.model.is_transparent()
    }
}
