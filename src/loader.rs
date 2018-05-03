use std::path::PathBuf;
use std::{str, fs};
use std::io::{self, BufReader};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub fn load_string(resource_name: &str) -> Result<String, Error>
{
    let mut read_buffer = load_read_buffer(resource_name)?;
    let buffer = read_buffer.fill_buf()?;

    if buffer.iter().find(|i| **i == 0).is_some() {
        return Err(Error::FileContainsNil);
    }
    let temp = str::from_utf8(buffer).unwrap();
    Ok(temp.to_string())
}

pub fn load_read_buffer(resource_name: &str) -> Result<Box<io::BufRead>, Error>
{
    let root_path: PathBuf = PathBuf::from("");
    let file = fs::File::open(
        resource_name_to_path(&root_path,resource_name)
    )?;

    let buffer = BufReader::new(file);
    Ok(Box::new(buffer))
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
pub fn load(name: &str) -> Result<Box<io::BufRead>, Error>
{
    use emscripten::{emscripten};
    emscripten::wget_data(name);
    println!("Out name: {}", name);
    load_read_buffer(name)
}

#[cfg(not(target_os = "emscripten"))]
pub fn load(name: &str) -> Result<Box<io::BufRead>, Error>
{
    load_read_buffer(name)
}
