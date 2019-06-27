pub mod mesh;

pub use crate::geometries::mesh::*;

use crate::*;


#[derive(Debug)]
pub enum Error {
    Buffer(buffer::Error)
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}
