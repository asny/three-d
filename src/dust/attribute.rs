use gl;
use std;
use dust::program;

#[derive(Debug)]
pub enum Error {
}

pub struct Attribute {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl Attribute
{
    pub fn create(gl: &gl::Gl, name: &str, program: &program::Program, data: Vec<f32>) -> Result<Attribute, Error>
    {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            use std::ffi::{CString};
            let location = gl.GetAttribLocation(program.id(), CString::new(name).unwrap().as_ptr()) as gl::types::GLuint;
            gl.EnableVertexAttribArray(location);
            gl.VertexAttribPointer(
                location, // index of the generic vertex attribute
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Ok(Attribute { gl: gl.clone(), id: vbo })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}
