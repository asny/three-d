
use crate::core::*;

pub struct Mesh {
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

        Ok(Mesh {position_buffer, normal_buffer, index_buffer, uv_buffer})
    }

    pub fn create_program(context: &Context, fragment_shader_source: &str) -> Result<Program, Error>
    {
        Program::from_source(context, include_str!("shaders/mesh.vert"), fragment_shader_source)
    }

    pub fn has_uvs(&self) -> bool {
        self.uv_buffer.is_some()
    }

    pub fn has_normals(&self) -> bool {
        self.normal_buffer.is_some()
    }

    pub fn render(&self, program: &Program, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");

        program.use_attribute_vec3_float(&self.position_buffer, "position")?;
        if let Some(ref uv_buffer) = self.uv_buffer {
            program.use_attribute_vec2_float(uv_buffer, "uv_coordinates")?;
        }
        if let Some(ref normal_buffer) = self.normal_buffer {
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