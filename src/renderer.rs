//!
//! High-level features for easy rendering of different types of objects with different types of shading.
//! Can be combined seamlessly with the mid-level features in the `core` module and also with calls in the `context` module as long as the graphics state is reset.
//!

pub use crate::core::{math::*, render_target::*, texture::*, Camera, Context, Viewport};

pub mod material;
pub use material::*;

mod forward_pipeline;
#[doc(inline)]
pub use forward_pipeline::*;

mod deferred_pipeline;
#[doc(inline)]
pub use deferred_pipeline::*;

pub mod effect;
pub use effect::*;

pub mod light;
pub use light::*;

pub mod object;
pub use object::*;

pub(crate) use crate::Result;
use thiserror::Error;
///
/// Error in the [renderer](crate::renderer) module.
///
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum RendererError {}

impl crate::core::Camera {
    ///
    /// Finds the closest intersection between a ray from this camera in the given pixel coordinate and the given geometries.
    /// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    /// Returns ```None``` if no geometry was hit before the given maximum depth.
    ///
    pub fn pick(
        &self,
        pixel: (f32, f32),
        max_depth: f32,
        objects: &[&dyn Object],
    ) -> Result<Option<Vec3>> {
        let pos = self.position_at_pixel(pixel);
        let dir = self.view_direction_at_pixel(pixel);
        ray_intersect(&self.context, pos, dir, max_depth, objects)
    }
}

pub fn in_frustum(camera: &Camera, geometry: &dyn Geometry) -> bool {
    camera.in_frustum(&geometry.aabb())
}

pub fn ray_intersect(
    context: &Context,
    position: Vec3,
    direction: Vec3,
    max_depth: f32,
    geometries: &[&dyn Object],
) -> Result<Option<Vec3>> {
    use crate::core::*;
    let viewport = Viewport::new_at_origo(1, 1);
    let up = if direction.dot(vec3(1.0, 0.0, 0.0)).abs() > 0.99 {
        direction.cross(vec3(0.0, 1.0, 0.0))
    } else {
        direction.cross(vec3(1.0, 0.0, 0.0))
    };
    let camera = Camera::new_orthographic(
        context,
        viewport,
        position,
        position + direction * max_depth,
        up,
        0.01,
        0.0,
        max_depth,
    )?;
    let texture = ColorTargetTexture2D::<f32>::new(
        context,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        Format::RGBA,
    )?;
    let depth_texture = DepthTargetTexture2D::new(
        context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        DepthFormat::Depth32F,
    )?;
    let render_target = RenderTarget::new(context, &texture, &depth_texture)?;
    render_target.write(
        ClearState {
            red: Some(1.0),
            depth: Some(1.0),
            ..ClearState::none()
        },
        || {
            for geometry in geometries {
                if in_frustum(&camera, geometry) {
                    geometry.render_forward(&PickMaterial::default(), &camera, &[])?;
                }
            }
            Ok(())
        },
    )?;
    let depth = texture.read(viewport)?[0];
    Ok(if depth < 1.0 {
        Some(position + direction * depth * max_depth)
    } else {
        None
    })
}
