//!
//! High-level features for easy rendering of different types of objects with different types of shading.
//! Can be combined seamlessly with the mid-level features in the `core` module as well as functionality in the `context` module.
//!

pub use crate::core::{
    render_states::*, render_target::*, texture::*, Camera, Context, ScissorBox, Viewport,
};

pub use three_d_io::prelude::*;

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

pub mod geometry;
pub use geometry::*;

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

impl<'a> DepthTarget<'a> {
    ///
    /// Render the objects using the given camera and lights into this depth target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<&Self> {
        self.render_partially(self.scissor_box(), camera, objects, lights)
    }

    ///
    /// Render the objects using the given camera and lights into the part of this depth target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_partially(
        &self,
        scissor_box: ScissorBox,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<&Self> {
        self.write_partially(scissor_box, || render_pass(camera, objects, lights))?;
        Ok(self)
    }
}

impl<'a> ColorTarget<'a> {
    ///
    /// Render the objects using the given camera and lights into this color target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<&Self> {
        self.render_partially(self.scissor_box(), camera, objects, lights)
    }

    ///
    /// Render the objects using the given camera and lights into the part of this color target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_partially(
        &self,
        scissor_box: ScissorBox,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<&Self> {
        self.write_partially(scissor_box, || render_pass(camera, objects, lights))?;
        Ok(self)
    }
}

impl<'a> RenderTarget<'a> {
    ///
    /// Render the objects using the given camera and lights into this render target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<&Self> {
        self.render_partially(self.scissor_box(), camera, objects, lights)
    }

    ///
    /// Render the objects using the given camera and lights into the part of this render target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_partially(
        &self,
        scissor_box: ScissorBox,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<&Self> {
        self.write_partially(scissor_box, || render_pass(camera, objects, lights))?;
        Ok(self)
    }
}

///
/// Render the objects using the given camera and lights. If the objects materials doesn't require lighting, you can use `&[]` as the `lights` argument.
/// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
/// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
///
pub fn render_pass(
    camera: &Camera,
    objects: &[&dyn Object],
    lights: &[&dyn Light],
) -> ThreeDResult<()> {
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

///
/// Compare function for sorting objects based on distance from the camera.
/// The order is opaque objects from nearest to farthest away from the camera,
/// then transparent objects from farthest away to closest to the camera.
///
pub fn cmp_render_order(
    camera: &Camera,
    obj0: impl Object,
    obj1: impl Object,
) -> std::cmp::Ordering {
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
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

///
/// Finds the closest intersection between a ray from the given camera in the given pixel coordinate and the given geometries.
/// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the bottom left corner of the viewport
/// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the top right corner.
/// Returns ```None``` if no geometry was hit between the near (`z_near`) and far (`z_far`) plane for this camera.
///
pub fn pick(
    context: &Context,
    camera: &Camera,
    pixel: (f32, f32),
    geometries: &[&dyn Geometry],
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
pub fn ray_intersect(
    context: &Context,
    position: Vec3,
    direction: Vec3,
    max_depth: f32,
    geometries: &[&dyn Geometry],
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
    let mut texture = Texture2D::new_empty::<f32>(
        context,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
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
    let depth = RenderTarget::new(
        texture.as_color_target(None),
        depth_texture.as_depth_target(),
    )?
    .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))?
    .write(|| {
        for geometry in geometries {
            geometry.render_with_material(&depth_material, &camera, &[])?;
        }
        Ok(())
    })?
    .read_color()?[0];
    Ok(if depth < 1.0 {
        Some(position + direction * depth * max_depth)
    } else {
        None
    })
}
