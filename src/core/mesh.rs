use crate::core::*;
use std::rc::Rc;

///
/// A shader program used for rendering one or more instances of a [Mesh](Mesh). It has a fixed vertex shader and
/// customizable fragment shader for custom lighting. Use this in combination with [Mesh::render].
///
#[deprecated]
pub struct MeshProgram {
    program: Program,
}

#[allow(deprecated)]
impl MeshProgram {
    ///
    /// Constructs a new shader program for rendering meshes. The fragment shader can use the fragments position in world space by adding `in vec3 pos;`,
    /// its normal by `in vec3 nor;`, its uv coordinates by `in vec2 uvs;` and its per vertex color by `in vec4 col;` to the shader source code.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> ThreeDResult<Self> {
        Ok(Self {
            program: Program::from_source(
                context,
                &Mesh::vertex_shader_source(&fragment_shader_source),
                fragment_shader_source,
            )?,
        })
    }
}

#[allow(deprecated)]
impl std::ops::Deref for MeshProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.program
    }
}

///
/// A triangle mesh where the mesh data is transfered to the GPU.
///
#[derive(Clone)]
pub struct Mesh {
    context: Context,
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
    transformation: Mat4,
    normal_transformation: Mat4,
    aabb: AxisAlignedBoundingBox,
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
        let aabb = cpu_mesh.compute_aabb();
        Ok(Self {
            context: context.clone(),
            position_buffer,
            normal_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            aabb,
            name: cpu_mesh.name.clone(),
            transformation: Mat4::identity(),
            normal_transformation: Mat4::identity(),
        })
    }

    ///
    /// Returns the local to world transformation of this mesh.
    ///
    #[deprecated = "Use Model instead"]
    pub fn transformation(&self) -> &Mat4 {
        &self.transformation
    }

    ///
    /// Set the local to world transformation of this mesh.
    ///
    #[deprecated = "Use Model instead"]
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.normal_transformation = self.transformation.invert().unwrap().transpose();
    }

    ///
    /// Render the mesh with the given [Program].
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    /// # Errors
    /// Will return an error if the program requires a certain attribute and the mesh does not have that attribute data.
    /// For example if the program needs the normal to calculate lighting, but the mesh does not have per vertex normals, this
    /// function will return an error.
    ///
    #[deprecated = "Instead, use render_forward in Model or use Program and send attributes and uniforms self"]
    pub fn render(
        &self,
        render_states: RenderStates,
        program: &Program,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
        self.draw(render_states, program, camera_buffer, viewport)
    }

    ///
    /// Computes the axis aligned bounding box for this mesh.
    ///
    #[deprecated = "Use Model instead"]
    pub fn aabb(&self) -> &AxisAlignedBoundingBox {
        &self.aabb
    }

    pub(crate) fn draw(
        &self,
        render_states: RenderStates,
        program: &Program,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.use_attributes(program, camera_buffer)?;
        if program.requires_attribute("normal") {
            program.use_uniform_mat4("normalMatrix", &self.normal_transformation)?;
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
