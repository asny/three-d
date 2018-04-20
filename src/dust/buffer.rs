use gl;

#[derive(Debug)]
pub enum Error {

}

pub struct Buffer {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl Buffer
{
    pub fn create_vertex_buffer(gl: &gl::Gl) -> Result<Buffer, Error>
    {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        Ok(Buffer{gl: gl.clone(), id: id})
    }

    pub fn bind(&self)
    {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self)
    {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
