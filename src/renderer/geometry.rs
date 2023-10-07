#![macro_use]
//!
//! A collection of geometries implementing the [Geometry] trait.
//!
//! A geometry together with a [material] can be rendered directly, or combined into an [object] (see [Gm]) that can be used in a render call, for example [RenderTarget::render].
//!

macro_rules! impl_geometry_body {
    ($inner:ident) => {
        fn draw(
            &self,
            camera: &Camera,
            program: &Program,
            render_states: RenderStates,
            attributes: FragmentAttributes,
        ) {
            self.$inner()
                .draw(camera, program, render_states, attributes)
        }

        fn vertex_shader_source(&self, required_attributes: FragmentAttributes) -> String {
            self.$inner().vertex_shader_source(required_attributes)
        }

        fn id(&self, required_attributes: FragmentAttributes) -> u16 {
            self.$inner().id(required_attributes)
        }

        fn render_with_material(
            &self,
            material: &dyn Material,
            camera: &Camera,
            lights: &[&dyn Light],
        ) {
            self.$inner().render_with_material(material, camera, lights)
        }

        fn render_with_effect(
            &self,
            material: &dyn Effect,
            camera: &Camera,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) {
            self.$inner()
                .render_with_effect(material, camera, lights, color_texture, depth_texture)
        }

        fn aabb(&self) -> AxisAlignedBoundingBox {
            self.$inner().aabb()
        }
    };
}

mod mesh;
#[doc(inline)]
pub use mesh::*;

mod instanced_mesh;
#[doc(inline)]
pub use instanced_mesh::*;

mod sprites;
#[doc(inline)]
pub use sprites::*;

mod particles;
#[doc(inline)]
pub use particles::*;

mod bounding_box;
#[doc(inline)]
pub use bounding_box::*;

mod line;
#[doc(inline)]
pub use line::*;

mod rectangle;
#[doc(inline)]
pub use rectangle::*;

mod circle;
#[doc(inline)]
pub use circle::*;

use crate::core::*;
use crate::renderer::*;

pub use three_d_asset::{
    Geometry as CpuGeometry, Indices, KeyFrameAnimation, KeyFrames, PointCloud, Positions,
    TriMesh as CpuMesh,
};

///
/// Represents a 3D geometry that, together with a [material], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [RenderTarget::render].
///
/// If requested by the material, the geometry has to support the following attributes in the vertex shader source code.
/// - position: `out vec3 pos;` (must be in world space)
/// - normal: `out vec3 nor;`
/// - tangent: `out vec3 tang;`
/// - bitangent: `out vec3 bitang;`
/// - uv coordinates: `out vec2 uvs;` (must be flipped in v compared to standard uv coordinates, ie. do `uvs = vec2(uvs.x, 1.0 - uvs.y);` in the vertex shader or do the flip before constructing the uv coordinates vertex buffer)
/// - color: `out vec4 col;`
///
pub trait Geometry {
    ///
    /// Draw this geometry.
    ///
    fn draw(
        &self,
        camera: &Camera,
        program: &Program,
        render_states: RenderStates,
        attributes: FragmentAttributes,
    );

    ///
    /// Returns the vertex shader source for this geometry given that the fragment shader needs the given vertex attributes.
    ///
    fn vertex_shader_source(&self, required_attributes: FragmentAttributes) -> String;

    ///
    /// Returns a unique ID for each variation of the shader source returned from `Geometry::vertex_shader_source`.
    ///
    /// **Note:** The last bit is reserved to internally implemented geometries, so if implementing the `Geometry` trait
    /// outside of this crate, always return an id that is smaller than `0b1u16 << 15`.
    ///
    fn id(&self, required_attributes: FragmentAttributes) -> u16;

    ///
    /// Render the geometry with the given [Material].
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    fn render_with_material(&self, material: &dyn Material, camera: &Camera, lights: &[&dyn Light]);

    ///
    /// Render the geometry with the given [Effect].
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    fn render_with_effect(
        &self,
        material: &dyn Effect,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    );

    ///
    /// Returns the [AxisAlignedBoundingBox] for this geometry in the global coordinate system.
    ///
    fn aabb(&self) -> AxisAlignedBoundingBox;

    ///
    /// For updating the animation of this geometry if it is animated, if not, this method does nothing.
    /// The time parameter should be some continious time, for example the time since start.
    ///
    fn animate(&mut self, _time: f32) {}
}

use std::ops::Deref;
impl<T: Geometry + ?Sized> Geometry for &T {
    impl_geometry_body!(deref);
}

impl<T: Geometry + ?Sized> Geometry for &mut T {
    impl_geometry_body!(deref);

    fn animate(&mut self, time: f32) {
        self.deref().animate(time)
    }
}

impl<T: Geometry> Geometry for Box<T> {
    impl_geometry_body!(as_ref);
}

impl<T: Geometry> Geometry for std::rc::Rc<T> {
    impl_geometry_body!(as_ref);
}

impl<T: Geometry> Geometry for std::sync::Arc<T> {
    impl_geometry_body!(as_ref);
}

impl<T: Geometry> Geometry for std::cell::RefCell<T> {
    impl_geometry_body!(borrow);

    fn animate(&mut self, time: f32) {
        self.borrow_mut().animate(time)
    }
}

impl<T: Geometry> Geometry for std::sync::RwLock<T> {
    fn draw(
        &self,
        camera: &Camera,
        program: &Program,
        render_states: RenderStates,
        attributes: FragmentAttributes,
    ) {
        self.read()
            .unwrap()
            .draw(camera, program, render_states, attributes)
    }

    fn vertex_shader_source(&self, required_attributes: FragmentAttributes) -> String {
        self.read()
            .unwrap()
            .vertex_shader_source(required_attributes)
    }

    fn id(&self, required_attributes: FragmentAttributes) -> u16 {
        self.read().unwrap().id(required_attributes)
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.read()
            .unwrap()
            .render_with_material(material, camera, lights)
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.read().unwrap().render_with_effect(
            material,
            camera,
            lights,
            color_texture,
            depth_texture,
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.read().unwrap().aabb()
    }

    fn animate(&mut self, time: f32) {
        self.write().unwrap().animate(time)
    }
}

struct BaseMesh {
    indices: Option<ElementBuffer>,
    positions: VertexBuffer,
    normals: Option<VertexBuffer>,
    tangents: Option<VertexBuffer>,
    uvs: Option<VertexBuffer>,
    colors: Option<VertexBuffer>,
}

impl BaseMesh {
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> Self {
        #[cfg(debug_assertions)]
        cpu_mesh.validate().expect("invalid cpu mesh");

        Self {
            indices: match &cpu_mesh.indices {
                Indices::U8(ind) => Some(ElementBuffer::new_with_data(context, ind)),
                Indices::U16(ind) => Some(ElementBuffer::new_with_data(context, ind)),
                Indices::U32(ind) => Some(ElementBuffer::new_with_data(context, ind)),
                Indices::None => None,
            },
            positions: VertexBuffer::new_with_data(context, &cpu_mesh.positions.to_f32()),
            normals: cpu_mesh
                .normals
                .as_ref()
                .map(|data| VertexBuffer::new_with_data(context, data)),
            tangents: cpu_mesh
                .tangents
                .as_ref()
                .map(|data| VertexBuffer::new_with_data(context, data)),
            uvs: cpu_mesh.uvs.as_ref().map(|data| {
                VertexBuffer::new_with_data(
                    context,
                    &data
                        .iter()
                        .map(|uv| vec2(uv.x, 1.0 - uv.y))
                        .collect::<Vec<_>>(),
                )
            }),
            colors: cpu_mesh.colors.as_ref().map(|data| {
                VertexBuffer::new_with_data(
                    context,
                    &data.iter().map(|c| c.to_linear_srgb()).collect::<Vec<_>>(),
                )
            }),
        }
    }

    pub fn draw(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera: &Camera,
        attributes: FragmentAttributes,
    ) {
        self.use_attributes(program, attributes);
        if let Some(index_buffer) = &self.indices {
            program.draw_elements(render_states, camera.viewport(), index_buffer)
        } else {
            program.draw_arrays(
                render_states,
                camera.viewport(),
                self.positions.vertex_count(),
            )
        }
    }

    pub fn draw_instanced(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera: &Camera,
        attributes: FragmentAttributes,
        instance_count: u32,
    ) {
        self.use_attributes(program, attributes);

        if let Some(index_buffer) = &self.indices {
            program.draw_elements_instanced(
                render_states,
                camera.viewport(),
                index_buffer,
                instance_count,
            )
        } else {
            program.draw_arrays_instanced(
                render_states,
                camera.viewport(),
                self.positions.vertex_count(),
                instance_count,
            )
        }
    }

    fn use_attributes(&self, program: &Program, attributes: FragmentAttributes) {
        program.use_vertex_attribute("position", &self.positions);

        if attributes.normal {
            program.use_vertex_attribute(
                "normal",
                self.normals.as_ref().unwrap_or_else(|| {
                    panic!(
                        "the material requires normal attributes but the geometry did not provide it"
                    )
                }),
            );
        }

        if attributes.tangents {
            program.use_vertex_attribute(
                "tangent",
                self.tangents.as_ref().unwrap_or_else(|| {
                    panic!(
                        "the material requires tangent attributes but the geometry did not provide it"
                    )
                }),
            );
        }

        if attributes.uv {
            program.use_vertex_attribute(
                "uv_coordinates",
                self.uvs.as_ref().unwrap_or_else(|| {
                    panic!(
                        "the material requires uv coordinate attributes but the geometry did not provide it"
                    )
                }),
            );
        }

        if attributes.color {
            if let Some(colors) = &self.colors {
                program.use_vertex_attribute("color", colors);
            }
        }
    }
}
