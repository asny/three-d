use crate::renderer::shading::*;
use crate::renderer::*;

///
/// Forward pipeline using physically based rendering (PBR) and supporting a performance-limited
/// amount of directional, point and spot lights with shadows.
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
    /// Render the [geometries](crate::ShadedGeometry) with the given surface materials and the given set of lights using physically based rendering (PBR).
    /// This function must be called in a render target render function.
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

        Ok(())
    }
}
