use gl;
use std;

#[derive(Debug)]
pub enum Error {
}

pub struct Model {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl Model
{
    pub fn create(gl: &gl::Gl) -> Result<Model, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        Ok(Model { gl: gl.clone(), id: vao })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        /*unsafe {
            //TODO:self.gl.DeleteProgram(self.id);
        }*/
    }
}
