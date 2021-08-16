use crate::core::*;

///
/// A shader program used for rendering one or more instances of a [InstancedMesh]. It has a fixed vertex shader and
/// customizable fragment shader for custom lighting. Use this in combination with [InstancedMesh::render].
///
pub struct InstancedMeshProgram {
    mesh_program: MeshProgram,
}

impl InstancedMeshProgram {
    ///
    /// Constructs a new shader program for rendering instanced meshes. The fragment shader can use the fragments position by adding `in vec3 pos;`,
    /// its normal by `in vec3 nor;`, its uv coordinates by `in vec2 uvs;` and its per vertex color by `in vec4 col;` to the shader source code.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self, Error> {
        Ok(Self {
            mesh_program: MeshProgram::new_internal(context, fragment_shader_source, true)?,
        })
    }
}

impl std::ops::Deref for InstancedMeshProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.mesh_program
    }
}

///
/// Similar to [Mesh], except it is possible to render many instances of the same triangle mesh efficiently.
///
pub struct InstancedMesh {
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    color_buffer: Option<VertexBuffer>,
    pub(crate) transparent: bool,
    instance_count: u32,
    instance_buffer1: VertexBuffer,
    instance_buffer2: VertexBuffer,
    instance_buffer3: VertexBuffer,
    pub name: String,
    transformation: Mat4,
}

impl InstancedMesh {
    ///
    /// Constructs a new InstancedMesh from the given [CPUMesh]. The mesh is rendered
    /// in as many instances as there are transformation matrices in the transformations parameter.
    /// Each instance is transformed with the given transformation before it is rendered.
    /// The transformations can be updated by the [update_transformations](Self::update_transformations) function.
    ///
    pub fn new(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
    ) -> Result<Self, Error> {
        let position_buffer = VertexBuffer::new_with_static(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(VertexBuffer::new_with_static(context, normals)?)
        } else {
            None
        };
        let index_buffer = if let Some(ref indices) = cpu_mesh.indices {
            Some(match indices {
                Indices::U8(ind) => ElementBuffer::new(context, ind)?,
                Indices::U16(ind) => ElementBuffer::new(context, ind)?,
                Indices::U32(ind) => ElementBuffer::new(context, ind)?,
            })
        } else {
            None
        };
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(VertexBuffer::new_with_static(context, uvs)?)
        } else {
            None
        };
        let mut transparent = false;
        let color_buffer = if let Some(ref colors) = cpu_mesh.colors {
            for i in 0..colors.len() / 4 {
                if colors[i * 4] != 255 {
                    transparent = true;
                    break;
                }
            }
            Some(VertexBuffer::new_with_static(context, colors)?)
        } else {
            None
        };

        let mut mesh = Self {
            name: cpu_mesh.name.clone(),
            instance_count: 0,
            position_buffer,
            normal_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            transparent,
            instance_buffer1: VertexBuffer::new(context)?,
            instance_buffer2: VertexBuffer::new(context)?,
            instance_buffer3: VertexBuffer::new(context)?,
            transformation: Mat4::identity(),
        };
        mesh.update_transformations(transformations);
        Ok(mesh)
    }

    pub fn transformation(&self) -> &Mat4 {
        &self.transformation
    }

    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }

    ///
    /// Render the instanced mesh with the given [InstancedMeshProgram].
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced mesh.
    ///
    /// # Errors
    /// Will return an error if the instanced mesh shader program requires a certain attribute and the instanced mesh does not have that attribute.
    /// For example if the program needs the normal to calculate lighting, but the mesh does not have per vertex normals, this
    /// function will return an error.
    ///
    pub fn render(
        &self,
        render_states: RenderStates,
        program: &InstancedMeshProgram,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> Result<(), Error> {
        program.use_attribute_vec4_divisor("row1", &self.instance_buffer1, 1)?;
        program.use_attribute_vec4_divisor("row2", &self.instance_buffer2, 1)?;
        program.use_attribute_vec4_divisor("row3", &self.instance_buffer3, 1)?;

        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
        program.use_uniform_block("Camera", camera_buffer);

        program.use_attribute_vec3("position", &self.position_buffer)?;
        if program.mesh_program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(Error::MeshError {
                message:
                    "The mesh shader program needs uv coordinates, but the mesh does not have any."
                        .to_string(),
            })?;
            program.use_attribute_vec2("uv_coordinates", uv_buffer)?;
        }
        if program.mesh_program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.use_attribute_vec3("normal", normal_buffer)?;
        }
        if program.mesh_program.use_colors {
            let color_buffer = self.color_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs per vertex colors, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec4("color", color_buffer)?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(
                render_states,
                viewport,
                index_buffer,
                self.instance_count,
            );
        } else {
            program.draw_arrays_instanced(
                render_states,
                viewport,
                self.position_buffer.count() as u32 / 3,
                self.instance_count,
            );
        }
        Ok(())
    }

    ///
    /// Updates the transformations applied to each mesh instance before they are rendered.
    /// The mesh is rendered in as many instances as there are transformation matrices.
    ///
    pub fn update_transformations(&mut self, transformations: &[Mat4]) {
        self.instance_count = transformations.len() as u32;
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        for transform in transformations {
            row1.push(transform.x.x);
            row1.push(transform.y.x);
            row1.push(transform.z.x);
            row1.push(transform.w.x);

            row2.push(transform.x.y);
            row2.push(transform.y.y);
            row2.push(transform.z.y);
            row2.push(transform.w.y);

            row3.push(transform.x.z);
            row3.push(transform.y.z);
            row3.push(transform.z.z);
            row3.push(transform.w.z);
        }
        self.instance_buffer1.fill_with_dynamic(&row1);
        self.instance_buffer2.fill_with_dynamic(&row2);
        self.instance_buffer3.fill_with_dynamic(&row3);
    }
}
