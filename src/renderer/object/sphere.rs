use crate::renderer::*;

#[derive(Clone)]
pub struct Sphere {
    model: Model,
}

impl Sphere {
    pub fn new(context: &Context, center: Vec3, radius: f32) -> ThreeDResult<Self> {
        let mut mesh = CPUMesh::sphere(8);
        mesh.transform(&(Mat4::from_translation(center) * Mat4::from_scale(radius)));
        let model = Model::new(context, &mesh)?;
        Ok(Self { model })
    }
}

impl Shadable for Sphere {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        self.model.render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.model.render_deferred(material, camera, viewport)
    }
}

impl Shadable for &Sphere {
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

impl Geometry for Sphere {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        self.model.aabb()
    }
}
impl Geometry for &Sphere {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

impl Object for Sphere {
    fn render(&self, camera: &Camera, _lights: &Lights) -> ThreeDResult<()> {
        self.model.render_forward(
            &ColorMaterial {
                color: Color::WHITE,
                ..Default::default()
            },
            camera,
            &Lights::default(),
        )
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
impl Object for &Sphere {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        (*self).render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}
