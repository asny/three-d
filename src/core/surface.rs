use gl;
use crate::core::buffer;
use crate::core::program;
pub use crate::core::buffer::Attribute;

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
    id: gl::VertexArrayObject,
    count: usize
}

impl TriangleSurface
{
    pub fn new(gl: &gl::Gl, indices: &[u32]) -> Result<TriangleSurface, Error>
    {
        let id = gl.create_vertex_array().unwrap();

        let model = TriangleSurface { gl: gl.clone(), id, count: indices.len() };
        model.bind();

        let index_buffer = buffer::ElementBuffer::new(&gl)?;
        index_buffer.fill_with(indices);

        Ok(model)
    }

    pub fn add_attributes(&mut self, program: &program::Program, attributes: &[Attribute]) -> Result<buffer::VertexBuffer, Error>
    {
        // Create buffer
        let mut buffer = buffer::VertexBuffer::new(&self.gl)?;

        // Add data to the buffer
        buffer.fill_from_attributes(attributes)?;

        // Link data and program
        program.setup_attributes(&buffer)?;

        Ok(buffer)
    }

    pub fn render(&self) -> Result<(), Error>
    {
        self.bind();
        self.gl.draw_elements(gl::consts::TRIANGLES, self.count as u32, gl::consts::UNSIGNED_INT, 0);
        Ok(())
    }

    pub fn render_instances(&self, no_instances: usize) -> Result<(), Error>
    {
        self.bind();
        self.gl.draw_elements_instanced(gl::consts::TRIANGLES, self.count as u32, gl::consts::UNSIGNED_INT, 0, no_instances as u32);
        Ok(())
    }

    fn bind(&self)
    {
        self.gl.bind_vertex_array(&self.id);
    }
}