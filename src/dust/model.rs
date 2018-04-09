use gl;
use material;

#[derive(Debug)]
pub enum Error {
}

pub struct Model {
    gl: gl::Gl,
    id: gl::types::GLuint,
    material: material::Material
}


impl Model
{
    pub fn create(gl: &gl::Gl, material: &material::Material) -> Result<Model, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        Ok(Model { gl: gl.clone(), id: vao, material: material.clone() })
    }

    pub fn draw(&self)
    {
        self.material.apply();
        unsafe {
            self.gl.BindVertexArray(self.id);
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
    }
}
