
use crate::math::*;
use crate::core::*;

pub struct InstancedMeshProgram {
    program: Program,
    use_normals: bool,
    use_uvs: bool,
}

impl InstancedMeshProgram {
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

                in vec4 row1;
                in vec4 row2;
                in vec4 row3;

                {} // Positions out
                {} // Normals in/out
                {} // UV coordinates in/out

                void main()
                {{
                    mat4 transform;
                    transform[0] = vec4(row1.x, row2.x, row3.x, 0.0);
                    transform[1] = vec4(row1.y, row2.y, row3.y, 0.0);
                    transform[2] = vec4(row1.z, row2.z, row3.z, 0.0);
                    transform[3] = vec4(row1.w, row2.w, row3.w, 1.0);

                    vec4 worldPosition = modelMatrix * transform * vec4(position, 1.);
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

impl std::ops::Deref for InstancedMeshProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.program
    }
}

///
/// Similar to [Mesh](crate::Mesh), except it is possible to draw many instances of the same triangle mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    instance_count: u32,
    instance_buffer1: VertexBuffer,
    instance_buffer2: VertexBuffer,
    instance_buffer3: VertexBuffer,
}

impl InstancedMesh
{
    pub fn new(context: &Context, transformations: &[Mat4], cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals { Some(VertexBuffer::new_with_static_f32(context, normals)?) } else {None};
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(context, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(context, uvs)?) } else {None};

        let mut mesh = Self { context: context.clone(), instance_count: 0,
            position_buffer, normal_buffer, index_buffer, uv_buffer,
            instance_buffer1: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            instance_buffer2: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            instance_buffer3: VertexBuffer::new_with_dynamic_f32(context, &[])?
        };
        mesh.update_transformations(transformations);
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(mesh)
    }

    pub fn render_depth(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = unsafe {
            if PROGRAM_DEPTH.is_none()
            {
                PROGRAM_DEPTH = Some(InstancedMeshProgram::new(&self.context, "void main() {}")?);
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
                PROGRAM_COLOR = Some(InstancedMeshProgram::new(&self.context, "
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
                PROGRAM_TEXTURE = Some(InstancedMeshProgram::new(&self.context, "
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

    pub fn render(&self, program: &InstancedMeshProgram, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.use_attribute_vec4_divisor(&self.instance_buffer1, "row1", 1)?;
        program.use_attribute_vec4_divisor(&self.instance_buffer2, "row2", 1)?;
        program.use_attribute_vec4_divisor(&self.instance_buffer3, "row3", 1)?;

        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");

        program.use_attribute_vec3(&self.position_buffer, "position")?;
        if program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(
                Error::FailedToCreateMesh {message: "The mesh shader program needs uv coordinates, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec2(uv_buffer, "uv_coordinates")?;
        }
        if program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::FailedToCreateMesh {message: "The mesh shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;
            program.use_attribute_vec3(normal_buffer, "normal")?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(render_states, viewport,index_buffer, self.instance_count);
        } else {
            program.draw_arrays_instanced(render_states, viewport,self.position_buffer.count() as u32/3, self.instance_count);
        }
        Ok(())
    }

    pub fn update_transformations(&mut self, transformations: &[Mat4])
    {
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

impl Drop for InstancedMesh {

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

static mut PROGRAM_COLOR: Option<InstancedMeshProgram> = None;
static mut PROGRAM_TEXTURE: Option<InstancedMeshProgram> = None;
static mut PROGRAM_DEPTH: Option<InstancedMeshProgram> = None;
static mut MESH_COUNT: u32 = 0;