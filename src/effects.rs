pub mod debug_effect;

pub use crate::effects::debug_effect::*;

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