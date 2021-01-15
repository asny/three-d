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
    mesh: GPUMesh,
    pub material: PhongMaterial,
    pub acceleration: Vec3,
    instance_count: u32
}

impl Particles {
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial, acceleration: &Vec3) -> Result<Self, Error>
    {
        Ok(Self {gl: gl.clone(),
            program_color_ambient: Program::from_source(gl, include_str!("shaders/particles.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient.frag")))?,
            program_texture_ambient: Program::from_source(gl, include_str!("shaders/particles.vert"),
                                                                     &format!("{}\n{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                              &include_str!("shaders/textured_forward_ambient.frag")))?,
            start_position_buffer: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            start_velocity_buffer: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            mesh: GPUMesh::new(gl, cpu_mesh)?,
            material: material.clone(),
            acceleration: *acceleration,
            instance_count:0 })
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

    pub fn render_with_ambient(&self, time: f32, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => &self.program_color_ambient,
            ColorSource::Texture(_) => &self.program_texture_ambient
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;

        program.add_uniform_vec3("acceleration", &self.acceleration)?;
        program.add_uniform_float("time", &time)?;

        program.use_attribute_vec3_float_divisor(&self.start_position_buffer, "start_position", 1)?;
        program.use_attribute_vec3_float_divisor(&self.start_velocity_buffer, "start_velocity", 1)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        self.mesh.render(&program, &self.material, Some(self.instance_count))?;
        Ok(())
    }
}