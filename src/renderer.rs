//!
//! High-level features for easy rendering of different types of objects with different types of shading.
//! Can be combined seamlessly with the mid-level features in the `core` module and also with calls in the `context` module as long as the graphics state is reset.
//!

pub use crate::core::{
    math::*, render_states::*, render_target::*, texture::*, Camera, Context, TextureTransform,
    Viewport,
};

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

pub use crate::ThreeDResult;
use thiserror::Error;
///
/// Error in the [renderer](crate::renderer) module.
///
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum RendererError {}

///
/// Render the objects. Also avoids rendering objects outside the camera frustum and render the objects in the order given by [cmp_render_order].
/// Must be called in a render target render function, for example in the callback function of [Screen::write].
///
pub fn render_pass(camera: &Camera, objects: &[impl Object], lights: &Lights) -> ThreeDResult<()> {
    let mut culled_objects = objects
        .iter()
        .filter(|o| camera.in_frustum(&o.aabb()))
        .collect::<Vec<_>>();
    culled_objects.sort_by(|a, b| cmp_render_order(camera, a, b));
    for object in culled_objects {
        object.render(camera, lights)?;
    }
    Ok(())
}

use std::cmp::Ordering;
pub fn cmp_render_order(camera: &Camera, obj0: impl Object, obj1: impl Object) -> Ordering {
    if obj0.is_transparent() == obj1.is_transparent() {
        let distance_a = camera.position().distance2(obj0.aabb().center());
        let distance_b = camera.position().distance2(obj1.aabb().center());
        if obj0.is_transparent() {
            distance_b.partial_cmp(&distance_a).unwrap()
        } else {
            distance_a.partial_cmp(&distance_b).unwrap()
        }
    } else {
        if obj0.is_transparent() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

///
/// Finds the closest intersection between a ray from the given camera in the given pixel coordinate and the given geometries.
/// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
/// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
/// Returns ```None``` if no geometry was hit between the near (`z_near`) and far (`z_far`) plane for this camera.
///
pub fn pick<S: Shadable>(
    context: &Context,
    camera: &Camera,
    pixel: (f32, f32),
    geometries: &[S],
) -> ThreeDResult<Option<Vec3>> {
    let pos = camera.position_at_pixel(pixel);
    let dir = camera.view_direction_at_pixel(pixel);
    ray_intersect(
        context,
        pos + dir * camera.z_near(),
        dir,
        camera.z_far() - camera.z_near(),
        geometries,
    )
}

///
/// Finds the closest intersection between a ray starting at the given position in the given direction and the given geometries.
/// Returns ```None``` if no geometry was hit before the given maximum depth.
///
pub fn ray_intersect<S: Shadable>(
    context: &Context,
    position: Vec3,
    direction: Vec3,
    max_depth: f32,
    geometries: &[S],
) -> ThreeDResult<Option<Vec3>> {
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
    let mut texture = Texture2D::<f32>::new_empty(
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
    let mut depth_texture = DepthTargetTexture2D::new(
        context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        DepthFormat::Depth32F,
    )?;
    let depth_material = DepthMaterial {
        render_states: RenderStates {
            write_mask: WriteMask {
                red: true,
                ..WriteMask::DEPTH
            },
            ..Default::default()
        },
        ..Default::default()
    };
    {
        let render_target = RenderTarget::new(context, &mut texture, &mut depth_texture)?;
        render_target.write(
            ClearState {
                red: Some(1.0),
                depth: Some(1.0),
                ..ClearState::none()
            },
            || {
                for geometry in geometries {
                    geometry.render_with_material(&depth_material, &camera, &Lights::default())?;
                }
                Ok(())
            },
        )?;
    }
    let depth = texture.read(viewport)?[0];
    Ok(if depth < 1.0 {
        Some(position + direction * depth * max_depth)
    } else {
        None
    })
}
