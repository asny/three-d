use gl;
use material;

#[derive(Debug)]
pub enum Error {
}

pub struct Model<'a> {
    gl: gl::Gl,
    id: gl::types::GLuint,
    material: &'a material::Material
}


impl<'a> Model<'a>
{
    pub fn create(gl: &gl::Gl, material: &'a material::Material) -> Result<Model<'a>, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        Ok(Model { gl: gl.clone(), id: vao, material: material })
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

impl<'a> Drop for Model<'a> {
    fn drop(&mut self) {
        /*unsafe {
            //TODO:self.gl.DeleteProgram(self.id);
        }*/
    }
}
