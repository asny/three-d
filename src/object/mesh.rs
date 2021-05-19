use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;
use std::rc::Rc;

///
/// A shader program used for rendering one or more instances of a [Mesh](Mesh). It has a fixed vertex shader and
/// customizable fragment shader for custom lighting. Use this in combination with [render](Mesh::render).
///
pub struct MeshProgram {
    program: Program,
    pub(in crate::object) use_normals: bool,
    pub(in crate::object) use_uvs: bool,
    pub(in crate::object) use_colors: bool,
}

impl MeshProgram {
    ///
    /// Constructs a new shader program for rendering meshes. The fragment shader can use the fragments position by adding `in vec3 pos;`,
    /// its normal by `in vec3 nor;`, its uv coordinates by `in vec2 uvs;` and its per vertex color by `in vec4 col;` to the shader source code.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self, Error> {
        Self::new_internal(context, fragment_shader_source, false)
    }

    pub(in crate::object) fn new_internal(
        context: &Context,
        fragment_shader_source: &str,
        instanced: bool,
    ) -> Result<Self, Error> {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        let vertex_shader_source = &format!(
            "
                layout (std140) uniform Camera
                {{
                    mat4 viewProjection;
                    mat4 view;
                    mat4 projection;
                    vec3 position;
                    float padding;
                }} camera;

                uniform mat4 modelMatrix;
                in vec3 position;

                {} // Instancing
                {} // Positions out
                {} // Normals in/out
                {} // UV coordinates in/out
                {} // Colors in/out

                void main()
                {{
                    mat4 local2World = modelMatrix;
                    {} // Instancing
                    vec4 worldPosition = local2World * vec4(position, 1.);
                    gl_Position = camera.viewProjection * worldPosition;
                    {} // Position
                    {} // Normal
                    {} // UV coordinates
                    {} // Colors
                }}
            ",
            if instanced {
                "in vec4 row1;
                in vec4 row2;
                in vec4 row3;"
            } else {
                ""
            },
            if use_positions { "out vec3 pos;" } else { "" },
            if use_normals {
                "uniform mat4 normalMatrix;
                in vec3 normal;
                out vec3 nor;"
            } else {
                ""
            },
            if use_uvs {
                "in vec2 uv_coordinates;
                out vec2 uvs;"
            } else {
                ""
            },
            if use_colors {
                "in vec4 color;
                out vec4 col;"
            } else {
                ""
            },
            if instanced {
                "
                    mat4 transform;
                    transform[0] = vec4(row1.x, row2.x, row3.x, 0.0);
                    transform[1] = vec4(row1.y, row2.y, row3.y, 0.0);
                    transform[2] = vec4(row1.z, row2.z, row3.z, 0.0);
                    transform[3] = vec4(row1.w, row2.w, row3.w, 1.0);
                    local2World *= transform;"
            } else {
                ""
            },
            if use_positions {
                "pos = worldPosition.xyz;"
            } else {
                ""
            },
            if use_normals {
                "nor = mat3(normalMatrix) * normal;"
            } else {
                ""
            },
            if use_uvs { "uvs = uv_coordinates;" } else { "" },
            if use_colors { "col = color;" } else { "" }
        );

        let program = Program::from_source(context, vertex_shader_source, fragment_shader_source)?;
        Ok(Self {
            program,
            use_normals,
            use_uvs,
            use_colors,
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
/// A triangle mesh which can be rendered with one of the default render functions or with a custom [MeshProgram](MeshProgram).
/// See also [PhongMesh](crate::PhongMesh) for rendering a mesh with lighting.
///
pub struct Mesh {
    context: Context,
    position_buffer: Rc<VertexBuffer>,
    normal_buffer: Option<Rc<VertexBuffer>>,
    index_buffer: Option<Rc<ElementBuffer>>,
    uv_buffer: Option<Rc<VertexBuffer>>,
    color_buffer: Option<Rc<VertexBuffer>>,
    aabb: AxisAlignedBoundingBox,
    pub name: String,
    pub cull: CullType,
    pub transformation: Mat4,
}

impl Mesh {
    ///
    /// Copies the per vertex data defined in the given [CPUMesh](crate::CPUMesh) to the GPU, thereby
    /// making it possible to render the mesh.
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self, Error> {
        let position_buffer = Rc::new(VertexBuffer::new_with_static_f32(
            context,
            &cpu_mesh.positions,
        )?);
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(Rc::new(VertexBuffer::new_with_static_f32(
                context, normals,
            )?))
        } else {
            None
        };
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices {
            Some(Rc::new(ElementBuffer::new_with_u32(context, ind)?))
        } else {
            None
        };
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(Rc::new(VertexBuffer::new_with_static_f32(context, uvs)?))
        } else {
            None
        };
        let color_buffer = if let Some(ref colors) = cpu_mesh.colors {
            Some(Rc::new(VertexBuffer::new_with_static_u8(context, colors)?))
        } else {
            None
        };
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Mesh {
            context: context.clone(),
            position_buffer,
            normal_buffer,
            index_buffer,
            uv_buffer,
            color_buffer,
            aabb: cpu_mesh.compute_aabb(),
            name: cpu_mesh.name.clone(),
            transformation: Mat4::identity(),
            cull: CullType::None,
        })
    }

    ///
    /// Render the mesh with a color per triangle vertex. The colors are defined when constructing the mesh.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no colors.
    ///
    pub fn render_color(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_PER_VERTEX_COLOR.is_none() {
                PROGRAM_PER_VERTEX_COLOR = Some(MeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_vertex_color.frag"),
                )?);
            }
            PROGRAM_PER_VERTEX_COLOR.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, camera)
    }

    ///
    /// Render the mesh with the given color.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
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
                PROGRAM_COLOR = Some(MeshProgram::new(
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
    /// Render the uv coordinates of the mesh in red (u) and green (v).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    pub fn render_uvs(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_UVS.is_none() {
                PROGRAM_UVS = Some(MeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_uvs.frag"),
                )?);
            }
            PROGRAM_UVS.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, camera)
    }

    ///
    /// Render the normals of the mesh.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no normals.
    ///
    pub fn render_normals(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_NORMALS.is_none() {
                PROGRAM_NORMALS = Some(MeshProgram::new(
                    &self.context,
                    include_str!("shaders/mesh_normals.frag"),
                )?);
            }
            PROGRAM_NORMALS.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, camera)
    }

    ///
    /// Render the mesh with the given texture.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    pub fn render_with_texture(
        &self,
        texture: &impl Texture,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_TEXTURE.is_none() {
                PROGRAM_TEXTURE = Some(MeshProgram::new(
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
    /// Render the mesh with the given [MeshProgram](MeshProgram).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh shader program requires a certain attribute and the mesh does not have that attribute.
    /// For example if the program needs the normal to calculate lighting, but the mesh does not have per vertex normals, this
    /// function will return an error.
    ///
    pub fn render(
        &self,
        program: &MeshProgram,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
        program.use_uniform_block(camera.uniform_buffer(), "Camera");

        program.use_attribute_vec3(&self.position_buffer, "position")?;
        if program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(Error::MeshError {
                message:
                    "The mesh shader program needs uv coordinates, but the mesh does not have any."
                        .to_string(),
            })?;
            program.use_attribute_vec2(uv_buffer, "uv_coordinates")?;
        }
        if program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.use_uniform_mat4(
                "normalMatrix",
                &self.transformation.invert().unwrap().transpose(),
            )?;
            program.use_attribute_vec3(normal_buffer, "normal")?;
        }
        if program.use_colors {
            let color_buffer = self.color_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs per vertex colors, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec4(color_buffer, "color")?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(render_states, self.cull, viewport, index_buffer);
        } else {
            program.draw_arrays(
                render_states,
                self.cull,
                viewport,
                self.position_buffer.count() as u32 / 3,
            );
        }
        Ok(())
    }
}

impl Geometry for Mesh {
    fn render_depth_to_red(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAM_PICK.is_none() {
                PROGRAM_PICK = Some(MeshProgram::new(
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
                PROGRAM_DEPTH = Some(MeshProgram::new(&self.context, "void main() {}")?);
            }
            PROGRAM_DEPTH.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, camera)
    }

    fn aabb(&self) -> Option<AxisAlignedBoundingBox> {
        let mut aabb = self.aabb.clone();
        aabb.transform(&self.transformation);
        Some(aabb)
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        unsafe {
            MESH_COUNT += 1;
        }
        Self {
            context: self.context.clone(),
            position_buffer: self.position_buffer.clone(),
            normal_buffer: self.normal_buffer.clone(),
            index_buffer: self.index_buffer.clone(),
            uv_buffer: self.uv_buffer.clone(),
            color_buffer: self.color_buffer.clone(),
            aabb: self.aabb.clone(),
            name: self.name.clone(),
            cull: self.cull.clone(),
            transformation: self.transformation.clone(),
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_DEPTH = None;
                PROGRAM_PICK = None;
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
                PROGRAM_UVS = None;
                PROGRAM_NORMALS = None;
                PROGRAM_PER_VERTEX_COLOR = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<MeshProgram> = None;
static mut PROGRAM_TEXTURE: Option<MeshProgram> = None;
static mut PROGRAM_DEPTH: Option<MeshProgram> = None;
static mut PROGRAM_UVS: Option<MeshProgram> = None;
static mut PROGRAM_NORMALS: Option<MeshProgram> = None;
static mut PROGRAM_PICK: Option<MeshProgram> = None;
static mut PROGRAM_PER_VERTEX_COLOR: Option<MeshProgram> = None;
static mut MESH_COUNT: u32 = 0;
