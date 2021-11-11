use crate::core::*;

///
/// A triangle mesh where the mesh data is transfered to the GPU.
///
pub struct Mesh {
    /// Buffer with the position data, ie. `(x, y, z)` for each vertex
    pub position_buffer: VertexBuffer,
    /// Buffer with the normal data, ie. `(x, y, z)` for each vertex.
    pub normal_buffer: Option<VertexBuffer>,
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
            index_buffer,
            uv_buffer,
            color_buffer,
            name: cpu_mesh.name.clone(),
        })
    }

    pub fn draw(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
        transformation: &Mat4,
    ) -> ThreeDResult<()> {
        program.use_uniform_block("Camera", camera_buffer);
        program.use_uniform_mat4("modelMatrix", transformation)?;

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
            program.use_uniform_mat4(
                "normalMatrix",
                &transformation.invert().unwrap().transpose(),
            )?;
        }
        if program.requires_attribute("color") {
            let color_buffer = self
                .color_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("color".to_string()))?;
            program.use_attribute_vec4("color", color_buffer)?;
        }
        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(render_states, viewport, index_buffer);
        } else {
            program.draw_arrays(
                render_states,
                viewport,
                self.position_buffer.count() as u32 / 3,
            );
        }
        Ok(())
    }

    pub fn vertex_shader_source(fragment_shader_source: &str) -> String {
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
