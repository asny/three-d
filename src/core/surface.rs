use gl;
use std;
use glm;
use gust::mesh;
use core::buffer;
use core::program;

#[derive(Debug)]
pub enum Error {
    Buffer(buffer::Error),
    Program(program::Error)
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

pub struct TriangleSurface {
    gl: gl::Gl,
    id: gl::types::GLuint,
    no_indices: usize
}

impl TriangleSurface
{
    pub fn create(gl: &gl::Gl, mesh: &mesh::Mesh, program: &program::Program) -> Result<TriangleSurface, Error>
    {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }
        let model = TriangleSurface { gl: gl.clone(), id, no_indices: mesh.indices().len() };
        model.bind();

        let index_buffer = buffer::ElementBuffer::create(&gl)?;
        index_buffer.fill_with(&mesh.indices());

        program.add_attributes(&mesh.attributes)?;
        Ok(model)
    }

    pub fn update_attributes(&self) -> Result<(), Error>
    {
        // TODO: Update the attributes in the relevant vertex buffers
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error>
    {
        self.bind();
        unsafe {
            self.gl.DrawElements(
                gl::TRIANGLES, // mode
                self.no_indices as i32, // number of indices to be rendered
                gl::UNSIGNED_SHORT,
                std::ptr::null() // starting index in the enabled arrays
            );
        }
        Ok(())
    }

    fn bind(&self)
    {
        bind(&self.gl, self.id);
    }
}

fn bind(gl: &gl::Gl, id: u32)
{
    unsafe {
        static mut CURRENTLY_USED: gl::types::GLuint = std::u32::MAX;
        if id != CURRENTLY_USED
        {
            gl.BindVertexArray(id);
            CURRENTLY_USED = id;
        }
    }
}