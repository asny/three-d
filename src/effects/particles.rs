use crate::core::*;
use crate::scene::*;

pub struct ParticleData {
    pub start_position: Vec3,
    pub start_velocity: Vec3
}

pub struct Particles {
    program: program::Program,
    start_position_buffer: VertexBuffer,
    start_velocity_buffer: VertexBuffer,
    position_buffer: VertexBuffer,
    uv_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    pub acceleration: Vec3,
    instance_count: u32
}

impl Particles {
    pub fn new(gl: &Gl, fragment_shader: &str, cpu_mesh: &CPUMesh, acceleration: &Vec3) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        Ok(Self {
            program: Program::from_source(gl, include_str!("shaders/particles.vert"), fragment_shader)?,
            position_buffer, index_buffer, uv_buffer,
            start_position_buffer: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            start_velocity_buffer: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            acceleration: *acceleration,
            instance_count: 0
        })
    }

    pub fn update(&mut self, data: &[ParticleData])
    {
        let mut start_position = Vec::new();
        let mut start_velocity = Vec::new();
        for particle in data {
            start_position.push(particle.start_position.x);
            start_position.push(particle.start_position.y);
            start_position.push(particle.start_position.z);
            start_velocity.push(particle.start_velocity.x);
            start_velocity.push(particle.start_velocity.y);
            start_velocity.push(particle.start_velocity.z);
        }
        self.start_position_buffer.fill_with_dynamic_f32(&start_position);
        self.start_velocity_buffer.fill_with_dynamic_f32(&start_velocity);
        self.instance_count = data.len() as u32;
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn render(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera, time: f32) -> Result<(), Error>
    {
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.add_uniform_vec3("acceleration", &self.acceleration)?;
        self.program.add_uniform_float("time", &time)?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_attribute_vec3_float_divisor(&self.start_position_buffer, "start_position", 1)?;
        self.program.use_attribute_vec3_float_divisor(&self.start_velocity_buffer, "start_velocity", 1)?;
        self.program.use_attribute_vec3_float(&self.position_buffer, "position")?;
        if let Some(ref uv_buffer) = self.uv_buffer {
            self.program.use_attribute_vec2_float(uv_buffer, "uv_coordinates")?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            self.program.draw_elements_instanced(render_states, viewport,index_buffer, self.instance_count);
        } else {
            self.program.draw_arrays_instanced(render_states, viewport,self.position_buffer.count() as u32/3, self.instance_count);
        }
        Ok(())
    }
}