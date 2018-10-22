use gl;
use std;
use core::renderable::Renderable;
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
    count: usize
}

impl TriangleSurface
{
    pub fn create(gl: &gl::Gl, mesh: &Renderable) -> Result<TriangleSurface, Error>
    {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }

        let indices = mesh.indices();
        let model = TriangleSurface { gl: gl.clone(), id, count: indices.len() };
        model.bind();

        let index_buffer = buffer::ElementBuffer::create(&gl)?;
        index_buffer.fill_with(indices);

        Ok(model)
    }

    pub fn add_attributes(&mut self, mesh: &Renderable, program: &program::Program, attribute_names: &Vec<&str>) -> Result<buffer::VertexBuffer, Error>
    {
        // Create buffer
        let mut buffer = buffer::VertexBuffer::create(&self.gl)?;

        // Add data to the buffer
        buffer.fill_from_attributes(mesh, attribute_names)?;

        // Link data and program
        program.setup_attributes(&buffer)?;

        Ok(buffer)
    }

    pub fn render(&self) -> Result<(), Error>
    {
        self.bind();
        unsafe {
            self.gl.DrawElements(
                gl::TRIANGLES, // mode
                self.count as i32, // number of indices to be rendered
                gl::UNSIGNED_INT,
                std::ptr::null() // starting index in the enabled arrays
            );
        }
        Ok(())
    }

    pub fn render_instances(&self, no_instances: usize) -> Result<(), Error>
    {
        self.bind();
        unsafe {
            self.gl.DrawElementsInstanced(
                gl::TRIANGLES, // mode
                self.count as i32, // number of indices to be rendered
                gl::UNSIGNED_INT,
                std::ptr::null(), // starting index in the enabled arrays
                no_instances as i32
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