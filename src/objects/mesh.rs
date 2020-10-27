
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Mesh {
    program: program::Program,
    pub position_buffer: VertexBuffer,
    pub normal_buffer: VertexBuffer,
    pub index_buffer: ElementBuffer,
    pub color: Vec3,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Mesh
{

    pub fn empty(gl: &Gl) -> Self
    {
        Self::new(gl, &[], &[], &[]).unwrap()
    }

    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl, normals)?;
        let index_buffer = ElementBuffer::new_with_u32(gl, indices)?;

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/shaded.frag"))?;

        Ok(Mesh { index_buffer, position_buffer, normal_buffer, program, color: vec3(1.0, 1.0, 1.0),
            diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 6.0 })
    }

    pub fn from_file(gl: &Gl, path: &'static str) -> Rc<RefCell<Mesh>> {
        let mesh = Rc::new(RefCell::new(Self::empty(gl)));
        CPUMesh::from_file_with_mapping(path, mesh.clone(), |cpu_mesh, mesh| {
            mesh.borrow_mut().index_buffer.fill_with_u32(&cpu_mesh.indices);
            mesh.borrow_mut().position_buffer.fill_with_static_f32(&cpu_mesh.positions);
            mesh.borrow_mut().normal_buffer.fill_with_static_f32(&cpu_mesh.normals);
        });
        mesh
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

        self.program.draw_elements(&self.index_buffer);
    }
}
