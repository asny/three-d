pub mod debug;
pub mod fog;
pub mod copy;

pub use crate::effects::debug::*;
pub use crate::effects::fog::*;
pub use crate::effects::copy::*;

use crate::*;
#[derive(Debug)]
pub enum Error {
    Program(program::Error),
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}