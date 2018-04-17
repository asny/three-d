use gl;
use dust::material;
use dust::mesh;
use dust::input;
use dust::material::Shade;

#[derive(Debug)]
pub enum Error {
    Material(material::Error)
}

impl From<material::Error> for Error {
    fn from(other: material::Error) -> Self {
        Error::Material(other)
    }
}

pub struct Model {
    gl: gl::Gl,
    id: gl::types::GLuint,
    material: material::Material
}

impl Model
{
    pub fn create(gl: &gl::Gl, material: &material::Material, mesh: &mesh::Mesh) -> Result<Model, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        material.program().add_vertex_attribute("Position", mesh.positions());

        Ok(Model { gl: gl.clone(), id: vao, material: material.clone() })
    }

    pub fn add_custom_attribute(&self, name: &str, data: &Vec<f32>) -> Result<(), Error>
    {
        self.material.program().add_vertex_attribute(name, data);
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        self.material.setup_uniforms(&input)?;
        self.material.apply();
        unsafe {
            self.gl.BindVertexArray(self.id);
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
        Ok(())
    }
}
