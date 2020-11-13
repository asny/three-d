
#[cfg(feature = "3d-io")]
pub mod threed;

#[cfg(feature = "3d-io")]
pub use threed::*;

#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    IO(std::io::Error)
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Bincode(err).into()
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err).into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_3d_file(mesh: &crate::CPUMesh, path: &str) -> Result<(), Error>
{
    let mut file = std::fs::File::create(path)?;
    use std::io::prelude::*;
    file.write_all(&ThreeD::serialize(mesh)?)?;
    Ok(())
}


