use crate::core::*;

///
/// A shader program used for rendering one or more instances of a [InstancedMesh]. It has a fixed vertex shader and
/// customizable fragment shader for custom lighting. Use this in combination with [InstancedMesh::render].
///
#[deprecated]
pub struct InstancedMeshProgram {
    program: Program,
}

#[allow(deprecated)]
impl InstancedMeshProgram {
    ///
    /// Constructs a new shader program for rendering instanced meshes. The fragment shader can use the fragments position by adding `in vec3 pos;`,
    /// its normal by `in vec3 nor;`, its uv coordinates by `in vec2 uvs;` and its per vertex color by `in vec4 col;` to the shader source code.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> ThreeDResult<Self> {
        Ok(Self {
            program: Program::from_source(
                context,
                &InstancedMesh::vertex_shader_source(&fragment_shader_source),
                fragment_shader_source,
            )?,
        })
    }
}

#[allow(deprecated)]
impl std::ops::Deref for InstancedMeshProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.program
    }
}

///
/// Similar to [Mesh], except it is possible to render many instances of the same triangle mesh efficiently.
///
#[deprecated = "Use InstancedModel instead"]
pub struct InstancedMesh {
    pub(crate) mesh: Mesh,
    instance_count: u32,
    instance_buffer1: InstanceBuffer,
    instance_buffer2: InstanceBuffer,
    instance_buffer3: InstanceBuffer,
}

#[allow(deprecated)]
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
    ) -> ThreeDResult<Self> {
        let mut mesh = Self {
            instance_count: 0,
            mesh: Mesh::new(context, cpu_mesh)?,
            instance_buffer1: InstanceBuffer::new(context)?,
            instance_buffer2: InstanceBuffer::new(context)?,
            instance_buffer3: InstanceBuffer::new(context)?,
        };
        mesh.update_transformations(transformations);
        Ok(mesh)
    }

    ///
    /// Render the instanced mesh with the given [Program].
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the program requires a certain attribute and the instanced mesh does not have that attribute.
    /// For example if the program needs the normal to calculate lighting, but the mesh does not have per vertex normals, this
    /// function will return an error.
    ///
    pub fn render(
        &self,
        render_states: RenderStates,
        program: &Program,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.draw(render_states, program, camera_buffer, viewport, None)
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

    pub(crate) fn draw(
        &self,
        render_states: RenderStates,
        program: &Program,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
        transformation: Option<Mat4>,
    ) -> ThreeDResult<()> {
        program.use_attribute_vec4_instanced("row1", &self.instance_buffer1)?;
        program.use_attribute_vec4_instanced("row2", &self.instance_buffer2)?;
        program.use_attribute_vec4_instanced("row3", &self.instance_buffer3)?;

        self.mesh.use_attributes(program, camera_buffer)?;
        program.use_uniform_mat4(
            "modelMatrix",
            &transformation.unwrap_or(*self.transformation()),
        )?;

        if let Some(ref index_buffer) = self.mesh.index_buffer {
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
                self.mesh.position_buffer.count() as u32 / 3,
                self.instance_count,
            );
        }
        Ok(())
    }

    pub(crate) fn vertex_shader_source(fragment_shader_source: &str) -> String {
        format!(
            "#define INSTANCED\n{}",
            Mesh::vertex_shader_source(fragment_shader_source)
        )
    }
}

#[allow(deprecated)]
impl std::ops::Deref for InstancedMesh {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

#[allow(deprecated)]
impl std::ops::DerefMut for InstancedMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}
