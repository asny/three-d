pub mod mesh_factory;
pub mod mesh_loader;

#[derive(Debug)]
pub enum Error {
    Buffer(core::buffer::Error)
}

impl From<core::buffer::Error> for Error {
    fn from(other: core::buffer::Error) -> Self {
        Error::Buffer(other)
    }
}
