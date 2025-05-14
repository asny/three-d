#![macro_use]
//!
//! A collection of geometries implementing the [Geometry] trait.
//!
//! A geometry together with a [material] can be rendered directly, or combined into an [object] (see [Gm]) that can be used in a render call, for example [RenderTarget::render].
//!

macro_rules! impl_geometry_body {
    ($inner:ident) => {
        fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
            self.$inner().draw(viewer, program, render_states)
        }

        fn vertex_shader_source(&self) -> String {
            self.$inner().vertex_shader_source()
        }

        fn id(&self) -> GeometryId {
            self.$inner().id()
        }

        fn render_with_material(
            &self,
            material: &dyn Material,
            viewer: &dyn Viewer,
            lights: &[&dyn Light],
        ) {
            self.$inner().render_with_material(material, viewer, lights)
        }

        fn render_with_effect(
            &self,
            material: &dyn Effect,
            viewer: &dyn Viewer,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) {
            self.$inner()
                .render_with_effect(material, viewer, lights, color_texture, depth_texture)
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
/// In addition, for the geometry to be pickable using the [pick] or [ray_intersect] methods (ie. combined with the [IntersectionMaterial]),
/// it needs to support `flat out int instance_id;`. Simply set it to the built-in glsl variable: `gl_InstanceID`.
///
pub trait Geometry {
    ///
    /// Draw this geometry.
    ///
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates);

    ///
    /// Returns the vertex shader source for this geometry given that the fragment shader needs the given vertex attributes.
    ///
    fn vertex_shader_source(&self) -> String;

    ///
    /// Returns a unique ID for each variation of the shader source returned from `Geometry::vertex_shader_source`.
    ///
    /// **Note:** The last bit is reserved to internally implemented geometries, so if implementing the `Geometry` trait
    /// outside of this crate, always return an id in the public use range as defined by [GeometryId].
    ///
    fn id(&self) -> GeometryId;

    ///
    /// Render the geometry with the given [Material].
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    );

    ///
    /// Render the geometry with the given [Effect].
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    /// Use an empty array for the `lights` argument, if the material does not require lights to be rendered.
    ///
    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
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
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        self.read().unwrap().draw(viewer, program, render_states)
    }

    fn vertex_shader_source(&self) -> String {
        self.read().unwrap().vertex_shader_source()
    }

    fn id(&self) -> GeometryId {
        self.read().unwrap().id()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        self.read()
            .unwrap()
            .render_with_material(material, viewer, lights)
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.read().unwrap().render_with_effect(
            material,
            viewer,
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

///
/// The index buffer used to determine the three vertices for each triangle in a mesh.
/// A triangle is defined by three consequitive indices in the index buffer.
/// Each index points to a position in the vertex buffers.
///
pub enum IndexBuffer {
    /// No index buffer is used, ie. every triangle consist of three consequitive vertices.
    None,
    /// Use an index buffer with indices defined in `u8` format.
    U8(ElementBuffer<u8>),
    /// Use an index buffer with indices defined in `u16` format.
    U16(ElementBuffer<u16>),
    /// Use an index buffer with indices defined in `u32` format.
    U32(ElementBuffer<u32>),
}

struct BaseMesh {
    indices: IndexBuffer,
    positions: VertexBuffer<Vec3>,
    normals: Option<VertexBuffer<Vec3>>,
    tangents: Option<VertexBuffer<Vec4>>,
    uvs: Option<VertexBuffer<Vec2>>,
    colors: Option<VertexBuffer<Vec4>>,
}

impl BaseMesh {
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> Self {
        #[cfg(debug_assertions)]
        cpu_mesh.validate().expect("invalid cpu mesh");

        Self {
            indices: match &cpu_mesh.indices {
                Indices::U8(ind) => IndexBuffer::U8(ElementBuffer::new_with_data(context, ind)),
                Indices::U16(ind) => IndexBuffer::U16(ElementBuffer::new_with_data(context, ind)),
                Indices::U32(ind) => IndexBuffer::U32(ElementBuffer::new_with_data(context, ind)),
                Indices::None => IndexBuffer::None,
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

    pub fn draw(&self, program: &Program, render_states: RenderStates, viewer: &dyn Viewer) {
        self.use_attributes(program);

        let mode = crate::context::TRIANGLES;

        match &self.indices {
            IndexBuffer::None => program.draw_arrays(
                render_states,
                viewer.viewport(),
                self.positions.vertex_count(), mode
            ),
            IndexBuffer::U8(element_buffer) => {
                program.draw_elements(render_states, viewer.viewport(), element_buffer, mode)
            }
            IndexBuffer::U16(element_buffer) => {
                program.draw_elements(render_states, viewer.viewport(), element_buffer, mode)
            }
            IndexBuffer::U32(element_buffer) => {
                program.draw_elements(render_states, viewer.viewport(), element_buffer, mode)
            }
        }
    }

    pub fn draw_instanced(
        &self,
        program: &Program,
        render_states: RenderStates,
        viewer: &dyn Viewer,
        instance_count: u32,
    ) {
        self.use_attributes(program);

        match &self.indices {
            IndexBuffer::None => program.draw_arrays_instanced(
                render_states,
                viewer.viewport(),
                self.positions.vertex_count(),
                instance_count,
            ),
            IndexBuffer::U8(element_buffer) => program.draw_elements_instanced(
                render_states,
                viewer.viewport(),
                element_buffer,
                instance_count,
            ),
            IndexBuffer::U16(element_buffer) => program.draw_elements_instanced(
                render_states,
                viewer.viewport(),
                element_buffer,
                instance_count,
            ),
            IndexBuffer::U32(element_buffer) => program.draw_elements_instanced(
                render_states,
                viewer.viewport(),
                element_buffer,
                instance_count,
            ),
        }
    }

    fn use_attributes(&self, program: &Program) {
        program.use_vertex_attribute("position", &self.positions);

        if program.requires_attribute("normal") {
            if let Some(normals) = &self.normals {
                program.use_vertex_attribute("normal", normals);
            }
        }

        if program.requires_attribute("tangent") {
            if let Some(tangents) = &self.tangents {
                program.use_vertex_attribute("tangent", tangents);
            }
        }

        if program.requires_attribute("uv_coordinates") {
            if let Some(uvs) = &self.uvs {
                program.use_vertex_attribute("uv_coordinates", uvs);
            }
        }

        if program.requires_attribute("color") {
            if let Some(colors) = &self.colors {
                program.use_vertex_attribute("color", colors);
            }
        }
    }

    fn vertex_shader_source(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            if self.normals.is_some() {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if self.tangents.is_some() {
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if self.uvs.is_some() {
                "#define USE_UVS\n"
            } else {
                ""
            },
            if self.colors.is_some() {
                "#define USE_VERTEX_COLORS\n"
            } else {
                ""
            },
            include_str!("../core/shared.frag"),
            include_str!("geometry/shaders/mesh.vert"),
        )
    }
}
