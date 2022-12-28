//!
//! High-level features for easy rendering of different types of objects with different types of shading.
//! Can be combined seamlessly with the mid-level features in the [core](crate::core) module as well as functionality in the [context](crate::context) module.
//!
//! This module contains five main traits
//! - [Geometry] - a geometric representation in 3D space
//! - [Material] - a material that can be applied to a geometry
//! - [PostMaterial] - a material that can be applied to a geometry and rendered after the rest of the scene has been rendered
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

pub mod effect;
pub use effect::*;

pub mod light;
pub use light::*;

pub mod geometry;
pub use geometry::*;

pub mod object;
pub use object::*;

pub mod control;
pub use control::*;

impl DepthTarget<'_> {
    ///
    /// Render the objects using the given camera and lights into this depth target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(
        &self,
        camera: &Camera,
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) -> &Self {
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
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) -> &Self {
        self.as_render_target()
            .render_partially(scissor_box, camera, objects, lights);
        self
    }

    ///
    /// Render the geometries with the given [Material] using the given camera and lights into this depth target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
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
    /// Render the geometries with the given [Material] using the given camera and lights into the part of this depth target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_partially_with_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn Material,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
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

    ///
    /// Render the geometries with the given [PostMaterial] using the given camera and lights into this depth target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> &Self {
        self.render_partially_with_post_material(
            self.scissor_box(),
            material,
            camera,
            geometries,
            lights,
            color_texture,
            depth_texture,
        )
    }

    ///
    /// Render the geometries with the given [PostMaterial] using the given camera and lights into the part of this depth target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_partially_with_post_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn PostMaterial,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> &Self {
        self.as_render_target().render_partially_with_post_material(
            scissor_box,
            material,
            camera,
            geometries,
            lights,
            color_texture,
            depth_texture,
        );
        self
    }
}

impl ColorTarget<'_> {
    ///
    /// Render the objects using the given camera and lights into this color target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(
        &self,
        camera: &Camera,
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) -> &Self {
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
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) -> &Self {
        self.as_render_target()
            .render_partially(scissor_box, camera, objects, lights);
        self
    }

    ///
    /// Render the geometries with the given [Material] using the given camera and lights into this color target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
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
    /// Render the geometries with the given [Material] using the given camera and lights into the part of this color target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_partially_with_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn Material,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
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

    ///
    /// Render the geometries with the given [PostMaterial] using the given camera and lights into this color target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> &Self {
        self.render_partially_with_post_material(
            self.scissor_box(),
            material,
            camera,
            geometries,
            lights,
            color_texture,
            depth_texture,
        )
    }

    ///
    /// Render the geometries with the given [PostMaterial] using the given camera and lights into the part of this color target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_partially_with_post_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn PostMaterial,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> &Self {
        self.as_render_target().render_partially_with_post_material(
            scissor_box,
            material,
            camera,
            geometries,
            lights,
            color_texture,
            depth_texture,
        );
        self
    }
}

impl RenderTarget<'_> {
    ///
    /// Render the objects using the given camera and lights into this render target.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    /// Also, objects outside the camera frustum are not rendered and the objects are rendered in the order given by [cmp_render_order].
    ///
    pub fn render(
        &self,
        camera: &Camera,
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) -> &Self {
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
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) -> &Self {
        let (mut deferred_objects, mut forward_objects): (Vec<_>, Vec<_>) = objects
            .into_iter()
            .filter(|o| camera.in_frustum(&o.aabb()))
            .partition(|o| o.material_type() == MaterialType::Deferred);

        // Deferred
        if deferred_objects.len() > 0 {
            // Geometry pass
            let mut geometry_pass_camera = camera.clone();
            let viewport =
                Viewport::new_at_origo(camera.viewport().width, camera.viewport().height);
            geometry_pass_camera.set_viewport(viewport);
            deferred_objects.sort_by(|a, b| cmp_render_order(&geometry_pass_camera, a, b));
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
            let mut geometry_pass_depth_texture = DepthTexture2D::new::<f32>(
                &self.context,
                viewport.width,
                viewport.height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            );
            let gbuffer_layers = [0, 1, 2];
            RenderTarget::new(
                geometry_pass_texture.as_color_target(&gbuffer_layers, None),
                geometry_pass_depth_texture.as_depth_target(),
            )
            .clear(ClearState::default())
            .write(|| {
                for object in deferred_objects {
                    object.render(&geometry_pass_camera, lights);
                }
            });

            // Lighting pass
            self.write_partially(scissor_box, || {
                DeferredPhysicalMaterial::lighting_pass(
                    &self.context,
                    camera,
                    ColorTexture::Array {
                        texture: &geometry_pass_texture,
                        layers: &gbuffer_layers,
                    },
                    DepthTexture::Single(&geometry_pass_depth_texture),
                    lights,
                )
            });
        }

        // Forward
        forward_objects.sort_by(|a, b| cmp_render_order(camera, a, b));
        self.write_partially(scissor_box, || {
            for object in forward_objects {
                object.render(camera, lights);
            }
        });
        self
    }

    ///
    /// Render the geometries with the given [Material] using the given camera and lights into this render target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
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
    /// Render the geometries with the given [Material] using the given camera and lights into the part of this render target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_partially_with_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn Material,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
    ) -> &Self {
        self.write_partially(scissor_box, || {
            for object in geometries
                .into_iter()
                .filter(|o| camera.in_frustum(&o.aabb()))
            {
                object.render_with_material(material, camera, lights);
            }
        });
        self
    }

    ///
    /// Render the geometries with the given [PostMaterial] using the given camera and lights into this render target.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> &Self {
        self.render_partially_with_post_material(
            self.scissor_box(),
            material,
            camera,
            geometries,
            lights,
            color_texture,
            depth_texture,
        )
    }

    ///
    /// Render the geometries with the given [PostMaterial] using the given camera and lights into the part of this render target defined by the scissor box.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    pub fn render_partially_with_post_material(
        &self,
        scissor_box: ScissorBox,
        material: &dyn PostMaterial,
        camera: &Camera,
        geometries: impl IntoIterator<Item = impl Geometry>,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> &Self {
        self.write_partially(scissor_box, || {
            for object in geometries
                .into_iter()
                .filter(|o| camera.in_frustum(&o.aabb()))
            {
                object.render_with_post_material(
                    material,
                    camera,
                    lights,
                    color_texture,
                    depth_texture,
                );
            }
        });
        self
    }
}

///
/// Returns a camera for viewing 2D content.
///
pub fn camera2d(viewport: Viewport) -> Camera {
    Camera::new_orthographic(
        viewport,
        vec3(
            viewport.width as f32 * 0.5,
            viewport.height as f32 * 0.5,
            -1.0,
        ),
        vec3(
            viewport.width as f32 * 0.5,
            viewport.height as f32 * 0.5,
            0.0,
        ),
        vec3(0.0, -1.0, 0.0),
        viewport.height as f32,
        0.0,
        10.0,
    )
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
/// To convert from control input positions to the pixel (viewport) positions use [`crate::control::control_position_to_viewport_position`].
/// Returns ```None``` if no geometry was hit between the near (`z_near`) and far (`z_far`) plane for this camera.
///
pub fn pick(
    context: &Context,
    camera: &Camera,
    pixel: (f32, f32),
    geometries: impl IntoIterator<Item = impl Geometry>,
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
    geometries: impl IntoIterator<Item = impl Geometry>,
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
    let mut depth_texture = DepthTexture2D::new::<f32>(
        context,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
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
