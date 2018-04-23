use gl;
use std;

#[derive(Debug)]
pub enum Error {

}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl VertexBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = VertexBuffer{gl: gl.clone(), id };
        buffer.bind();
        Ok(buffer)
    }

    pub fn fill_with(&self, data: &Vec<f32>)
    {
        self.bind();
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
        }
    }

    fn bind(&self)
    {
        unsafe {
            static mut CURRENTLY_USED: gl::types::GLuint = std::u32::MAX;
            if self.id != CURRENTLY_USED
            {
                self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id);
                CURRENTLY_USED = self.id;
            }
        }
    }
}
