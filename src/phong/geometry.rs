use crate::camera::*;
use crate::core::*;
use crate::math::*;
use crate::Geometry;

///
/// Used for [deferred Phong rendering](crate::PhongDeferredPipeline).
/// Implemented by [PhongMesh](crate::PhongMesh) and [PhongInstancedMesh](crate::PhongInstancedMesh).
///
pub trait PhongGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the mesh, ie. the first part of a [deferred render pass](crate::PhongDeferredPipeline::geometry_pass).
    ///
    fn geometry_pass(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error>;
}
