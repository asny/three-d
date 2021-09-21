use crate::core::*;
use crate::renderer::*;

#[derive(Clone)]
pub struct Model2D {
    model: Model,
    context: Context,
}

impl Model2D {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
        Ok(Self {
            model: Model::new(context, cpu_mesh)?,
            context: context.clone(),
        })
    }

    ///
    /// Set the local to world transformation of this mesh.
    ///
    pub fn set_transformation(&mut self, transformation: Mat3) {
        self.model.set_transformation(Mat4::from_cols(
            vec4(
                transformation.x.x,
                transformation.x.y,
                0.0,
                transformation.x.z,
            ),
            vec4(
                transformation.y.x,
                transformation.y.y,
                0.0,
                transformation.y.z,
            ),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(
                transformation.z.x,
                transformation.z.y,
                0.0,
                transformation.z.z,
            ),
        ));
    }

    #[deprecated = "Use 'render_forward' instead."]
    pub fn render_with_color(&self, color: Color, viewport: Viewport) -> Result<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render_forward(
                &ColorMaterial {
                    color,
                    ..Default::default()
                },
                camera2d,
                None,
                &[],
                &[],
                &[],
            )
        })
    }

    #[deprecated = "Use 'render_forward' instead."]
    pub fn render_with_texture(&self, texture: &Texture2D, viewport: Viewport) -> Result<()> {
        self.context.camera2d(viewport, |camera2d| {
            self.model.render_with_texture(texture, camera2d)
        })
    }
}

impl Object for Model2D {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        self.model.render_forward(
            material,
            camera,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
        )
    }
    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        self.model.render_deferred(material, camera, viewport)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.model.aabb()
    }
}
