use gl;
use std::rc::Rc;
use dust::material;
use dust::mesh;
use dust::input;

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
    material: Rc<material::Material>,
    mesh: mesh::Mesh
}

impl Model
{
    pub fn create(gl: &gl::Gl, material: Rc<material::Material>, mesh: mesh::Mesh) -> Result<Model, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }
        material.setup_attributes(&mesh)?;
        let model = Model { gl: gl.clone(), id: vao, material: material.clone(), mesh: mesh };

        Ok(model)
    }

    pub fn update_attributes(&self) -> Result<(), Error>
    {
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        self.material.apply();
        self.material.setup_uniforms(&input)?;
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
