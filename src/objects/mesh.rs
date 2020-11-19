
use crate::*;

pub struct Mesh {
    program: program::Program,
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    index_buffer: Option<ElementBuffer>,
    pub color: Vec3,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Mesh
{
    pub fn from_cpu_mesh(gl: &Gl, cpu_mesh: &CPUMesh) -> Result<Self, Error> {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.normals)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices {
            Some(ElementBuffer::new_with_u32(gl, ind)?)
        } else {None};

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/shaded.frag"))?;

        Ok(Self { index_buffer, position_buffer, normal_buffer, program,
            color: cpu_mesh.color.unwrap_or(vec3(1.0, 1.0, 1.0)),
            diffuse_intensity: cpu_mesh.diffuse_intensity.unwrap_or(0.5),
            specular_intensity: cpu_mesh.specular_intensity.unwrap_or(0.2),
            specular_power: cpu_mesh.specular_power.unwrap_or(6.0) })
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();
        self.program.add_uniform_vec3("color", &self.color).unwrap();

        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.use_attribute_vec3_float(&self.normal_buffer, "normal").unwrap();

        if let Some(ref index_buffer) = self.index_buffer {
            self.program.draw_elements(index_buffer);
        } else {
            self.program.draw_arrays(self.position_buffer.count() as u32/3);
        }
    }
}
