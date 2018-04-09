use gl;
use program;

#[derive(Debug)]
pub enum Error {
}

#[derive(Clone)]
pub struct Material {
    program: program::Program
}


impl Material
{
    pub fn create(gl: &gl::Gl, shader_program: &program::Program) -> Result<Material, Error>
    {
        Ok(Material { program: shader_program.clone() })
    }

    pub fn program(&self) -> &program::Program {
        &self.program
    }

    pub fn apply(&self)
    {
        self.program.set_used();
    }
}
