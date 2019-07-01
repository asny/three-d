pub mod mesh;
pub mod simple;
pub mod full_screen_quad;

pub use crate::geometries::mesh::*;
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
