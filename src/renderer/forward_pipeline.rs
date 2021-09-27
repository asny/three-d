use crate::renderer::*;

///
/// Forward render pipeline which can render objects (implementing the [Object] trait) with materials (implementing the [ForwardMaterial] trait) and lighting.
/// Forward rendering directly draws to the given render target (for example the screen) and is therefore the same as calling [Object::render] directly.
///
pub struct ForwardPipeline {
    context: Context,
}

impl ForwardPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> Result<Self> {
        Ok(Self {
            context: context.clone(),
        })
    }

    ///
    /// Render the objects with the given surface materials and the given set of lights.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    #[deprecated = "Use render_pass instead"]
    pub fn light_pass(
        &self,
        camera: &Camera,
        objects: &[(&dyn Object, &dyn ForwardMaterial)],
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        self.render_pass(
            camera,
            objects,
            &Lights {
                ambient_light: ambient_light.map(|l| l.clone()),
                directional_lights: directional_lights.iter().map(|l| (*l).clone()).collect(),
                spot_lights: spot_lights.iter().map(|l| (*l).clone()).collect(),
                point_lights: point_lights.iter().map(|l| (*l).clone()).collect(),
            },
        )
    }

    pub fn render_pass(
        &self,
        camera: &Camera,
        objects: &[(&dyn Object, &dyn ForwardMaterial)],
        lights: &Lights,
    ) -> Result<()> {
        for (object, material) in objects {
            if in_frustum(camera, object) {
                object.render_forward(*material, camera, lights)?;
            }
        }

        Ok(())
    }

    pub fn depth_pass(&self, camera: &Camera, objects: &[&dyn Object]) -> Result<()> {
        for object in objects {
            if in_frustum(camera, object) {
                object.render_forward(&DepthMaterial::default(), camera, &Lights::NONE)?;
            }
        }
        Ok(())
    }

    pub fn depth_pass_texture(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
    ) -> Result<DepthTargetTexture2D> {
        let depth_texture = DepthTargetTexture2D::new(
            &self.context,
            camera.viewport().width,
            camera.viewport().height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        depth_texture.write(Some(1.0), || self.depth_pass(&camera, objects))?;
        Ok(depth_texture)
    }
}
