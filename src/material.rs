use program;
use input;
use gust::mesh;
use gl;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Mesh(mesh::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<mesh::Error> for Error {
    fn from(other: mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub trait Reflecting {
    fn apply(&self);
    fn setup_states(&self, gl: &gl::Gl) -> Result<(), Error>;
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error>;
    fn setup_attributes(&self, mesh: &mesh::Mesh) -> Result<(), Error>;
}