use crate::renderer::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
#[derive(Clone)]
pub struct Axes {
    model: Model,
    pub transformation: Mat4,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> Result<Self> {
        let mut mesh = CPUMesh::arrow(0.9, 0.6, 16);
        mesh.transform(&Mat4::from_nonuniform_scale(length, radius, radius));
        Ok(Self {
            model: Model::new(context, &mesh)?,
            transformation: Mat4::identity(),
        })
    }
}

impl Geometry for Axes {
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()> {
        unimplemented!()
    }

    fn render_depth(&self, camera: &Camera) -> Result<()> {
        unimplemented!()
    }

    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()> {
        let mut model = self.model.clone();
        model.set_transformation(self.transformation);
        model.render_forward(material, camera)?;
        model.set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        model.render_forward(material, camera)?;
        model.set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        model.render_forward(material, camera)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE // TODO
    }

    fn transformation(&self) -> &Mat4 {
        &self.transformation
    }
}

impl Object for Axes {
    fn render(&self, camera: &Camera) -> Result<()> {
        let mut model = self.model.clone();
        model.render_with_color(Color::RED, camera)?;
        model.set_transformation(Mat4::from_angle_z(degrees(90.0)));
        model.render_with_color(Color::GREEN, camera)?;
        model.set_transformation(Mat4::from_angle_y(degrees(-90.0)));
        model.render_with_color(Color::BLUE, camera)?;
        Ok(())
    }
}
