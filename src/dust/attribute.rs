use gl;
use std;

#[derive(Debug)]
pub enum Error {
}

pub struct Attribute {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl Attribute
{
    pub fn create(gl: &gl::Gl) -> Result<Attribute, Error>
    {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Ok(Attribute { gl: gl.clone(), id: vbo })
    }

    pub fn populate(&self, vertices: Vec<f32>)
    {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id);
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Attribute {
    fn drop(&mut self) {
        /*unsafe {
            //TODO:self.gl.DeleteProgram(self.id);
        }*/
    }
}
