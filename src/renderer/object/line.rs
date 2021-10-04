use crate::renderer::*;

#[derive(Clone)]
pub struct Line {
    context: Context,
    model: Model,
    pixel0: Vec2,
    pixel1: Vec2,
    width: f32,
}

impl Line {
    pub fn new(context: &Context, pixel0: Vec2, pixel1: Vec2, width: f32) -> Result<Self> {
        let mut mesh = CPUMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))));
        let mut line = Self {
            context: context.clone(),
            model: Model::new(context, &mesh)?,
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
        let rot = Mat4::new(
            c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        self.model.set_transformation(
            Mat4::from_translation(self.pixel0.extend(0.0))
                * rot
                * Mat4::from_nonuniform_scale(length, self.width, 1.0),
        );
    }
}

impl Geometry for Line {
    fn render_depth(&self, _camera: &Camera) -> Result<()> {
        unimplemented!()
    }

    fn render_depth_to_red(&self, _camera: &Camera, _max_depth: f32) -> Result<()> {
        unimplemented!()
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.model.aabb()
    }
}

impl Object2D for Line {
    fn render(
        &self,
        material: &dyn ForwardMaterial,
        viewport: Viewport,
        lights: &[&dyn Light],
    ) -> Result<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render_forward(material, camera2d, lights)
        })
    }
}
