use crate::camera::*;
use crate::core::*;
use crate::light::*;
use crate::math::*;

pub trait Geometry {
    ///
    /// Render only the depth into the current depth render target which is useful for shadow maps or depth pre-pass.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error>;

    ///
    /// Render the depth (scaled such that a value of 1 corresponds to max_depth) into the red channel of the current color render target which for example is used for picking.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_depth_to_red(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error>;

    fn aabb(&self) -> Option<AxisAlignedBoundingBox>;
}

///
/// Used for [deferred Phong rendering](crate::PhongDeferredPipeline).
/// Implemented by [PhongMesh](crate::PhongMesh) and [PhongInstancedMesh](crate::PhongInstancedMesh).
///
pub trait ShadedGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the mesh, ie. the first part of a [deferred render pass](crate::PhongDeferredPipeline::geometry_pass).
    ///
    fn geometry_pass(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error>;

    ///
    /// Render the triangle mesh shaded with the given lights based on the Phong shading model.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_with_lighting(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error>;
}
