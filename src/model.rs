use gl;
use std;
use std::rc::Rc;
use material;
use gust::mesh;
use input;
use buffer;

#[derive(Debug)]
pub enum Error {
    Material(material::Error),
    Buffer(buffer::Error)
}

impl From<material::Error> for Error {
    fn from(other: material::Error) -> Self {
        Error::Material(other)
    }
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
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

        let index_buffer = buffer::ElementBuffer::create(&model.gl)?;
        index_buffer.fill_with(&model.mesh.indices());

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
        unsafe {
            self.gl.DrawElements(
                draw_mode, // mode
                self.mesh.indices().len() as i32, // number of indices to be rendered
                gl::UNSIGNED_INT,
                std::ptr::null() // starting index in the enabled arrays
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
