use dust::program;
use dust::input;

#[derive(Debug)]
pub enum Error {
    Program(program::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

pub trait Shade {
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct Material {
    program: program::Program
}

impl Shade for Material {
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error> {
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }
}

impl Material
{
    pub fn create(shader_program: &program::Program) -> Result<Material, Error>
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
