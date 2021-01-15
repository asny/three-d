use crate::*;

pub struct Particles {
    gl: Gl,
    program_color_ambient: program::Program,
    program_texture_ambient: program::Program,
    start_position_buffer: VertexBuffer,
    mesh: GPUMesh,
    material: PhongMaterial,
    instance_count: u32
}

impl Particles {
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
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
            mesh: GPUMesh::new(gl, cpu_mesh)?,
            material: material.clone(),
            instance_count:0 })
    }

    pub fn update_positions(&mut self, positions: &[Vec3])
    {
        let mut buffer = Vec::new();
        for position in positions {
            buffer.push(position.x);
            buffer.push(position.y);
            buffer.push(position.z);
        }
        self.start_position_buffer.fill_with_dynamic_f32(&buffer);
        self.instance_count = positions.len() as u32;
    }

    pub fn render_with_ambient(&self, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => &self.program_color_ambient,
            ColorSource::Texture(_) => &self.program_texture_ambient
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        program.use_attribute_vec3_float_divisor(&self.start_position_buffer, "start_pos", 1)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        self.mesh.render(&program, &self.material, Some(self.instance_count))?;
        Ok(())
    }
}