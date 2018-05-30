use gust::mesh;
use core::program;
use core::surface;
use core::texture;
use glm;

pub struct ReflectingInput {
    pub model: glm::Matrix4<f32>,
    pub view: glm::Matrix4<f32>,
    pub projection: glm::Matrix4<f32>,
    pub normal: glm::Matrix4<f32>,
    pub camera_position: glm::Vec3
}

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Model(surface::Error),
    Texture(texture::Error),
    Mesh(mesh::Error)
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

impl From<mesh::Error> for Error {
    fn from(other: mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub trait Emitting {
}

pub trait Reflecting {
    fn reflect(&self, input: &ReflectingInput) -> Result<(), Error>;
}