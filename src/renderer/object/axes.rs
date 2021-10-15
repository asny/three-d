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
    pub fn new(context: &Context, radius: f32, length: f32) -> ThreeDResult<Self> {
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
    ) -> ThreeDResult<()> {
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
    ) -> ThreeDResult<()> {
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
    ) -> ThreeDResult<()> {
        (*self).render_forward(material, camera, lights)
    }
    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (*self).render_deferred(material, camera, viewport)
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

impl Object for Axes {
    fn render(&self, camera: &Camera, _lights: &Lights) -> ThreeDResult<()> {
        let mut model = self.model.clone();
        model.render_forward(
            &ColorMaterial {
                color: Color::RED,
                ..Default::default()
            },
            camera,
            &Lights::default(),
        )?;
        model.set_transformation(Mat4::from_angle_z(degrees(90.0)));
        model.render_forward(
            &ColorMaterial {
                color: Color::GREEN,
                ..Default::default()
            },
            camera,
            &Lights::default(),
        )?;
        model.set_transformation(Mat4::from_angle_y(degrees(-90.0)));
        model.render_forward(
            &ColorMaterial {
                color: Color::BLUE,
                ..Default::default()
            },
            camera,
            &Lights::default(),
        )?;
        Ok(())
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
impl Object for &Axes {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        (*self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}
