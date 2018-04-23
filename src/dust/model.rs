use gl;
use std;
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
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }
        let model = Model { gl: gl.clone(), id, material, mesh };
        model.bind();
        model.material.setup_attributes(&model.mesh)?;
        Ok(model)
    }

    pub fn update_attributes(&self) -> Result<(), Error>
    {
        // TODO: Update the attributes in the relevant vertex buffers
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        self.material.apply();
        self.material.setup_states()?;
        self.material.setup_uniforms(&input)?;

        self.bind();
        unsafe {
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
        Ok(())
    }

    fn bind(&self)
    {
        unsafe {
            static mut CURRENTLY_USED: gl::types::GLuint = std::u32::MAX;
            if self.id != CURRENTLY_USED
            {
                self.gl.BindVertexArray(self.id);
                CURRENTLY_USED = self.id;
            }
        }
    }

}
