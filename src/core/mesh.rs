use crate::core::*;
use std::rc::Rc;

///
/// A triangle mesh where the mesh data is transfered to the GPU.
///
#[derive(Clone)]
pub struct Mesh {
    /// Buffer with the position data, ie. `(x, y, z)` for each vertex
    pub position_buffer: Rc<VertexBuffer>,
    /// Buffer with the normal data, ie. `(x, y, z)` for each vertex.
    pub normal_buffer: Option<Rc<VertexBuffer>>,
    /// Buffer with the uv coordinate data, ie. `(u, v)` for each vertex.
    pub uv_buffer: Option<Rc<VertexBuffer>>,
    /// Buffer with the color data, ie. `(r, g, b)` for each vertex.
    pub color_buffer: Option<Rc<VertexBuffer>>,
    /// Buffer with the index data, ie. three contiguous integers define the triangle where each integer is and index into the other vertex buffers.
    pub index_buffer: Option<Rc<ElementBuffer>>,
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

        let position_buffer = Rc::new(VertexBuffer::new_with_static(context, &cpu_mesh.positions)?);
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(Rc::new(VertexBuffer::new_with_static(context, normals)?))
        } else {
            None
        };
        let index_buffer = if let Some(ref indices) = cpu_mesh.indices {
            Some(Rc::new(match indices {
                Indices::U8(ind) => ElementBuffer::new_with(context, ind)?,
                Indices::U16(ind) => ElementBuffer::new_with(context, ind)?,
                Indices::U32(ind) => ElementBuffer::new_with(context, ind)?,
            }))
        } else {
            None
        };
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(Rc::new(VertexBuffer::new_with_static(context, uvs)?))
        } else {
            None
        };
        let color_buffer = if let Some(ref colors) = cpu_mesh.colors {
            Some(Rc::new(VertexBuffer::new_with_static(context, colors)?))
        } else {
            None
        };
        Ok(Self {
            position_buffer,
            normal_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            name: cpu_mesh.name.clone(),
        })
    }

    pub(crate) fn use_attributes(
        &self,
        program: &Program,
        camera_buffer: &UniformBuffer,
    ) -> ThreeDResult<()> {
        program.use_uniform_block("Camera", camera_buffer);

        if program.requires_attribute("position") {
            program.use_attribute_vec3("position", &self.position_buffer)?;
        }
        if program.requires_attribute("uv_coordinates") {
            let uv_buffer = self
                .uv_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("uv coordinates".to_string()))?;
            program.use_attribute_vec2("uv_coordinates", uv_buffer)?;
        }
        if program.requires_attribute("normal") {
            let normal_buffer = self
                .normal_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("normal".to_string()))?;
            program.use_attribute_vec3("normal", normal_buffer)?;
        }
        if program.requires_attribute("color") {
            let color_buffer = self
                .color_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("color".to_string()))?;
            program.use_attribute_vec4("color", color_buffer)?;
        }
        Ok(())
    }

    pub(crate) fn vertex_shader_source(fragment_shader_source: &str) -> String {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        format!(
            "{}{}{}{}{}{}",
            if use_positions {
                "#define USE_POSITIONS\n"
            } else {
                ""
            },
            if use_normals {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if use_uvs { "#define USE_UVS\n" } else { "" },
            if use_colors {
                "#define USE_COLORS\n"
            } else {
                ""
            },
            include_str!("shared.frag"),
            include_str!("mesh.vert"),
        )
    }
}
