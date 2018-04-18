use dust::program;
use gl;
use dust::input;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    Program(program::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

pub trait Material {
    fn apply(&self);
    fn setup_states(&self) -> Result<(), Error>;
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error>;
    fn get_attribute_location(&self, name: &str) -> Result<i32, Error>;
}

pub struct TriangleMaterial {
    program: program::Program
}

impl Material for TriangleMaterial
{
    fn apply(&self)
    {
        self.program.set_used();
    }

    fn setup_states(&self) -> Result<(), Error> {
        Ok(())
    }

    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error> {
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }

    fn get_attribute_location(&self, name: &str) -> Result<i32, Error> {
        let location = self.program.get_attribute_location(name)?;
        Ok(location)
    }
}

impl TriangleMaterial
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<TriangleMaterial>, Error>
    {
        let shader_program = program::Program::from_resource(&gl, "assets/shaders/triangle")?;
        Ok(Rc::new(TriangleMaterial { program: shader_program }))
    }
}
