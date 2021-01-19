use crate::*;

pub struct ParticleData {
    pub start_position: Vec3,
    pub start_velocity: Vec3
}

pub struct Particles {
    gl: Gl,
    program_color_ambient: program::Program,
    program_texture_ambient: program::Program,
    start_position_buffer: VertexBuffer,
    start_velocity_buffer: VertexBuffer,
    position_buffer: VertexBuffer,
    index_buffer: Option<ElementBuffer>,
    pub material: PhongMaterial,
    pub acceleration: Vec3,
    instance_count: u32
}

impl Particles {
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial, acceleration: &Vec3) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};

        Ok(Self {gl: gl.clone(),
            program_color_ambient: Program::from_source(gl, include_str!("shaders/particles.vert"),
                                                              include_str!("shaders/particles.frag"))?,
            program_texture_ambient: Program::from_source(gl, include_str!("shaders/particles.vert"),
                                                              include_str!("shaders/particles.frag"))?,
            position_buffer, index_buffer,
            start_position_buffer: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            start_velocity_buffer: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            material: material.clone(),
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

    pub fn render_with_ambient(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, time: f32) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => &self.program_color_ambient,
            ColorSource::Texture(_) => &self.program_texture_ambient
        };

        let ambient = ambient_light.intensity() * ambient_light.color();
        match self.material.color_source {
            ColorSource::Color(ref color) => {
                program.add_uniform_vec4("color", &vec4(color.x * ambient.x, color.y * ambient.y, color.z * ambient.z, color.w))?;
            },
            ColorSource::Texture(ref texture) => {
                program.add_uniform_vec3("ambientColor", &ambient)?;
                program.use_texture(texture.as_ref(),"tex")?;
                //program.use_attribute_vec2_float(uv_buffer, "uv_coordinates")?;
            }
        }

        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.add_uniform_vec3("acceleration", &self.acceleration)?;
        program.add_uniform_float("time", &time)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");

        program.use_attribute_vec3_float_divisor(&self.start_position_buffer, "start_position", 1)?;
        program.use_attribute_vec3_float_divisor(&self.start_velocity_buffer, "start_velocity", 1)?;
        program.use_attribute_vec3_float(&self.position_buffer, "position")?;

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(index_buffer, self.instance_count);
        } else {
            program.draw_arrays_instanced(self.position_buffer.count() as u32/3, self.instance_count);
        }
        Ok(())
    }
}