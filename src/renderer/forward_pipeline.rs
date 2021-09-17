use crate::renderer::*;

///
/// Forward render pipeline which can render objects implementing the [Object] trait with materials and lighting.
/// Supports different types of lighting models by changing the [ForwardPipeline::lighting_model] field.
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
    /// Render the geometries with the given surface materials and the given set of lights.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    pub fn light_pass(
        &self,
        camera: &Camera,
        geometries: &[(&dyn Object, &dyn Paint)],
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        for (geometry, material) in geometries {
            if camera.in_frustum(&geometry.aabb()) {
                geometry.render(
                    *material,
                    camera,
                    ambient_light,
                    directional_lights,
                    spot_lights,
                    point_lights,
                )?;
            }
        }

        Ok(())
    }

    pub fn depth_pass(&self, camera: &Camera, objects: &[&dyn Object]) -> Result<()> {
        for object in objects {
            if camera.in_frustum(&object.aabb()) {
                object.render(&DepthMaterial::default(), camera, None, &[], &[], &[])?;
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
