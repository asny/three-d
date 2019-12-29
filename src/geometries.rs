pub mod mesh_factory;
pub mod full_screen_quad;
pub mod mesh_loader;

pub use crate::geometries::full_screen_quad::*;

#[derive(Debug)]
pub enum Error {
    Buffer(core::buffer::Error)
}

impl From<core::buffer::Error> for Error {
    fn from(other: core::buffer::Error) -> Self {
        Error::Buffer(other)
    }
}
