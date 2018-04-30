use std::path::PathBuf;
use std::{string, fs};
use std::io::{self, Read};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToConvertToString(string::FromUtf8Error)
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(other: string::FromUtf8Error) -> Self {
        Error::FailedToConvertToString(other)
    }
}

pub fn load_string(resource_name: &str) -> Result<String, Error>
{
    let buffer = load_buffer(resource_name)?;
    let str = String::from_utf8(buffer)?;
    Ok(str)
}

pub fn load_buffer(resource_name: &str) -> Result<Vec<u8>, Error>
{
    let root_path: PathBuf = PathBuf::from("");
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
    Ok(buffer)
}

use std::path::Path;

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}

#[cfg(target_os = "emscripten")]
pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(Vec<u8>)
{
    let on_l = |temp: String| {
        use loader;
        let data = load_buffer(temp.as_str()).unwrap();
        on_load(data);
    };
    let on_error = |cause: String| {
        panic!(cause);
    };
    use emscripten::{emscripten};
    emscripten::async_wget_data(name, on_l, on_error);
}

#[cfg(not(target_os = "emscripten"))]
pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(Vec<u8>)
{
    use loader;
    let data = load_buffer(name).unwrap();
    on_load(data);
}
