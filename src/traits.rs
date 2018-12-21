use crate::core::program;
use crate::core::surface;
use crate::core::texture;
use crate::*;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Model(surface::Error),
    Texture(texture::Error),
    Mesh(static_mesh::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<surface::Error> for Error {
    fn from(other: surface::Error) -> Self {
        Error::Model(other)
    }
}

impl From<texture::Error> for Error {
    fn from(other: texture::Error) -> Self {
        Error::Texture(other)
    }
}

impl From<static_mesh::Error> for Error {
    fn from(other: static_mesh::Error) -> Self {
        Error::Mesh(other)
    }
}