use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;
use crate::object::mesh::*;

///
/// A shader program used for rendering one or more instances of a [InstancedMesh](InstancedMesh). It has a fixed vertex shader and
/// customizable fragment shader for custom lighting. Use this in combination with [render](InstancedMesh::render).
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
/// Similar to [Mesh](crate::Mesh), except it is possible to render many instances of the same triangle mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    color_buffer: Option<VertexBuffer>,
    instance_count: u32,
    instance_buffer1: VertexBuffer,
    instance_buffer2: VertexBuffer,
    instance_buffer3: VertexBuffer,
    pub cull: CullType,
    pub transformation: Mat4,
}

impl InstancedMesh {
    ///
    /// Constructs a new InstancedMesh from the given [CPUMesh](crate::CPUMesh). The mesh is rendered
    /// in as many instances as there are transformation matrices in the transformations parameter.
    /// Each instance is transformed with the given transformation before it is rendered.
    /// The transformations can be updated by the [update_transformations](Self::update_transformations) function.
    ///
    pub fn new(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
    ) -> Result<Self, Error> {
        let position_buffer = VertexBuffer::new_with_static_f32(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(VertexBuffer::new_with_static_f32(context, normals)?)
        } else {
            None
        };
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices {
            Some(ElementBuffer::new_with_u32(context, ind)?)
        } else {
            None
        };
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(VertexBuffer::new_with_static_f32(context, uvs)?)
        } else {
            None
        };
        let color_buffer = if let Some(ref colors) = cpu_mesh.colors {
            Some(VertexBuffer::new_with_static_u8(context, colors)?)
        } else {
            None
        };

        let mut mesh = Self {
            context: context.clone(),
            instance_count: 0,
            position_buffer,
            normal_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            instance_buffer1: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            instance_buffer2: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            instance_buffer3: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            cull: CullType::None,
            transformation: Mat4::identity(),
        };
        mesh.update_transformations(transformations);
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(mesh)
    }

    ///
    /// Render the instanced mesh with a color per triangle vertex. The colors are defined when constructing the instanced mesh.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced mesh.
    ///
    /// # Errors
    /// Will return an error if the instanced mesh has no colors.
    ///
    pub fn render_color(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_PER_VERTEX_COLOR.is_none() {
                PROGRAM_PER_VERTEX_COLOR = Some(InstancedMeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_vertex_color.frag"),
                )?);
            }
            PROGRAM_PER_VERTEX_COLOR.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, camera)
    }

    ///
    /// Render the instanced mesh with the given color.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced mesh.
    ///
    pub fn render_with_color(
        &self,
        color: &Vec4,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_COLOR.is_none() {
                PROGRAM_COLOR = Some(InstancedMeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_color.frag"),
                )?);
            }
            PROGRAM_COLOR.as_ref().unwrap()
        };
        program.use_uniform_vec4("color", color)?;
        self.render(program, render_states, viewport, camera)
    }

    ///
    /// Render the instanced mesh with the given texture.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced mesh.
    ///
    /// # Errors
    /// Will return an error if the instanced mesh has no uv coordinates.
    ///
    pub fn render_with_texture(
        &self,
        texture: &dyn Texture,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_TEXTURE.is_none() {
                PROGRAM_TEXTURE = Some(InstancedMeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_texture.frag"),
                )?);
            }
            PROGRAM_TEXTURE.as_ref().unwrap()
        };
        program.use_texture(texture, "tex")?;
        self.render(program, render_states, viewport, camera)
    }

    ///
    /// Render the instanced mesh with the given [InstancedMeshProgram](InstancedMeshProgram).
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
        program: &InstancedMeshProgram,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        program.use_attribute_vec4_divisor(&self.instance_buffer1, "row1", 1)?;
        program.use_attribute_vec4_divisor(&self.instance_buffer2, "row2", 1)?;
        program.use_attribute_vec4_divisor(&self.instance_buffer3, "row3", 1)?;

        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");

        program.use_attribute_vec3(&self.position_buffer, "position")?;
        if program.mesh_program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(Error::MeshError {
                message:
                    "The mesh shader program needs uv coordinates, but the mesh does not have any."
                        .to_string(),
            })?;
            program.use_attribute_vec2(uv_buffer, "uv_coordinates")?;
        }
        if program.mesh_program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.use_uniform_mat4(
                "normalMatrix",
                &self.transformation.invert().unwrap().transpose(),
            )?;
            program.use_attribute_vec3(normal_buffer, "normal")?;
        }
        if program.mesh_program.use_colors {
            let color_buffer = self.color_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs per vertex colors, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec4(color_buffer, "color")?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(
                render_states,
                self.cull,
                viewport,
                index_buffer,
                self.instance_count,
            );
        } else {
            program.draw_arrays_instanced(
                render_states,
                self.cull,
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
        self.instance_buffer1.fill_with_dynamic_f32(&row1);
        self.instance_buffer2.fill_with_dynamic_f32(&row2);
        self.instance_buffer3.fill_with_dynamic_f32(&row3);
    }
}

impl Geometry for InstancedMesh {
    fn render_depth_to_red(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_PICK.is_none() {
                PROGRAM_PICK = Some(InstancedMeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_pick.frag"),
                )?);
            }
            PROGRAM_PICK.as_ref().unwrap()
        };
        program.use_uniform_float("maxDistance", &max_depth)?;
        self.render(program, render_states, viewport, camera)?;
        Ok(())
    }

    fn render_depth(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_DEPTH.is_none() {
                PROGRAM_DEPTH = Some(InstancedMeshProgram::new(&self.context, "void main() {}")?);
            }
            PROGRAM_DEPTH.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, camera)
    }

    fn aabb(&self) -> Option<AxisAlignedBoundingBox> {
        None // TODO: Compute bounding box
    }
}

impl Drop for InstancedMesh {
    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_DEPTH = None;
                PROGRAM_PICK = None;
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
                PROGRAM_PER_VERTEX_COLOR = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<InstancedMeshProgram> = None;
static mut PROGRAM_TEXTURE: Option<InstancedMeshProgram> = None;
static mut PROGRAM_DEPTH: Option<InstancedMeshProgram> = None;
static mut PROGRAM_PICK: Option<InstancedMeshProgram> = None;
static mut PROGRAM_PER_VERTEX_COLOR: Option<InstancedMeshProgram> = None;
static mut MESH_COUNT: u32 = 0;
