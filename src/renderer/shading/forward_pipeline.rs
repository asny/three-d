use crate::renderer::*;

///
/// Forward render pipeline which can render objects implementing the [ShadedGeometry] trait with materials and lighting.
/// Supports different types of lighting models by changing the [ForwardPipeline::lighting_model] field.
/// Forward rendering directly draws to the given render target (for example the screen) and is therefore the same as calling [ShadedGeometry::render_with_lighting] directly.
///
pub struct ForwardPipeline {
    _context: Context,
    pub lighting_model: LightingModel,
}

impl ForwardPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> Result<Self, Error> {
        Ok(Self {
            _context: context.clone(),
            lighting_model: LightingModel::Blinn,
        })
    }

    ///
    /// Render the geometries with the given surface materials and the given set of lights.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    pub fn light_pass(
        &self,
        camera: &Camera,
        geometries: &[(&dyn ShadedGeometry, &Material)],
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        for (geometry, material) in geometries {
            if camera.in_frustum(&geometry.aabb()) {
                geometry.render_with_lighting(
                    camera,
                    material,
                    self.lighting_model,
                    ambient_light,
                    directional_lights,
                    spot_lights,
                    point_lights,
                )?;
            }
        }

        Ok(())
    }
}
