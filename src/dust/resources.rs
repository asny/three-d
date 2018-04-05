use std::path::PathBuf;
use std::fs;
use std::string;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
    FailedToConvertToString
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_exe_path() -> Result<Resources, Error>
    {
        /*let dir = PathBuf::from(
            ::std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| Error::FailedToGetExePath)?);*/
        Ok(Resources {
            root_path: PathBuf::from("./") //dir.into()
        })
    }

    pub fn load_string(&self, resource_name: &str) -> Result<String, Error>
    {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
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
}

use std::path::Path;

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
