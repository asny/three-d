
use crate::*;

#[derive(Debug)]
pub enum Error {
    Program(program::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

pub struct ShadedMesh {
    program: program::Program,
    pub color: Vec3,
    pub texture: Option<texture::Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl ShadedMesh
{
    pub fn new(gl: &Gl) -> Result<ShadedMesh, Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/shaded.frag"))?;

        Ok(ShadedMesh { program, color: vec3(1.0, 1.0, 1.0), texture: None, diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 })
    }

    pub fn render(&self, mesh: &Mesh, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::NONE);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        if let Some(ref tex) = self.texture
        {
            self.program.add_uniform_int("use_texture", &1).unwrap();
            self.program.use_texture(tex,"tex").unwrap();
        }
        else {
            self.program.add_uniform_int("use_texture", &0).unwrap();
            self.program.add_uniform_vec3("color", &self.color).unwrap();
        }

        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(mesh.position_buffer(), "position", 0).unwrap();
        self.program.use_attribute_vec3_float(mesh.normal_buffer(), "normal", 0).unwrap();

        self.program.draw_elements(mesh.index_buffer());
    }
}
