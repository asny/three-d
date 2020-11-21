
use crate::*;
use std::rc::Rc;

pub struct TexturedMesh {
    program: program::Program,
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    uv_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    pub texture: Rc<Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl TexturedMesh
{
    pub fn from_cpu_mesh(gl: &Gl, loaded: &Loaded, cpu_mesh: &CPUMesh) -> Result<Self, Error> {

        let path = cpu_mesh.texture.ok_or(
            Error::FailedToCreateMesh {message:"Cannot create a textured mesh without a texture.".to_string()})?.clone();
        let bytes = loaded.get(path).ok_or(
            Error::FailedToCreateMesh {message:format!("Tried to use a texture which was not loaded: {}", path)})?.as_ref().map_err(
            |_| Error::FailedToCreateMesh {message:format!("Could not load texture: {}", path)})?;
        let texture = Rc::new(texture::Texture2D::new_from_bytes(&gl,
         Interpolation::Linear, Interpolation::Linear, Some(Interpolation::Linear),
       Wrapping::Repeat, Wrapping::Repeat, bytes).unwrap());
        Self::from_cpu_mesh_and_texture(gl, cpu_mesh, texture)
    }

    pub fn from_cpu_mesh_and_texture(gl: &Gl, cpu_mesh: &CPUMesh, texture: Rc<Texture2D>) -> Result<Self, Error> {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl,
              cpu_mesh.normals.as_ref().ok_or(Error::FailedToCreateMesh {message:
              "Cannot create a textured mesh without normals. Consider calling compute_normals on the CPUMesh before creating the textured mesh.".to_string()})?)?;
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(VertexBuffer::new_with_static_f32(gl, uvs)?)
        } else {None};
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices {
            Some(ElementBuffer::new_with_u32(gl, ind)?)
        } else {None};

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/textured.frag"))?;

        Ok(Self { index_buffer, position_buffer, normal_buffer, uv_buffer, program, texture,
            diffuse_intensity: cpu_mesh.diffuse_intensity.unwrap_or(0.5),
            specular_intensity: cpu_mesh.specular_intensity.unwrap_or(0.2),
            specular_power: cpu_mesh.specular_power.unwrap_or(6.0) })
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