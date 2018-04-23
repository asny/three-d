use gl;
use std;
use std::rc::Rc;
use material;
use gust::mesh;
use input;

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
    pub fn create(gl: &gl::Gl, mesh: mesh::Mesh, material: Rc<material::Material>) -> Result<Model, Error>
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
        self.material.setup_states(&self.gl)?;
        self.material.setup_uniforms(&input)?;

        self.bind();
        let draw_mode = self.get_draw_mode();
        let no_indices = self.mesh.no_vertices();
        unsafe {
            self.gl.DrawArrays(
                draw_mode, // mode
                0, // starting index in the enabled arrays
                no_indices as i32 // number of indices to be rendered
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

    fn get_draw_mode(&self) -> u32
    {
        gl::TRIANGLES
    }
}
