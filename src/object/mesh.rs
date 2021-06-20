use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;
use crate::object::*;
use crate::shading::*;
use crate::Color;
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
    /// Constructs a new shader program for rendering meshes. The fragment shader can use the fragments position in world space by adding `in vec3 pos;`,
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
            "{}{}{}{}{}{}{}",
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
            if instanced { "#define INSTANCED\n" } else { "" },
            include_str!("../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
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
    pub material: Material,
    pub lighting_model: LightingModel,
}

impl Mesh {
    fn validate(cpu_mesh: &CPUMesh) -> Result<(), Error> {
        if let Some(ref indices) = cpu_mesh.indices {
            let index_count = match indices {
                Indices::U8(ind) => ind.len(),
                Indices::U16(ind) => ind.len(),
                Indices::U32(ind) => ind.len(),
            };
            if index_count % 3 != 0 {
                return Err(Error::MeshError {
                    message: format!(
                        "element count in indices of mesh `{}` \
                            must be divisible by 3, actual count is {}",
                        cpu_mesh.name, index_count
                    ),
                });
            }
            if cpu_mesh.positions.len() % 3 != 0 {
                return Err(Error::MeshError {
                    message: format!(
                        "when indices specified, element count in positions of mesh `{}` \
                            must be divisible by 3, actual count is {}",
                        cpu_mesh.name,
                        cpu_mesh.positions.len()
                    ),
                });
            }
            if cfg!(debug) {
                let indices_valid = match indices {
                    Indices::U8(ind) => {
                        let len = cpu_mesh.positions.len();
                        ind.iter().all(|&i| (i as usize) < len)
                    }
                    Indices::U16(ind) => {
                        let len = cpu_mesh.positions.len();
                        ind.iter().all(|&i| (i as usize) < len)
                    }
                    Indices::U32(ind) => {
                        let len = cpu_mesh.positions.len();
                        ind.iter().all(|&i| (i as usize) < len)
                    }
                };
                if !indices_valid {
                    return Err(Error::MeshError {
                        message: format!(
                            "some indices of mesh `{}` \
                                are outside of valid number of positions, which is {}",
                            cpu_mesh.name,
                            cpu_mesh.positions.len()
                        ),
                    });
                }
            }
        } else {
            if cpu_mesh.positions.len() % 9 != 0 {
                return Err(Error::MeshError {
                    message: format!(
                        "when indices unspecified, element count in positions of mesh `{}` \
                            must be divisible by 9, actual count is {}",
                        cpu_mesh.name,
                        cpu_mesh.positions.len()
                    ),
                });
            }
        };
        Ok(())
    }

    ///
    /// Copies the per vertex data defined in the given [CPUMesh](crate::CPUMesh) to the GPU, thereby
    /// making it possible to render the mesh.
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self, Error> {
        Self::validate(cpu_mesh)?;

        let position_buffer = Rc::new(VertexBuffer::new_with_static(context, &cpu_mesh.positions)?);
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(Rc::new(VertexBuffer::new_with_static(context, normals)?))
        } else {
            None
        };
        let index_buffer = if let Some(ref indices) = cpu_mesh.indices {
            Some(Rc::new(match indices {
                Indices::U8(ind) => ElementBuffer::new(context, ind)?,
                Indices::U16(ind) => ElementBuffer::new(context, ind)?,
                Indices::U32(ind) => ElementBuffer::new(context, ind)?,
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
            material: Material::default(),
            lighting_model: LightingModel::Blinn,
        })
    }

    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CPUMesh,
        material: &Material,
    ) -> Result<Self, Error> {
        let mut mesh = Self::new(context, cpu_mesh)?;
        mesh.material = material.clone();
        Ok(mesh)
    }

    ///
    /// Render the mesh with a color per triangle vertex. The colors are defined when constructing the mesh and are assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no colors.
    ///
    pub fn render_color(&self, render_states: RenderStates, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(&format!(
            "{}{}",
            include_str!("../core/shared.frag"),
            include_str!("shaders/mesh_vertex_color.frag")
        ))?;
        self.render(program, render_states, camera)
    }

    ///
    /// Render the mesh with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render_with_color(
        &self,
        color: &Color,
        render_states: RenderStates,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_color.frag"))?;
        program.use_uniform_vec4("color", &color.to_vec4())?;
        self.render(program, render_states, camera)
    }

    ///
    /// Render the uv coordinates of the mesh in red (u) and green (v).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    pub fn render_uvs(&self, render_states: RenderStates, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_uvs.frag"))?;
        self.render(program, render_states, camera)
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
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_normals.frag"))?;
        self.render(program, render_states, camera)
    }

    ///
    /// Render the mesh with the given texture which is assumed to be in sRGB color space with or without an alpha channel.
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
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_texture.frag"))?;
        program.use_texture("tex", texture)?;
        self.render(program, render_states, camera)
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
        camera: &Camera,
    ) -> Result<(), Error> {
        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
        program.use_uniform_block("Camera", camera.uniform_buffer());

        program.use_attribute_vec3("position", &self.position_buffer)?;
        if program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(Error::MeshError {
                message:
                    "The mesh shader program needs uv coordinates, but the mesh does not have any."
                        .to_string(),
            })?;
            program.use_attribute_vec2("uv_coordinates", uv_buffer)?;
        }
        if program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.use_uniform_mat4(
                "normalMatrix",
                &self.transformation.invert().unwrap().transpose(),
            )?;
            program.use_attribute_vec3("normal", normal_buffer)?;
        }
        if program.use_colors {
            let color_buffer = self.color_buffer.as_ref().ok_or(
                Error::MeshError {message: "The mesh shader program needs per vertex colors, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec4("color", color_buffer)?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(render_states, self.cull, camera.viewport(), index_buffer);
        } else {
            program.draw_arrays(
                render_states,
                self.cull,
                camera.viewport(),
                self.position_buffer.count() as u32 / 3,
            );
        }
        Ok(())
    }

    pub(crate) fn get_or_insert_program(
        &self,
        fragment_shader_source: &str,
    ) -> Result<&MeshProgram, Error> {
        unsafe {
            if PROGRAMS.is_none() {
                PROGRAMS = Some(std::collections::HashMap::new());
            }
            if !PROGRAMS
                .as_ref()
                .unwrap()
                .contains_key(fragment_shader_source)
            {
                PROGRAMS.as_mut().unwrap().insert(
                    fragment_shader_source.to_string(),
                    MeshProgram::new(&self.context, fragment_shader_source)?,
                );
            };
            Ok(PROGRAMS
                .as_ref()
                .unwrap()
                .get(fragment_shader_source)
                .unwrap())
        }
    }
}

impl Geometry for Mesh {
    fn render_depth_to_red(
        &self,
        render_states: RenderStates,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_pick.frag"))?;
        program.use_uniform_float("maxDistance", &max_depth)?;
        self.render(program, render_states, camera)?;
        Ok(())
    }

    fn render_depth(&self, render_states: RenderStates, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program("void main() {}")?;
        self.render(program, render_states, camera)
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
            material: self.material.clone(),
            lighting_model: self.lighting_model,
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAMS = None;
            }
        }
    }
}

static mut MESH_COUNT: u32 = 0;
static mut PROGRAMS: Option<std::collections::HashMap<String, MeshProgram>> = None;
