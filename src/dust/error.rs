use loader;

#[derive(Debug)]
pub enum DustError {
    Load(loader::LoadError)
}

impl From<loader::LoadError> for DustError {
    fn from(other: loader::LoadError) -> Self {
        DustError::Load(other)
    }
}
