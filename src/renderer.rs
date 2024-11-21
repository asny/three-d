//!
//! High-level features for easy rendering of different types of objects with different types of shading.
//! Can be combined seamlessly with the mid-level features in the [core](crate::core) module as well as functionality in the [context](crate::context) module.
//!
//! This module contains five main traits
//! - [Geometry] - a geometric representation in 3D space
//! - [Material] - a material that can be applied to a geometry or the screen
//! - [Effect] - an effect that can be applied to a geometry or the screen after the rest of the scene has been rendered
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
    #[cfg(feature = "text")]
    #[error("Failed to find font with index {0} in the given font collection")]
    MissingFont(u32),
}

mod shader_ids;
pub use shader_ids::*;

mod camera;
pub use camera::*;

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

#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text")]
pub use text::*;

macro_rules! impl_render_target_extensions_body {
    () => {
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
                .filter(|o| camera.in_frustum(o.aabb()))
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
                .write::<RendererError>(|| {
                    for object in deferred_objects {
                        object.render(&geometry_pass_camera, lights);
                    }
                    Ok(())
                })
                .unwrap();

                // Lighting pass
                self.apply_screen_effect_partially(
                    scissor_box,
                    &lighting_pass::LightingPassEffect {},
                    camera,
                    lights,
                    Some(ColorTexture::Array {
                        texture: &geometry_pass_texture,
                        layers: &gbuffer_layers,
                    }),
                    Some(DepthTexture::Single(&geometry_pass_depth_texture)),
                );
            }

            // Forward
            forward_objects.sort_by(|a, b| cmp_render_order(camera, a, b));
            self.write_partially::<RendererError>(scissor_box, || {
                for object in forward_objects {
                    object.render(camera, lights);
                }
                Ok(())
            })
            .unwrap();
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
            self.write_partially::<RendererError>(scissor_box, || {
                for geometry in geometries
                    .into_iter()
                    .filter(|o| camera.in_frustum(o.aabb()))
                {
                    render_with_material(&self.context, camera, geometry, material, lights);
                }
                Ok(())
            })
            .unwrap();
            self
        }

        ///
        /// Render the geometries with the given [Effect] using the given camera and lights into this render target.
        /// Use an empty array for the `lights` argument, if the effect does not require lights to be rendered.
        ///
        pub fn render_with_effect(
            &self,
            effect: &dyn Effect,
            camera: &Camera,
            geometries: impl IntoIterator<Item = impl Geometry>,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) -> &Self {
            self.render_partially_with_effect(
                self.scissor_box(),
                effect,
                camera,
                geometries,
                lights,
                color_texture,
                depth_texture,
            )
        }

        ///
        /// Render the geometries with the given [Effect] using the given camera and lights into the part of this render target defined by the scissor box.
        /// Use an empty array for the `lights` argument, if the effect does not require lights to be rendered.
        ///
        pub fn render_partially_with_effect(
            &self,
            scissor_box: ScissorBox,
            effect: &dyn Effect,
            camera: &Camera,
            geometries: impl IntoIterator<Item = impl Geometry>,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) -> &Self {
            self.write_partially::<RendererError>(scissor_box, || {
                for geometry in geometries
                    .into_iter()
                    .filter(|o| camera.in_frustum(o.aabb()))
                {
                    render_with_effect(
                        &self.context,
                        camera,
                        geometry,
                        effect,
                        lights,
                        color_texture,
                        depth_texture,
                    );
                }
                Ok(())
            })
            .unwrap();
            self
        }

        ///
        /// Apply the given [Material] to this render target.
        /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
        ///
        pub fn apply_screen_material(
            &self,
            material: &dyn Material,
            camera: &Camera,
            lights: &[&dyn Light],
        ) -> &Self {
            self.apply_screen_material_partially(self.scissor_box(), material, camera, lights)
        }

        ///
        /// Apply the given [Material] to the part of this render target defined by the scissor box.
        /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
        ///
        pub fn apply_screen_material_partially(
            &self,
            scissor_box: ScissorBox,
            material: &dyn Material,
            camera: &Camera,
            lights: &[&dyn Light],
        ) -> &Self {
            self.write_partially::<RendererError>(scissor_box, || {
                apply_screen_material(&self.context, material, camera, lights);
                Ok(())
            })
            .unwrap();
            self
        }

        ///
        /// Apply the given [Effect] to this render target.
        /// Use an empty array for the `lights` argument, if the effect does not require lights to be rendered.
        ///
        pub fn apply_screen_effect(
            &self,
            effect: &dyn Effect,
            camera: &Camera,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) -> &Self {
            self.apply_screen_effect_partially(
                self.scissor_box(),
                effect,
                camera,
                lights,
                color_texture,
                depth_texture,
            )
        }

        ///
        /// Apply the given [Effect] to the part of this render target defined by the scissor box.
        /// Use an empty array for the `lights` argument, if the effect does not require lights to be rendered.
        ///
        pub fn apply_screen_effect_partially(
            &self,
            scissor_box: ScissorBox,
            effect: &dyn Effect,
            camera: &Camera,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) -> &Self {
            self.write_partially::<RendererError>(scissor_box, || {
                apply_screen_effect(
                    &self.context,
                    effect,
                    camera,
                    lights,
                    color_texture,
                    depth_texture,
                );
                Ok(())
            })
            .unwrap();
            self
        }
    };
}

macro_rules! impl_render_target_extensions {
    // 2 generic arguments with bounds
    ($name:ident < $a:ident : $ta:tt , $b:ident : $tb:tt >) => {
        impl<$a: $ta, $b: $tb> $name<$a, $b> {
            impl_render_target_extensions_body!();
        }
    };
    // 1 generic argument with bound
    ($name:ident < $a:ident : $ta:tt >) => {
        impl<$a: $ta> $name<$a> {
            impl_render_target_extensions_body!();
        }
    };
    // 1 liftetime argument
    ($name:ident < $lt:lifetime >) => {
        impl<$lt> $name<$lt> {
            impl_render_target_extensions_body!();
        }
    };
    // without any arguments
    ($name:ty) => {
        impl $name {
            impl_render_target_extensions_body!();
        }
    };
}

impl_render_target_extensions!(RenderTarget<'a>);
impl_render_target_extensions!(ColorTarget<'a>);
impl_render_target_extensions!(DepthTarget<'a>);
impl_render_target_extensions!(
    RenderTargetMultisample<C: TextureDataType, D: DepthTextureDataType>
);
impl_render_target_extensions!(ColorTargetMultisample<C: TextureDataType>);
impl_render_target_extensions!(DepthTargetMultisample<D: DepthTextureDataType>);

///
/// Combines shader ID components together into a single ID vector, to be used as a key in shader caching.
///
fn combine_ids(
    geometry: GeometryId,
    effect_material: EffectMaterialId,
    lights: impl Iterator<Item = LightId>,
) -> Vec<u8> {
    let mut id = geometry.0.to_le_bytes().to_vec();
    id.extend(effect_material.0.to_le_bytes());
    id.extend(lights.map(|l| l.0));
    return id;
}

///
/// Render the given [Geometry] with the given [Material].
/// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
/// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
///
pub fn render_with_material(
    context: &Context,
    camera: &Camera,
    geometry: impl Geometry,
    material: impl Material,
    lights: &[&dyn Light],
) {
    let fragment_attributes = material.fragment_attributes();
    let id = combine_ids(
        geometry.id(fragment_attributes),
        material.id(),
        lights.iter().map(|l| l.id()),
    );

    let mut programs = context.programs.write().unwrap();
    let program = programs.entry(id).or_insert_with(|| {
        match Program::from_source(
            context,
            &geometry.vertex_shader_source(fragment_attributes),
            &material.fragment_shader_source(lights),
        ) {
            Ok(program) => program,
            Err(err) => panic!("{}", err.to_string()),
        }
    });
    material.use_uniforms(program, camera, lights);
    geometry.draw(
        camera,
        program,
        material.render_states(),
        fragment_attributes,
    );
}

///
/// Render the given [Geometry] with the given [Effect].
/// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
/// Use an empty array for the `lights` argument, if the effect does not require lights to be rendered.
///
pub fn render_with_effect(
    context: &Context,
    camera: &Camera,
    geometry: impl Geometry,
    effect: impl Effect,
    lights: &[&dyn Light],
    color_texture: Option<ColorTexture>,
    depth_texture: Option<DepthTexture>,
) {
    let fragment_attributes = effect.fragment_attributes();
    let id = combine_ids(
        geometry.id(fragment_attributes),
        effect.id(color_texture, depth_texture),
        lights.iter().map(|l| l.id()),
    );

    let mut programs = context.programs.write().unwrap();
    let program = programs.entry(id).or_insert_with(|| {
        match Program::from_source(
            context,
            &geometry.vertex_shader_source(fragment_attributes),
            &effect.fragment_shader_source(lights, color_texture, depth_texture),
        ) {
            Ok(program) => program,
            Err(err) => panic!("{}", err.to_string()),
        }
    });
    effect.use_uniforms(program, camera, lights, color_texture, depth_texture);
    geometry.draw(camera, program, effect.render_states(), fragment_attributes);
}

///
/// Apply the given [Material] to the entire sceen.
/// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
/// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
///
pub fn apply_screen_material(
    context: &Context,
    material: impl Material,
    camera: &Camera,
    lights: &[&dyn Light],
) {
    let fragment_attributes = material.fragment_attributes();
    if fragment_attributes.normal || fragment_attributes.position || fragment_attributes.tangents {
        panic!("Not possible to use the given material to render full screen, the full screen geometry only provides uv coordinates and color");
    }
    let id = combine_ids(
        GeometryId::Screen,
        material.id(),
        lights.iter().map(|l| l.id()),
    );

    let mut programs = context.programs.write().unwrap();
    let program = programs.entry(id).or_insert_with(|| {
        match Program::from_source(
            context,
            full_screen_vertex_shader_source(),
            &material.fragment_shader_source(lights),
        ) {
            Ok(program) => program,
            Err(err) => panic!("{}", err.to_string()),
        }
    });
    material.use_uniforms(program, camera, lights);
    full_screen_draw(
        context,
        program,
        material.render_states(),
        camera.viewport(),
    );
}

///
/// Apply the given [Effect] to the entire sceen.
/// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
/// Use an empty array for the `lights` argument, if the effect does not require lights to be rendered.
///
pub fn apply_screen_effect(
    context: &Context,
    effect: impl Effect,
    camera: &Camera,
    lights: &[&dyn Light],
    color_texture: Option<ColorTexture>,
    depth_texture: Option<DepthTexture>,
) {
    let fragment_attributes = effect.fragment_attributes();
    if fragment_attributes.normal || fragment_attributes.position || fragment_attributes.tangents {
        panic!("Not possible to use the given effect to render full screen, the full screen geometry only provides uv coordinates and color");
    }
    let id = combine_ids(
        GeometryId::Screen,
        effect.id(color_texture, depth_texture),
        lights.iter().map(|l| l.id()),
    );

    let mut programs = context.programs.write().unwrap();
    let program = programs.entry(id).or_insert_with(|| {
        match Program::from_source(
            context,
            full_screen_vertex_shader_source(),
            &effect.fragment_shader_source(lights, color_texture, depth_texture),
        ) {
            Ok(program) => program,
            Err(err) => panic!("{}", err.to_string()),
        }
    });
    effect.use_uniforms(program, camera, lights, color_texture, depth_texture);
    full_screen_draw(context, program, effect.render_states(), camera.viewport());
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
    pixel: impl Into<PhysicalPoint> + Copy,
    geometries: impl IntoIterator<Item = impl Geometry>,
) -> Option<IntersectionResult> {
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

/// Result from an intersection test
#[derive(Debug, Clone, Copy)]
pub struct IntersectionResult {
    /// The position of the intersection.
    pub position: Vec3,
    /// The index of the intersected geometry in the list of geometries.
    pub geometry_id: u32,
    /// The index of the intersected instance in the list of instances, ie. [gl_InstanceID](https://registry.khronos.org/OpenGL-Refpages/gl4/html/gl_InstanceID.xhtml).
    /// This is 0 if the intersection did not hit an instanced geometry.
    pub instance_id: u32,
    /// The id of the primitive in the intersected geometry, ie. [gl_PrimitiveId](https://registry.khronos.org/OpenGL-Refpages/gl4/html/gl_PrimitiveID.xhtml).
    pub primitive_id: u32,
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
) -> Option<IntersectionResult> {
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
    let mut texture = Texture2D::new_empty::<[f32; 4]>(
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
    let mut material = IntersectionMaterial {
        ..Default::default()
    };
    let result = RenderTarget::new(
        texture.as_color_target(None),
        depth_texture.as_depth_target(),
    )
    .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
    .write::<RendererError>(|| {
        for (id, geometry) in geometries.into_iter().enumerate() {
            material.geometry_id = id as u32;
            render_with_material(context, &camera, &geometry, &material, &[]);
        }
        Ok(())
    })
    .unwrap()
    .read_color::<[f32; 4]>()[0];
    let depth = result[0];
    if depth < 1.0 {
        Some(IntersectionResult {
            position: position + direction * depth * max_depth,
            geometry_id: result[1].to_bits(),
            instance_id: result[2].to_bits(),
            primitive_id: result[3].to_bits(),
        })
    } else {
        None
    }
}
