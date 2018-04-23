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
        Ok(VertexBuffer{gl: gl.clone(), id: id})
    }

    pub fn fill_with(&self, data: &Vec<f32>)
    {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id);
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
        }
    }
}
