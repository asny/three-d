//!
//! A collection of geometries implementing the [Geometry] trait, for example a [Mesh].
//! A geometry together with a [material] can be rendered directly, or combined into an [object] (see [Gm]) that can be used in a render call, for example [render_pass].
//!

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

use crate::core::*;
use crate::renderer::*;

pub use three_d_asset::{Indices, Positions, TriMesh as CpuMesh, VoxelGrid as CpuVolume};

///
/// Represents a 3D geometry that, together with a [material], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [render_pass].
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
    /// Render the geometry with the given material.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    /// Use an empty array for the `lights` argument, if the objects does not require lights to be rendered.
    ///
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()>;

    ///
    /// Returns the [AxisAlignedBoundingBox] for this geometry in the global coordinate system.
    ///
    fn aabb(&self) -> AxisAlignedBoundingBox;
}

impl<T: Geometry + ?Sized> Geometry for &T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        (*self).render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

impl<T: Geometry + ?Sized> Geometry for &mut T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        (**self).render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        (**self).aabb()
    }
}

impl<T: Geometry> Geometry for Box<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.as_ref().aabb()
    }
}

impl<T: Geometry> Geometry for std::rc::Rc<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.as_ref().aabb()
    }
}

impl<T: Geometry> Geometry for std::rc::Rc<std::cell::RefCell<T>> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.borrow().render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.borrow().aabb()
    }
}

///
/// Represents a 2D geometry that is possible to render with a [Material].
///
pub trait Geometry2D {
    ///
    /// Render the object with the given material.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    fn render_with_material(&self, material: &dyn Material, viewport: Viewport)
        -> ThreeDResult<()>;
}

impl<T: Geometry2D + ?Sized> Geometry2D for &T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (*self).render_with_material(material, viewport)
    }
}

impl<T: Geometry2D + ?Sized> Geometry2D for &mut T {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        (**self).render_with_material(material, viewport)
    }
}

impl<T: Geometry2D> Geometry2D for Box<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, viewport)
    }
}

impl<T: Geometry2D> Geometry2D for std::rc::Rc<T> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.as_ref().render_with_material(material, viewport)
    }
}

impl<T: Geometry2D> Geometry2D for std::rc::Rc<std::cell::RefCell<T>> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.borrow().render_with_material(material, viewport)
    }
}

use std::collections::HashMap;
fn vertex_buffers_from_mesh(
    context: &Context,
    cpu_mesh: &CpuMesh,
) -> ThreeDResult<HashMap<String, VertexBuffer>> {
    #[cfg(debug_assertions)]
    cpu_mesh.validate()?;

    let mut buffers = HashMap::new();
    buffers.insert(
        "position".to_string(),
        VertexBuffer::new_with_data(context, &cpu_mesh.positions.to_f32())?,
    );
    if let Some(ref normals) = cpu_mesh.normals {
        buffers.insert(
            "normal".to_string(),
            VertexBuffer::new_with_data(context, normals)?,
        );
    };
    if let Some(ref tangents) = cpu_mesh.tangents {
        buffers.insert(
            "tangent".to_string(),
            VertexBuffer::new_with_data(context, tangents)?,
        );
    };
    if let Some(ref uvs) = cpu_mesh.uvs {
        buffers.insert(
            "uv_coordinates".to_string(),
            VertexBuffer::new_with_data(
                context,
                &uvs.iter()
                    .map(|uv| vec2(uv.x, 1.0 - uv.y))
                    .collect::<Vec<_>>(),
            )?,
        );
    };
    if let Some(ref colors) = cpu_mesh.colors {
        buffers.insert(
            "color".to_string(),
            VertexBuffer::new_with_data(context, colors)?,
        );
    };
    Ok(buffers)
}

fn index_buffer_from_mesh(
    context: &Context,
    cpu_mesh: &CpuMesh,
) -> ThreeDResult<Option<ElementBuffer>> {
    Ok(if let Some(ref indices) = cpu_mesh.indices {
        Some(match indices {
            Indices::U8(ind) => ElementBuffer::new_with_data(context, ind)?,
            Indices::U16(ind) => ElementBuffer::new_with_data(context, ind)?,
            Indices::U32(ind) => ElementBuffer::new_with_data(context, ind)?,
        })
    } else {
        None
    })
}
