
use gl;
use crate::*;
use crate::surface::*;

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
    pub fn new(gl: &gl::Gl, indices: &[u32], attributes: &[Attribute]) -> Result<ShadedMesh, Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/shaded.frag"))?;
        let mut model = surface::TriangleSurface::new(gl, indices)?;
        let buffer = model.add_attributes(&program, attributes)?;

        Ok(ShadedMesh { program, model, buffer, color: vec3(1.0, 1.0, 1.0), texture: None, diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 })
    }

    pub fn new_from_obj_source(gl: &gl::Gl, source: String) -> Result<ShadedMesh, Error>
    {
        let objs = wavefront_obj::obj::parse(source).unwrap();
        let obj = objs.objects.first().unwrap();

        let mut positions = Vec::new();
        obj.vertices.iter().for_each(|v| {positions.push(v.x as f32); positions.push(v.y as f32); positions.push(v.z as f32);});
        let mut normals = vec![0.0f32; obj.vertices.len()*3];
        let mut indices = Vec::new();
        for shape in obj.geometry.first().unwrap().shapes.iter() {
            match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                    indices.push(i0.0 as u32);
                    indices.push(i1.0 as u32);
                    indices.push(i2.0 as u32);

                    let mut normal = obj.normals[i0.2.unwrap()];
                    normals[i0.0*3] = normal.x as f32;
                    normals[i0.0*3+1] = normal.y as f32;
                    normals[i0.0*3+2] = normal.z as f32;

                    normal = obj.normals[i1.2.unwrap()];
                    normals[i1.0*3] = normal.x as f32;
                    normals[i1.0*3+1] = normal.y as f32;
                    normals[i1.0*3+2] = normal.z as f32;

                    normal = obj.normals[i2.2.unwrap()];
                    normals[i2.0*3] = normal.x as f32;
                    normals[i2.0*3+1] = normal.y as f32;
                    normals[i2.0*3+2] = normal.z as f32;
                },
                _ => {}
            }
        }
        Self::new(&gl, &indices, &att!["position" => (positions, 3), "normal" => (normals, 3)])
    }

    pub fn update_attributes(&mut self, attributes: &[Attribute]) -> Result<(), Error>
    {
        self.buffer.fill_from_attributes(attributes)?;
        Ok(())
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
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
        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
