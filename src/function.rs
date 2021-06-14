//!
//! Additional functionality not related to any specific type.
//!

use crate::camera::*;
use crate::core::*;
use crate::math::*;
use crate::object::*;

pub fn ray_intersect(
    context: &Context,
    position: Vec3,
    direction: Vec3,
    max_depth: f32,
    geometries: &[&dyn Geometry],
) -> Result<Option<Vec3>, Error> {
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

    let render_states = RenderStates {
        write_mask: WriteMask {
            red: true,
            depth: true,
            ..WriteMask::NONE
        },
        depth_test: DepthTestType::Less,
        ..Default::default()
    };
    render_target.write(
        ClearState {
            red: Some(1.0),
            depth: Some(1.0),
            ..ClearState::none()
        },
        || {
            for geometry in geometries {
                if geometry
                    .aabb()
                    .map(|aabb| camera.in_frustum(&aabb))
                    .unwrap_or(true)
                {
                    geometry.render_depth_to_red(render_states, &camera, max_depth)?;
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
