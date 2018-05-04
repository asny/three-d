use std::path::Path;
use std::path::PathBuf;
use tobj;
use mesh;

#[derive(Debug)]
pub enum Error {
    ObjLoader(tobj::LoadError),
    Mesh(mesh::Error),
    FileDoesntContainModel{message: String}
}

impl From<tobj::LoadError> for Error {
    fn from(other: tobj::LoadError) -> Self {
        Error::ObjLoader(other)
    }
}

impl From<mesh::Error> for Error {
    fn from(other: mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub fn load_obj(name: &str) -> Result<mesh::Mesh, Error>
{
    let root_path: PathBuf = PathBuf::from("");
    let (models, _materials) = tobj::load_obj(&resource_name_to_path(&root_path,name))?;
    let m = &models.first().ok_or(Error::FileDoesntContainModel {message: format!("The file {} doesn't contain a model", name)})?.mesh;
    Ok(mesh::Mesh::create_unsafe(&m.indices.clone(), &m.positions.clone())?)
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}