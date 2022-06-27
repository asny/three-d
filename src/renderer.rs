//!
//! High-level features for easy rendering of different types of objects with different types of shading.
//! Can be combined seamlessly with the mid-level features in the [core](crate::core) module as well as functionality in the [context](crate::context) module.
//!
//! This module contains four main traits
//! - [Geometry] - a geometric representation in 3D space
//! - [Material] - a material that can be applied to a geometry
//! - [Object] - an object in 3D space which has both geometry and material information (use the [Gm] struct to combine any [Material] and [Geometry] into an object)
//! - [Light] - a light that shines onto objects in the scene (some materials are affected by lights, others are not)
//!
//! Common implementations of these traits are found in their respective modules but it is also possible to do a custom implementation by implementing one of the four traits.
//!
//! There are several ways to render something.
//! Objects can be rendered directly using [Object::render] or used in a render call, for example [RenderTarget::render].
//! Geometries can be rendered with a given material using [Geometry::render_with_material] or combined into an object using the [Gm] struct and again used in a render call.
//!

pub use crate::core::*;

use thiserror::Error;
///
/// Error in the [renderer](crate::renderer) module.
///
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum RendererError {
    #[error("{0} buffer length must be {1}, actual length is {2}")]
    InvalidBufferLength(String, usize, usize),
    #[error("the material {0} is required by the geometry {1} but could not be found")]
    MissingMaterial(String, String),
}

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

impl<'a> DepthTarget<'a> {
    ///
    /// Render the objects using the given camera and lights into this depth target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(&self, camera: &Camera, objects: &[&dyn Object], lights: &[&dyn Light]) -> &Self {
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
    ) -> &Self {
        self.as_render_target()
            .render_partially(scissor_box, camera, objects, lights);
        self
    }

    ///
    /// Render the geometries with the given material using the given camera and lights into this depth target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    /// Also, geometries outside the camera frustum are not rendered and the geometries are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        geometries: &[&dyn Geometry],
        lights: &[&dyn Light],
    ) -> &Self {
        self.render_partially_with_material(
            self.scissor_box(),
            material,
            camera,
            geometries,
            lights,
        );
        self
    }

    ///
    /// Render the geometries with the given material using the given camera and lights into the part of this depth target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    /// Also, geometries outside the camera frustum are not rendered and the geometries are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_partially_with_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn Material,
        camera: &Camera,
        geometries: &[&dyn Geometry],
        lights: &[&dyn Light],
    ) -> &Self {
        self.as_render_target().render_partially_with_material(
            scissor_box,
            material,
            camera,
            geometries,
            lights,
        );
        self
    }
}

impl<'a> ColorTarget<'a> {
    ///
    /// Render the objects using the given camera and lights into this color target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(&self, camera: &Camera, objects: &[&dyn Object], lights: &[&dyn Light]) -> &Self {
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
    ) -> &Self {
        self.as_render_target()
            .render_partially(scissor_box, camera, objects, lights);
        self
    }

    ///
    /// Render the geometries with the given material using the given camera and lights into this color target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    /// Also, geometries outside the camera frustum are not rendered and the geometries are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        geometries: &[&dyn Geometry],
        lights: &[&dyn Light],
    ) -> &Self {
        self.render_partially_with_material(
            self.scissor_box(),
            material,
            camera,
            geometries,
            lights,
        );
        self
    }

    ///
    /// Render the geometries with the given material using the given camera and lights into the part of this color target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    /// Also, geometries outside the camera frustum are not rendered and the geometries are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_partially_with_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn Material,
        camera: &Camera,
        geometries: &[&dyn Geometry],
        lights: &[&dyn Light],
    ) -> &Self {
        self.as_render_target().render_partially_with_material(
            scissor_box,
            material,
            camera,
            geometries,
            lights,
        );
        self
    }
}

impl<'a> RenderTarget<'a> {
    ///
    /// Render the objects using the given camera and lights into this render target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(&self, camera: &Camera, objects: &[&dyn Object], lights: &[&dyn Light]) -> &Self {
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
    ) -> &Self {
        #![allow(deprecated)]
        let (deferred_objects, forward_objects): (Vec<_>, Vec<_>) = objects
            .iter()
            .partition(|o| o.material_type() == MaterialType::Deferred);

        // Deferred
        if deferred_objects.len() > 0 {
            // Geometry pass
            let mut geometry_pass_camera = camera.clone();
            let viewport =
                Viewport::new_at_origo(camera.viewport().width, camera.viewport().height);
            geometry_pass_camera.set_viewport(viewport);
            let mut geometry_pass_texture = Texture2DArray::new_empty::<[u8; 4]>(
                &self.context,
                viewport.width,
                viewport.height,
                3,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            let mut geometry_pass_depth_texture = DepthTargetTexture2D::new(
                &self.context,
                viewport.width,
                viewport.height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            );
            RenderTarget::new(
                geometry_pass_texture.as_color_target(&[0, 1, 2], None),
                geometry_pass_depth_texture.as_depth_target(),
            )
            .clear(ClearState::default())
            .write(|| render_pass(&geometry_pass_camera, &deferred_objects, lights));

            // Lighting pass
            self.write_partially(scissor_box, || {
                DeferredPhysicalMaterial::lighting_pass(
                    &self.context,
                    camera,
                    &geometry_pass_texture,
                    &geometry_pass_depth_texture,
                    lights,
                )
            });
        }

        // Forward
        self.write_partially(scissor_box, || {
            render_pass(camera, &forward_objects, lights)
        });
        self
    }

    ///
    /// Render the geometries with the given material using the given camera and lights into this render target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    /// Also, geometries outside the camera frustum are not rendered and the geometries are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        geometries: &[&dyn Geometry],
        lights: &[&dyn Light],
    ) -> &Self {
        self.render_partially_with_material(
            self.scissor_box(),
            material,
            camera,
            geometries,
            lights,
        )
    }

    ///
    /// Render the geometries with the given material using the given camera and lights into the part of this render target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    /// Also, geometries outside the camera frustum are not rendered and the geometries are rendered in the order given by [cmp_render_order].
    ///
    pub fn render_partially_with_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn Material,
        camera: &Camera,
        geometries: &[&dyn Geometry],
        lights: &[&dyn Light],
    ) -> &Self {
        self.write_partially(scissor_box, || {
            for object in geometries.iter().filter(|o| camera.in_frustum(&o.aabb())) {
                object.render_with_material(material, camera, lights);
            }
        });
        self
    }
}

///
/// Render the objects using the given camera and lights.
/// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
/// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
///
/// **Note:**
/// Objects with a [DeferredPhysicalMaterial] applied is not supported.
/// Must be called when a render target is bound, for example in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
/// If you are using one of these targets, it is preferred to use the [RenderTarget::render], [ColorTarget::render] or [DepthTarget::render] methods.
///
#[deprecated = "use RenderTarget::render, ColorTarget::render or DepthTarget::render or render each object by using the Object::render method"]
pub fn render_pass(camera: &Camera, objects: &[&dyn Object], lights: &[&dyn Light]) {
    let mut culled_objects = objects
        .iter()
        .filter(|o| camera.in_frustum(&o.aabb()))
        .collect::<Vec<_>>();
    culled_objects.sort_by(|a, b| cmp_render_order(camera, a, b));
    for object in culled_objects {
        object.render(camera, lights);
    }
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
    if obj0.material_type() == MaterialType::Transparent
        && obj1.material_type() != MaterialType::Transparent
    {
        std::cmp::Ordering::Greater
    } else if obj0.material_type() != MaterialType::Transparent
        && obj1.material_type() == MaterialType::Transparent
    {
        std::cmp::Ordering::Less
    } else {
        let distance_a = camera.position().distance2(obj0.aabb().center());
        let distance_b = camera.position().distance2(obj1.aabb().center());
        if distance_a.is_nan() || distance_b.is_nan() {
            distance_a.is_nan().cmp(&distance_b.is_nan()) // whatever - just save us from panicing on unwrap below
        } else if obj0.material_type() == MaterialType::Transparent {
            distance_b.partial_cmp(&distance_a).unwrap()
        } else {
            distance_a.partial_cmp(&distance_b).unwrap()
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
) -> Option<Vec3> {
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
) -> Option<Vec3> {
    use crate::core::*;
    let viewport = Viewport::new_at_origo(1, 1);
    let up = if direction.dot(vec3(1.0, 0.0, 0.0)).abs() > 0.99 {
        direction.cross(vec3(0.0, 1.0, 0.0))
    } else {
        direction.cross(vec3(1.0, 0.0, 0.0))
    };
    let camera = Camera::new_orthographic(
        viewport,
        position,
        position + direction * max_depth,
        up,
        0.01,
        0.0,
        max_depth,
    );
    let mut texture = Texture2D::new_empty::<f32>(
        context,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    let mut depth_texture = DepthTargetTexture2D::new(
        context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
        DepthFormat::Depth32F,
    );
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
    )
    .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
    .write(|| {
        for geometry in geometries {
            geometry.render_with_material(&depth_material, &camera, &[]);
        }
    })
    .read_color()[0];
    if depth < 1.0 {
        Some(position + direction * depth * max_depth)
    } else {
        None
    }
}
