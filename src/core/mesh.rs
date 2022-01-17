use crate::core::*;

///
/// A triangle mesh where the mesh data is transfered to the GPU.
///
pub struct Mesh {
    /// Buffer with the position data, ie. `(x, y, z)` for each vertex
    pub position_buffer: VertexBuffer,
    /// Buffer with the normal data, ie. `(x, y, z)` for each vertex.
    pub normal_buffer: Option<VertexBuffer>,
    /// Buffer with the tangent data, ie. `(x, y, z)` for each vertex.
    pub tangent_buffer: Option<VertexBuffer>,
    /// Buffer with the uv coordinate data, ie. `(u, v)` for each vertex.
    pub uv_buffer: Option<VertexBuffer>,
    /// Buffer with the color data, ie. `(r, g, b)` for each vertex.
    pub color_buffer: Option<VertexBuffer>,
    /// Buffer with the index data, ie. three contiguous integers define the triangle where each integer is and index into the other vertex buffers.
    pub index_buffer: Option<ElementBuffer>,
    /// Optional name of the mesh.
    pub name: String,
}

impl Mesh {
    ///
    /// Copies the per vertex data defined in the given [CPUMesh](crate::CPUMesh) to the GPU, thereby
    /// making it possible to render the mesh.
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> ThreeDResult<Self> {
        cpu_mesh.validate()?;

        let position_buffer = VertexBuffer::new_with_static(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(VertexBuffer::new_with_static(context, normals)?)
        } else {
            None
        };
        let tangent_buffer = if let Some(ref tangents) = cpu_mesh.tangents {
            Some(VertexBuffer::new_with_static(context, tangents)?)
        } else {
            None
        };
        let index_buffer = if let Some(ref indices) = cpu_mesh.indices {
            Some(match indices {
                Indices::U8(ind) => ElementBuffer::new_with(context, ind)?,
                Indices::U16(ind) => ElementBuffer::new_with(context, ind)?,
                Indices::U32(ind) => ElementBuffer::new_with(context, ind)?,
            })
        } else {
            None
        };
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(VertexBuffer::new_with_static(context, uvs)?)
        } else {
            None
        };
        let color_buffer = if let Some(ref colors) = cpu_mesh.colors {
            Some(VertexBuffer::new_with_static(context, colors)?)
        } else {
            None
        };
        Ok(Self {
            position_buffer,
            normal_buffer,
            tangent_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            name: cpu_mesh.name.clone(),
        })
    }
}
