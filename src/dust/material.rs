use gl;
use program;

#[derive(Debug)]
pub enum Error {
}

pub struct Material {
    program: program::Program
}


impl Material
{
    pub fn create(gl: &gl::Gl) -> Result<Material, Error>
    {
        // set up shader program
        let shader_program = program::Program::from_resource(
            gl, "assets/shaders/triangle"
            ).unwrap();

        Ok(Material { program: shader_program })
    }

    pub fn program(&self) -> &program::Program {
        &self.program
    }

    pub fn apply(&self)
    {
        self.program.set_used();
    }
}

impl Drop for Material {
    fn drop(&mut self) {
    }
}
