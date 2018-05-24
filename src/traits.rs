use input;
use gust::mesh;
use core::program;
use core::attributes;
use core::texture;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Model(attributes::Error),
    Texture(texture::Error),
    Mesh(mesh::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<attributes::Error> for Error {
    fn from(other: attributes::Error) -> Self {
        Error::Model(other)
    }
}

impl From<texture::Error> for Error {
    fn from(other: texture::Error) -> Self {
        Error::Texture(other)
    }
}

impl From<mesh::Error> for Error {
    fn from(other: mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub trait Emitting {
}

pub trait Reflecting {
    fn reflect(&self, input: &input::ReflectingInput) -> Result<(), Error>;
}