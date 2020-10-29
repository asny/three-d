
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TexturedMesh {
    program: program::Program,
    pub position_buffer: VertexBuffer,
    pub normal_buffer: VertexBuffer,
    pub uv_buffer: VertexBuffer,
    pub index_buffer: ElementBuffer,
    pub texture: texture::Texture2D,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl TexturedMesh
{
    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32], uvs: &[f32], texture: texture::Texture2D) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl, normals)?;
        let uv_buffer = VertexBuffer::new_with_static_f32(gl, uvs)?;
        let index_buffer = ElementBuffer::new_with_u32(gl, indices)?;

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/textured.frag"))?;

        Ok(Self { index_buffer, position_buffer, normal_buffer, uv_buffer, program, texture,
            diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 6.0 })
    }

    pub fn from_file(gl: &Gl, path: &'static str, texture: texture::Texture2D) -> Rc<RefCell<Self>> {
        Self::from_file_with_mapping(gl, path, texture, |_| {})
    }

    pub fn from_file_with_mapping<F: 'static>(gl: &Gl, path: &'static str, texture: texture::Texture2D, mapping: F) -> Rc<RefCell<Self>>
        where F: Fn(CPUMesh)
    {
        let m = Rc::new(RefCell::new(Self::new(gl, &[], &[], &[], &[], texture).unwrap()));
        let clone = m.clone();
        CPUMesh::from_file_with_mapping(path, move |cpu_mesh| {
            m.borrow_mut().index_buffer.fill_with_u32(&cpu_mesh.indices);
            m.borrow_mut().position_buffer.fill_with_static_f32(&cpu_mesh.positions);
            m.borrow_mut().normal_buffer.fill_with_static_f32(&cpu_mesh.normals);
            m.borrow_mut().uv_buffer.fill_with_static_f32(&cpu_mesh.uvs);
            mapping(cpu_mesh);
        });
        clone
    }

    pub fn from_cpu_mesh(gl: &Gl, cpu_mesh: &CPUMesh, texture: texture::Texture2D) -> Result<Self, Error> {
        Self::new(gl, &cpu_mesh.indices, &cpu_mesh.positions, &cpu_mesh.normals, &cpu_mesh.uvs, texture)
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        self.program.use_texture(&self.texture,"tex").unwrap();

        self.program.add_uniform_int("use_uvs", &(if self.uv_buffer.count() > 0 {1} else {0})).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(&self.uv_buffer, "uv_coordinates").unwrap();
        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.use_attribute_vec3_float(&self.normal_buffer, "normal").unwrap();

        self.program.draw_elements(&self.index_buffer);
    }
}