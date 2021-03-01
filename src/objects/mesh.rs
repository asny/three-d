#[doc(hidden)]

use crate::math::*;
use crate::core::*;

pub struct MeshProgram {
    program: Program,
    use_normals: bool,
    use_uvs: bool,
}

impl MeshProgram {
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self, Error> {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let vertex_shader_source = &format!("
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

                {} // Positions out
                {} // Normals in/out
                {} // UV coordinates in/out

                void main()
                {{
                    vec4 worldPosition = modelMatrix * vec4(position, 1.);
                    gl_Position = camera.viewProjection * worldPosition;
                    {} // Position
                    {} // Normal
                    {} // UV coordinates
                }}
            ",
            if use_positions {"out vec3 pos;"} else {""},
            if use_normals {
                "uniform mat4 normalMatrix;
                in vec3 normal;
                out vec3 nor;"
            } else {""},
            if use_uvs {
                "in vec2 uv_coordinates;
                out vec2 uvs;"
            } else {""},
            if use_positions {"pos = worldPosition.xyz;"} else {""},
            if use_normals { "nor = mat3(normalMatrix) * normal;" } else {""},
            if use_uvs { "uvs = uv_coordinates;" } else {""}
        );

        let program = Program::from_source(context, vertex_shader_source, fragment_shader_source)?;
        Ok(Self {program, use_normals, use_uvs})
    }
}

impl std::ops::Deref for MeshProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.program
    }
}

///
/// A triangle mesh object with fixed vertex shader and customizable fragment shader for customizable lighting.
/// Supports rendering the depth and also with a fixed color and with a texture (ie. no lighting).
///
pub struct Mesh {
    context: Context,
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
}

impl Mesh {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals { Some(VertexBuffer::new_with_static_f32(context, normals)?) } else {None};
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(context, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(context, uvs)?) } else {None};
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Mesh {context: context.clone(), position_buffer, normal_buffer, index_buffer, uv_buffer})
    }

    pub fn render_depth(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = unsafe {
            if PROGRAM_DEPTH.is_none()
            {
                PROGRAM_DEPTH = Some(MeshProgram::new(&self.context, "void main() {}")?);
            }
            PROGRAM_DEPTH.as_ref().unwrap()
        };
        self.render(program, render_states, viewport, transformation, camera)
    }

    pub fn render_with_color(&self, color: &Vec4, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = unsafe {
            if PROGRAM_COLOR.is_none()
            {
                PROGRAM_COLOR = Some(MeshProgram::new(&self.context, "
                    uniform vec4 color;
                    layout (location = 0) out vec4 outColor;
                    void main()
                    {
                        outColor = color;
                    }")?);
            }
            PROGRAM_COLOR.as_ref().unwrap()
        };
        program.add_uniform_vec4("color", color)?;
        self.render(program, render_states, viewport, transformation, camera)
    }

    pub fn render_with_texture(&self, texture: &dyn Texture, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = unsafe {
            if PROGRAM_TEXTURE.is_none()
            {
                PROGRAM_TEXTURE = Some(MeshProgram::new(&self.context, "
                    uniform sampler2D tex;
                    in vec2 uvs;
                    layout (location = 0) out vec4 outColor;
                    void main()
                    {
                        outColor = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
                    }")?);
            }
            PROGRAM_TEXTURE.as_ref().unwrap()
        };
        program.use_texture(texture,"tex")?;
        self.render(program, render_states, viewport, transformation, camera)
    }

    pub fn render(&self, program: &MeshProgram, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");

        program.use_attribute_vec3_float(&self.position_buffer, "position")?;
        if program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(
                Error::FailedToCreateMesh {message: "The mesh shader program needs uv coordinates, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec2_float(uv_buffer, "uv_coordinates")?;
        }
        if program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::FailedToCreateMesh {message: "The mesh shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;
            program.use_attribute_vec3_float(normal_buffer, "normal")?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(render_states, viewport,index_buffer);
        } else {
            program.draw_arrays(render_states, viewport,self.position_buffer.count() as u32/3);
        }
        Ok(())
    }
}

impl Drop for Mesh {

    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_DEPTH = None;
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<MeshProgram> = None;
static mut PROGRAM_TEXTURE: Option<MeshProgram> = None;
static mut PROGRAM_DEPTH: Option<MeshProgram> = None;
static mut MESH_COUNT: u32 = 0;