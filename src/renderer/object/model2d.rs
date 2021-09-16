use crate::core::*;
use crate::renderer::*;

#[derive(Clone)]
pub struct Model2D {
    model: Model,
    context: Context,
}

impl Model2D {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
        let model = Model::new(context, cpu_mesh)?;
        unsafe {
            COUNT += 1;
        }
        Ok(Self {
            model,
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

    #[deprecated = "Use 'render' instead."]
    pub fn render_with_color(&self, color: Color, viewport: Viewport) -> Result<()> {
        self.model.render(
            &ColorMaterial { color },
            self.camera2d(viewport)?,
            None,
            &[],
            &[],
            &[],
        )
    }

    #[deprecated = "Use 'render' instead."]
    pub fn render_with_texture(&self, texture: &Texture2D, viewport: Viewport) -> Result<()> {
        self.model
            .render_with_texture(texture, self.camera2d(viewport)?)
    }

    pub fn camera2d(&self, viewport: Viewport) -> Result<&Camera> {
        unsafe {
            if CAMERA2D.is_none() {
                CAMERA2D = Some(Camera::new_orthographic(
                    &self.context,
                    viewport,
                    vec3(0.0, 0.0, -1.0),
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                    1.0,
                    0.0,
                    10.0,
                )?);
            }
            let camera = CAMERA2D.as_mut().unwrap();
            camera.set_viewport(viewport)?;
            camera.set_orthographic_projection(viewport.height as f32, 0.0, 10.0)?;
            camera.set_view(
                vec3(
                    viewport.width as f32 * 0.5,
                    viewport.height as f32 * 0.5,
                    -1.0,
                ),
                vec3(
                    viewport.width as f32 * 0.5,
                    viewport.height as f32 * 0.5,
                    0.0,
                ),
                vec3(0.0, -1.0, 0.0),
            )?;
            Ok(camera)
        }
    }
}

impl Geometry for Model2D {
    fn render_depth(&self, camera: &Camera) -> Result<()> {
        self.model.render_depth(camera)
    }

    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()> {
        self.model.render_depth_to_red(camera, max_depth)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.model.aabb()
    }
}

impl Object for Model2D {
    fn render(
        &self,
        material: &dyn Paint,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        self.model.render(
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
}

impl Drop for Model2D {
    fn drop(&mut self) {
        unsafe {
            COUNT -= 1;
            if COUNT == 0 {
                CAMERA2D = None;
            }
        }
    }
}

static mut COUNT: u32 = 0;
static mut CAMERA2D: Option<Camera> = None;
