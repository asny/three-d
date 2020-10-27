
use crate::*;

pub struct TexturedMesh {
    program: program::Program,
    pub position_buffer: VertexBuffer,
    pub normal_buffer: VertexBuffer,
    pub uv_buffer: Option<VertexBuffer>,
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
        let uv_buffer = if uvs.len() == 0 {None} else { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) };
        let index_buffer = ElementBuffer::new_with_u32(gl, indices)?;

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/textured.frag"))?;

        Ok(Self { index_buffer, position_buffer, normal_buffer, uv_buffer, program, texture,
            diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 6.0 })
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        self.program.use_texture(&self.texture,"tex").unwrap();

        if let Some(ref buffer) = self.uv_buffer
        {
            self.program.use_attribute_vec3_float(buffer, "uv_coordinates").unwrap();
            self.program.add_uniform_int("use_uvs", &1).unwrap();
        }
        else {
            self.program.add_uniform_int("use_uvs", &0).unwrap();
        }

        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.use_attribute_vec3_float(&self.normal_buffer, "normal").unwrap();

        self.program.draw_elements(&self.index_buffer);
    }
}