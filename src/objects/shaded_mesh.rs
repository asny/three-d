
use gl;
use ::*;
use mesh::StaticMesh;

#[derive(Debug)]
pub enum Error {
    Buffer(buffer::Error),
    Program(program::Error),
    Surface(surface::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}

impl From<surface::Error> for Error {
    fn from(other: surface::Error) -> Self {
        Error::Surface(other)
    }
}

pub struct ShadedMesh {
    gl: gl::Gl,
    program: program::Program,
    model: surface::TriangleSurface,
    buffer: buffer::VertexBuffer,
    pub color: Vec3,
    pub texture: Option<texture::Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl ShadedMesh
{
    pub fn create(gl: &gl::Gl, mesh: &StaticMesh) -> Result<ShadedMesh, Error>
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/mesh_shaded",
                                                      "../Dust/src/objects/shaders/shaded")?;
        let mut model = surface::TriangleSurface::create(gl, mesh)?;
        let buffer = model.add_attributes(mesh, &program, &vec!["position", "normal"])?;

        Ok(ShadedMesh { gl: gl.clone(), program, model, buffer, color: vec3(1.0, 1.0, 1.0), texture: None, diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 })
    }

    pub fn update_attributes(&mut self, mesh: &StaticMesh) -> Result<(), Error>
    {
        self.buffer.fill_from_attributes(mesh, &vec!["position", "normal"])?;
        Ok(())
    }

    pub fn update_mesh(&mut self, mesh: &StaticMesh) -> Result<(), Error>
    {
        self.model = surface::TriangleSurface::create(&self.gl, mesh)?;
        self.buffer = self.model.add_attributes(mesh, &self.program, &vec!["position", "normal"])?;
        Ok(())
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        if let Some(ref tex) = self.texture
        {
            self.program.add_uniform_int("use_texture", &1).unwrap();
            tex.bind(0);
            self.program.add_uniform_int("tex", &0).unwrap();
        }
        else {
            self.program.add_uniform_int("use_texture", &0).unwrap();
            self.program.add_uniform_vec3("color", &self.color).unwrap();
        }

        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
