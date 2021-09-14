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

impl MeshProgram {
    ///
    /// Constructs a new shader program for rendering meshes. The fragment shader can use the fragments position in world space by adding `in vec3 pos;`,
    /// its normal by `in vec3 nor;`, its uv coordinates by `in vec2 uvs;` and its per vertex color by `in vec4 col;` to the shader source code.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self> {
        Ok(Self {
            program: Program::from_source(
                context,
                &Mesh::vertex_shader_source(&fragment_shader_source),
                fragment_shader_source,
            )?,
        })
    }
}

impl std::ops::Deref for MeshProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.program
    }
}

///
/// A triangle mesh which can be rendered with a custom [MeshProgram](MeshProgram).
///
#[derive(Clone)]
pub struct Mesh {
    context: Context,
    position_buffer: Rc<VertexBuffer>,
    normal_buffer: Option<Rc<VertexBuffer>>,
    index_buffer: Option<Rc<ElementBuffer>>,
    uv_buffer: Option<Rc<VertexBuffer>>,
    color_buffer: Option<Rc<VertexBuffer>>,
    pub(crate) transparent: bool,
    aabb: AxisAlignedBoundingBox,
    /// Optional name of the mesh.
    pub name: String,
    transformation: Mat4,
    normal_transformation: Mat4,
}

impl Mesh {
    ///
    /// Copies the per vertex data defined in the given [CPUMesh](crate::CPUMesh) to the GPU, thereby
    /// making it possible to render the mesh.
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
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
        let mut transparent = false;
        let color_buffer = if let Some(ref colors) = cpu_mesh.colors {
            for i in 0..colors.len() / 4 {
                if colors[i * 4] != 255 {
                    transparent = true;
                    break;
                }
            }
            Some(Rc::new(VertexBuffer::new_with_static(context, colors)?))
        } else {
            None
        };
        Ok(Self {
            context: context.clone(),
            position_buffer,
            normal_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            transparent,
            aabb: cpu_mesh.compute_aabb(),
            name: cpu_mesh.name.clone(),
            transformation: Mat4::identity(),
            normal_transformation: Mat4::identity(),
        })
    }

    ///
    /// Returns the local to world transformation of this mesh.
    ///
    pub fn transformation(&self) -> &Mat4 {
        &self.transformation
    }

    ///
    /// Set the local to world transformation of this mesh.
    ///
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
    pub fn render(
        &self,
        render_states: RenderStates,
        program: &Program,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> Result<()> {
        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
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
            program.use_uniform_mat4("normalMatrix", &self.normal_transformation)?;
            program.use_attribute_vec3("normal", normal_buffer)?;
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

    ///
    /// Computes the axis aligned bounding box for this mesh.
    ///
    pub fn aabb(&self) -> AxisAlignedBoundingBox {
        let mut aabb = self.aabb.clone();
        aabb.transform(&self.transformation);
        aabb
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
            include_str!("../shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
    }
}
