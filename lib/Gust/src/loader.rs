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

    let no_vertices = m.positions.len()/3;

    // Create mesh
    let mut mesh = match m.indices.len() > 0 {
        true => mesh::Mesh::create_indexed(m.indices.clone(), m.positions.clone())?,
        false => mesh::Mesh::create(m.positions.clone())?
    };

    if m.normals.len() > 0
    {
        mesh.add_custom_vec3_attribute("normal", m.normals.clone())?;
    }

    Ok(mesh)
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}