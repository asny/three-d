
use crate::*;
use std::rc::Rc;

pub struct TexturedMesh {
    program: program::Program,
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    uv_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    texture: Rc<texture::Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl TexturedMesh
{
    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32], uvs: &[f32], texture: Rc<Texture2D>) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl, normals)?;
        let uv_buffer = if uvs.len() > 0 { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else { None };
        let index_buffer = if indices.len() > 0 { Some(ElementBuffer::new_with_u32(gl, indices)?) } else { None };

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

        self.program.use_texture(self.texture.as_ref(),"tex").unwrap();

        self.program.add_uniform_int("use_uvs", &(if self.uv_buffer.is_some() {1} else {0})).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.use_attribute_vec3_float(&self.normal_buffer, "normal").unwrap();
        if let Some(ref uv_buffer) = self.uv_buffer {
            self.program.use_attribute_vec2_float(uv_buffer, "uv_coordinates").unwrap();
        }

        if let Some(ref index_buffer) = self.index_buffer {
            self.program.draw_elements(index_buffer);
        } else {
            self.program.draw_arrays(self.position_buffer.count() as u32/3);
        }
    }
}