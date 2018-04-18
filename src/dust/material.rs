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

pub trait Material {
    fn apply(&self);
    fn setup_states(&self) -> Result<(), Error>;
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error>;
    fn get_attribute_location(&self, name: &str) -> Result<i32, Error>;
}