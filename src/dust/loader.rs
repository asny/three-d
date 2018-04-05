use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToConvertToString
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}


pub fn load_string(resource_name: &str) -> Result<String, Error>
{
    let root_path: PathBuf = PathBuf::from("./");
    let mut file = fs::File::open(
        resource_name_to_path(&root_path,resource_name)
    )?;

    // allocate buffer of the same size as file
    let mut buffer: Vec<u8> = Vec::with_capacity(
        file.metadata()?.len() as usize + 1
    );
    file.read_to_end(&mut buffer)?;

    // check for nul byte
    if buffer.iter().find(|i| **i == 0).is_some() {
        return Err(Error::FileContainsNil);
    }

    let str = String::from_utf8(buffer).map_err(|_| Error::FailedToConvertToString)?;
    Ok(str)
}

use std::path::Path;

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
