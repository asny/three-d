
use crate::*;
use std::rc::Rc;

pub enum ColorSource {
    Color(Vec3),
    Texture(Rc<Texture2D>)
}

pub struct Mesh {
    pub name: String,
    program_forward: program::Program,
    program_deferred: program::Program,
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    pub color: ColorSource,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Mesh
{
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let texture = if let Some(ref img) = cpu_mesh.texture {
            use image::GenericImageView;
            Some(Rc::new(texture::Texture2D::new_with_u8(&gl, Interpolation::Linear, Interpolation::Linear,
                                                                  Some(Interpolation::Linear), Wrapping::Repeat, Wrapping::Repeat,
                                                                  img.width(), img.height(), &img.to_bytes()).unwrap()))
        } else {None};
        Self::from_cpu_mesh_and_optional_texture(gl, cpu_mesh, texture)
    }

    pub fn new_with_texture(gl: &Gl, cpu_mesh: &CPUMesh, texture: Rc<Texture2D>) -> Result<Self, Error>
    {
        Self::from_cpu_mesh_and_optional_texture(gl, cpu_mesh, Some(texture))
    }

    fn from_cpu_mesh_and_optional_texture(gl: &Gl, cpu_mesh: &CPUMesh, texture: Option<Rc<Texture2D>>) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl,
              cpu_mesh.normals.as_ref().ok_or(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        let color = if let Some(tex) = texture {
            ColorSource::Texture(tex)
        } else {
            ColorSource::Color(cpu_mesh.color.map(|(r, g, b)| vec3(r, g, b)).unwrap_or(vec3(1.0, 1.0, 1.0)))
        };

        let program_forward = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    match color {
                                                        ColorSource::Color(_) => include_str!("shaders/shaded_forward.frag"),
                                                        ColorSource::Texture(_) => include_str!("shaders/textured_forward.frag")
                                                    })?;

        let program_deferred = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    match color {
                                                        ColorSource::Color(_) => include_str!("shaders/shaded.frag"),
                                                        ColorSource::Texture(_) => include_str!("shaders/textured.frag")
                                                    })?;

        Ok(Self { name: cpu_mesh.name.clone(), index_buffer, uv_buffer, position_buffer, normal_buffer, program_forward, program_deferred, color,
            diffuse_intensity: cpu_mesh.diffuse_intensity.unwrap_or(0.5),
            specular_intensity: cpu_mesh.specular_intensity.unwrap_or(0.2),
            specular_power: cpu_mesh.specular_power.unwrap_or(6.0) })
    }

    pub fn render_with_lighting(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.render_internal(&self.program_forward, transformation, camera);
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.render_internal(&self.program_deferred, transformation, camera);
    }

    fn render_internal(&self, program: &Program, transformation: &Mat4, camera: &camera::Camera)
    {
        program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        match self.color {
            ColorSource::Color(ref color) => {
                program.add_uniform_vec3("color", color).unwrap();
            },
            ColorSource::Texture(ref texture) => {
                program.use_texture(texture.as_ref(),"tex").unwrap();
                if let Some(ref uv_buffer) = self.uv_buffer {
                    program.add_uniform_int("use_uvs", &1).unwrap();
                    program.use_attribute_vec2_float(uv_buffer, "uv_coordinates").unwrap();
                } else {
                    program.add_uniform_int("use_uvs", &0).unwrap();
                }
            }
        }

        program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        program.use_attribute_vec3_float(&self.normal_buffer, "normal").unwrap();

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(index_buffer);
        } else {
            program.draw_arrays(self.position_buffer.count() as u32/3);
        }
    }
}
