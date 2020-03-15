pub mod vertices;
pub mod edges;
pub mod mesh;
pub mod cpu_mesh;
pub mod skybox;
pub mod imposter;

pub use crate::objects::vertices::*;
pub use crate::objects::edges::*;
pub use crate::objects::mesh::*;
pub use crate::objects::cpu_mesh::*;
pub use crate::objects::skybox::*;
pub use crate::objects::imposter::*;

#[derive(Debug)]
pub enum Error {
    Core(crate::core::Error),
    #[cfg(feature = "3d-io")]
    Bincode(bincode::Error),
    #[cfg(feature = "3d-io")]
    IO(std::io::Error)
}

#[cfg(feature = "3d-io")]
impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Bincode(err).into()
    }
}

#[cfg(feature = "3d-io")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err).into()
    }
}

impl From<crate::core::Error> for Error {
    fn from(other: crate::core::Error) -> Self {
        Error::Core(other)
    }
}