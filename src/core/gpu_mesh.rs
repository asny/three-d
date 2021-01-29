
use crate::*;

pub struct GPUMesh {
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
}

impl GPUMesh {
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals { Some(VertexBuffer::new_with_static_f32(gl, normals)?) } else {None};
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        Ok(GPUMesh {position_buffer, normal_buffer, index_buffer, uv_buffer})
    }

    pub fn create_program(gl: &Gl, fragment_shader_source: &str) -> Result<Program, Error>
    {
        Program::from_source(gl, include_str!("shaders/mesh.vert"), fragment_shader_source)
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

pub struct InstancedGPUMesh {
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    instance_count: u32,
    instance_buffer1: VertexBuffer,
    instance_buffer2: VertexBuffer,
    instance_buffer3: VertexBuffer,
}

impl InstancedGPUMesh
{
    pub fn new(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals { Some(VertexBuffer::new_with_static_f32(gl, normals)?) } else {None};
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        let mut mesh = Self { instance_count: 0,
            position_buffer, normal_buffer, index_buffer, uv_buffer,
            instance_buffer1: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            instance_buffer2: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            instance_buffer3: VertexBuffer::new_with_dynamic_f32(gl, &[])?
        };
        mesh.update_transformations(transformations);
        Ok(mesh)
    }

    pub fn create_program(gl: &Gl, fragment_shader_source: &str) -> Result<Program, Error>
    {
        Program::from_source(gl, include_str!("shaders/mesh_instanced.vert"), fragment_shader_source)
    }

    pub fn has_uvs(&self) -> bool {
        self.uv_buffer.is_some()
    }

    pub fn has_normals(&self) -> bool {
        self.normal_buffer.is_some()
    }

    pub fn render(&self, program: &Program, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.use_attribute_vec4_float_divisor(&self.instance_buffer1, "row1", 1)?;
        program.use_attribute_vec4_float_divisor(&self.instance_buffer2, "row2", 1)?;
        program.use_attribute_vec4_float_divisor(&self.instance_buffer3, "row3", 1)?;

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