//!
//! Adds functionality for rendering objects with lighting including support for physically based rendering (PBR).
//! To render an object implementing the [ShadedGeometry] trait, either call the [ShadedGeometry::render_with_lighting] method or use the [ForwardPipeline] or [DeferredPipeline].
//!

mod forward_pipeline;
#[doc(inline)]
pub use forward_pipeline::*;

mod deferred_pipeline;
#[doc(inline)]
pub use deferred_pipeline::*;

use crate::core::*;
use crate::renderer::*;

#[deprecated]
pub trait ShadedGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    #[deprecated = "Use 'render_deferred' instead"]
    fn geometry_pass(&self, camera: &Camera, viewport: Viewport, material: &Material)
        -> Result<()>;

    ///
    /// Render the object shaded with the given lights using physically based rendering (PBR).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render transparent if the material contain an albedo color with alpha value below 255 or if the albedo texture contain an alpha channel (ie. the format is [Format::RGBA]),
    /// you only need to render the model after all solid models.
    ///
    #[deprecated = "Use 'render' instead"]
    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &Material,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()>;
}

