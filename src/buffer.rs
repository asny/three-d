use gl;
use std;

#[derive(Debug)]
pub enum Error {

}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: u32,
}

impl VertexBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = VertexBuffer{gl: gl.clone(), id };
        bind(&buffer.gl, buffer.id, gl::ARRAY_BUFFER);
        Ok(buffer)
    }

    pub fn fill_with(&self, data: &Vec<f32>)
    {
        bind(&self.gl, self.id, gl::ARRAY_BUFFER);
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
        }
    }
}


pub struct ElementBuffer {
    gl: gl::Gl,
    id: u32,
}

impl ElementBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<ElementBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = ElementBuffer{gl: gl.clone(), id };
        bind(&buffer.gl, buffer.id, gl::ELEMENT_ARRAY_BUFFER);
        Ok(buffer)
    }

    pub fn fill_with(&self, data: &Vec<u32>)
    {
        bind(&self.gl, self.id, gl::ELEMENT_ARRAY_BUFFER);
        unsafe {
            self.gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
        }
    }
}



fn bind(gl: &gl::Gl, id: u32, buffer_type: u32)
{
    unsafe {
        static mut CURRENTLY_USED: u32 = std::u32::MAX;
        if id != CURRENTLY_USED
        {
            gl.BindBuffer(buffer_type, id);
            CURRENTLY_USED = id;
        }
    }
}
