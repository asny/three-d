use crate::renderer::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
#[derive(Clone)]
pub struct Axes {
    model: Model,
    aabb: AxisAlignedBoundingBox,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> Result<Self> {
        let mut mesh = CPUMesh::arrow(0.9, 0.6, 16);
        mesh.transform(&Mat4::from_nonuniform_scale(length, radius, radius));
        let model = Model::new(context, &mesh)?;
        let mut aabb = *model.aabb();
        let mut aabb2 = aabb.clone();
        aabb2.transform(&Mat4::from_angle_z(degrees(90.0)));
        aabb.expand_with_aabb(&aabb2);
        let mut aabb3 = aabb.clone();
        aabb3.transform(&Mat4::from_angle_y(degrees(-90.0)));
        aabb.expand_with_aabb(&aabb3);
        Ok(Self { model, aabb })
    }
}

impl Shadable for Axes {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        let mut model = self.model.clone();
        model.render_forward(material, camera, lights)?;
        model.set_transformation(Mat4::from_angle_z(degrees(90.0)));
        model.render_forward(material, camera, lights)?;
        model.set_transformation(Mat4::from_angle_y(degrees(-90.0)));
        model.render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        let mut model = self.model.clone();
        model.render_deferred(material, camera, viewport)?;
        model.set_transformation(Mat4::from_angle_z(degrees(90.0)));
        model.render_deferred(material, camera, viewport)?;
        model.set_transformation(Mat4::from_angle_y(degrees(-90.0)));
        model.render_deferred(material, camera, viewport)
    }
}

impl Shadable for &Axes {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        (*self).render_forward(material, camera, lights)
    }
    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

impl Drawable for Axes {
    fn render(&self, camera: &Camera, _lights: &Lights) -> Result<()> {
        let mut model = self.model.clone();
        model.render_with_color(Color::RED, camera)?;
        model.set_transformation(Mat4::from_angle_z(degrees(90.0)));
        model.render_with_color(Color::GREEN, camera)?;
        model.set_transformation(Mat4::from_angle_y(degrees(-90.0)));
        model.render_with_color(Color::BLUE, camera)?;
        Ok(())
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl Drawable for &Axes {
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}

impl Geometry for Axes {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        &self.aabb
    }
}
impl Geometry for &Axes {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

impl Object for Axes {}
impl Object for &Axes {}
